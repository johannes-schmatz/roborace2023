use anyhow::{bail, Context, Result};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use crate::menu;
use crate::pid::Pid;
use crate::robot::Robot;
use crate::state::RobotState;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Program {
	log: bool,

	robot_wheel_width: f64,
	diameter: f64,

	rotate_arm: bool,
	rotate_arm_speed: f64, // 0..=100

	line: Pid,
	low_ref_warn: f64,
	speed: f64,

	distance: Pid,
	distance_trigger: f64,
	stop_distance: f64,

	#[serde(skip)]
	state: RobotState,
	#[serde(skip)]
	top_motor_reduce_speed: Option<usize>,
}

impl Default for Program {
	fn default() -> Self {
		Self {
			log: true,

			robot_wheel_width: 14.0,
			diameter: 100.0,

			rotate_arm: true,
			rotate_arm_speed: 100.0,

			line: Pid {
				center: 50.0,
				k_p: -0.4,
				k_i: 0.0,
				k_d: 0.5,
				last_error: 0f64, integral: 0f64,
			},
			low_ref_warn: 17.0,
			speed: 50.0,

			distance: Pid {
				center: 20.0,
				k_p: 1.0,
				k_i: 0.0,
				k_d: 0.0,
				last_error: 0f64, integral: 0f64,
			},
			distance_trigger: 40.0,
			stop_distance: 20.0,

			state: RobotState::default(),
			top_motor_reduce_speed: None,
		}
	}
}

impl Program {
	fn test(&self, bot: &Robot) -> Result<()> {
		dbg!(&bot.left);
		dbg!(&bot.right);

		bot.top_arm.start_with_full_power()?;
		std::thread::sleep(Duration::from_millis(100));
		bot.top_arm.set_speed(self.rotate_arm_speed)?;

		std::thread::sleep(Duration::from_secs(4));

		bot.top_arm.stop()?;

		Ok(())
	}

	fn measure(&self, bot: &Robot) -> Result<()> {
		loop {
			bot.buttons.update();
			if bot.buttons.is_right() {
				break;
			}

			let reflection = bot.color.get_color()?;
			let distance = bot.distance.get_distance()?.unwrap_or(f64::NAN);
			println!("ref: {reflection:>5.1} -- dst: {distance:>5.1}");

			std::thread::sleep(Duration::from_millis(500));
		}

		Ok(())
	}

	fn prepare_drive(&mut self, bot: &Robot) -> Result<()> {
		self.line.last_error = bot.color.get_color()? - self.line.center;
		self.distance.last_error = 0.0;

		bot.left.start()?;
		bot.left.set_speed(self.speed)?;

		bot.right.start()?;
		bot.right.set_speed(self.speed)?;

		Ok(())
	}

	fn drive(&mut self, bot: &Robot, tick_counter: usize) -> Result<()> {
		let distance = bot.distance.get_distance()?;

		if let Some(distance) = distance {
			match self.state {
				RobotState::DriveExit if distance < self.stop_distance => {
					self.state = RobotState::Exit;
					println!("stopping because dst was: {distance:?}, which is less than {:?}", self.stop_distance);
				},
				RobotState::DriveFollow if distance > self.distance_trigger => {
					self.state = RobotState::DriveExit;
					bot.top_arm.stop()?;
				},
				RobotState::DriveEntry if distance < self.distance.center => {
					self.state = RobotState::DriveFollow;

					if self.rotate_arm {
						bot.top_arm.start_with_full_power()?;
						// in 100ms turn it off
						self.top_motor_reduce_speed = Some(tick_counter + 10); // 10 = 100ms
					}
				},
				_ => {},
			}
		}
		// use this to offset the top motor speed setting
		if self.top_motor_reduce_speed.is_some_and(|x| x < tick_counter) {
			self.top_motor_reduce_speed = None;
			bot.top_arm.set_speed(self.rotate_arm_speed)?;
		}

		// -0.5 ..= 0.5, default 0
		let speed_correction = distance
			.filter(|&x| x < self.distance_trigger && self.state == RobotState::DriveFollow)
			.map_or(0.0, |x| {
				self.distance.update(x) / 100.0
			});

		let reflection = bot.color.get_color()?;
		let line_correction = self.line.update(reflection) / 1000.0;

		let line_after_correction = if self.state == RobotState::DriveFollow {
			// TODO: we can even consider for the actual competition to not use this feature!
			self.robot_wheel_width / self.diameter
		} else {
			0.0
		};

		let l = self.speed * (1.0 + speed_correction) * (1.0 + line_correction + line_after_correction);
		let r = self.speed * (1.0 + speed_correction) * (1.0 - line_correction - line_after_correction);

		bot.left.set_speed(l)?;
		bot.right.set_speed(r)?;

		if self.log {
			match self.state {
				RobotState::DriveSimpleOnly => print!("si "),
				RobotState::DriveEntry      => print!("in "),
				RobotState::DriveFollow     => print!("fo "),
				RobotState::DriveExit       => print!("ex "),
				_                           => print!(" ? "),
			}
			match distance {
				Some(distance) => print!("{distance:>5.1} "),
				None => print!("no dst"),
			};
			if distance.is_some_and(|x| x < self.distance_trigger) {
				print!(" => trigger  -- ");
			} else {
				print!(" =>          -- ");
			}
			print!(" {speed_correction:>5.1} -- ref: {reflection:>5.1} -> l: {l:>5.1} r: {r:>5.1}");
			if reflection < self.low_ref_warn {
				print!(" low ref!");
			}
			println!();
		}

		Ok(())
	}

	fn tick(&mut self, bot: &Robot, tick_counter: usize) -> Result<bool> {
		bot.buttons.update();
		if bot.buttons.is_left() {
			std::thread::sleep(Duration::from_millis(300));
			self.next_state(bot, RobotState::InMenu)?;
		}

		match self.state {
			RobotState::Exit => {
				return Ok(true)
			},
			RobotState::InMenu => {
				if let Some(new_state) = menu::select(bot)? {
					self.next_state(bot, new_state)?;
				}
			},
			RobotState::Test => {
				self.test(bot)?;
				self.state = RobotState::Exit;
			},

			RobotState::Measure => {
				self.measure(bot)?;
				self.state = RobotState::InMenu;
			},
			RobotState::DriveSimpleOnly |
			RobotState::DriveEntry |
			RobotState::DriveFollow |
			RobotState::DriveExit => {
				self.drive(bot, tick_counter)?;
			},
		}

		Ok(false)
	}

	fn next_state(&mut self, bot: &Robot, new_state: RobotState) -> Result<()> {
		match self.state {
			RobotState::DriveSimpleOnly |
			RobotState::DriveEntry |
			RobotState::DriveFollow |
			RobotState::DriveExit => {
				bot.left.stop().context("Failed to end line drive")?;
				bot.right.stop().context("Failed to end line drive")?;
				bot.top_arm.stop().context("Failed to end line drive")?;
			},
			_ => {},
		}

		match &new_state {
			RobotState::DriveSimpleOnly |
			RobotState::DriveEntry |
			RobotState::DriveFollow |
			RobotState::DriveExit => {
				self.prepare_drive(bot)
					.context("Failed to prepare for line drive")?;
			},
			_ => {},
		}

		self.state = new_state;
		Ok(())
	}


	pub(crate) fn main(&mut self, bot: &Robot) -> Result<()> {
		let initial_state = if let Some(arg) = std::env::args().skip(1).next() {
			match arg.as_str() {
				"help" => {
					eprintln!("{}", RobotState::HELP_TEXT);

					return Ok(())
				},
				"exit" => RobotState::Exit,
				"menu" => RobotState::InMenu,
				"test" => RobotState::Test,
				"measure" => RobotState::Measure,
				"drive" => RobotState::DriveEntry,
				"driveS" => RobotState::DriveSimpleOnly,
				"l" => {
					let amount = std::env::args().skip(2)
						.next().map(|x| x.parse::<f64>()).context("You're missing an argument")?
						.context("Your second argument needs to be a floating point number")?;

					return bot.left.step(amount);
				},
				"r" => {
					let amount = std::env::args().skip(2)
						.next().map(|x| x.parse::<f64>()).context("You're missing an argument")?
						.context("Your second argument needs to be a floating point number")?;

					return bot.right.step(amount);
				},
				"print" => {
					println!("{bot:#?}");

					return Ok(());
				},
				_ => {
					eprintln!("{}", RobotState::HELP_TEXT);

					bail!("Failed to parse command line arguments: No sub-command {arg:?} known");
				},
			}
		} else {
			RobotState::InMenu
		};

		self.next_state(bot, initial_state)?;

		// we do 100 ticks per second
		let tick_time = Duration::from_millis(10);
		let mut counter = 0usize; // 31 bit are sufficient for 99h of incrementing this, this should not fail
		loop {
			let start = Instant::now();

			if self.tick(bot, counter).context("Failed to tick robot")? {
				break;
			}

			let end = start.elapsed();

			if self.log && counter % 100 == 0 {
				println!("tick took: {:?}", end);
			}
			counter += 1;

			if let Some(dur) = tick_time.checked_sub(end) {
				std::thread::sleep(dur)
			}
		}

		Ok(())
	}
}