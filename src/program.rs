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

	line_pid: Pid,
	line_center: f64,

	distance_pid: Pid,
	distance_center: f64,
	distance_trigger: f64,

	stop_distance: f64,

	speed: f64,

	#[serde(skip)]
	state: RobotState,
}

impl Default for Program {
	fn default() -> Self {
		Self {
			log: true,

			robot_wheel_width: 14.0, // obtained by measurement
			diameter: 100.0,

			rotate_arm: true,
			rotate_arm_speed: 100.0,

			line_pid: Pid::new(-0.4, 0.0, 0.5),
			line_center: 50.0,

			distance_pid: Pid::new(1.0, 0.0, 0.0),
			distance_center: 20.0,
			distance_trigger: 40.0,

			stop_distance: 20.0,

			speed: 50.0,

			state: RobotState::default(),
		}
	}
}

impl Program {
	fn test(&self, bot: &Robot) -> Result<()> {
		dbg!(&bot.left);
		dbg!(&bot.right);

		bot.top_arm.start()?;
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

			let color = bot.color.get_color()?;
			println!("{:?}", color);

			std::thread::sleep(Duration::from_millis(500));
		}

		Ok(())
	}

	fn prepare_drive(&mut self, bot: &Robot) -> Result<()> {
		let last = bot.color.get_color()? - self.line_center;
		self.line_pid.set_last(last);
		println!("set last to {last:?}");

		self.distance_pid.set_last(self.distance_trigger);

		bot.left.start()?;
		bot.left.set_speed(self.speed)?;

		bot.right.start()?;
		bot.right.set_speed(self.speed)?;

		Ok(())
	}

	fn drive(&mut self, bot: &Robot) -> Result<()> {
		if self.log {
			match self.state {
				RobotState::DriveSimpleOnly => print!("si "),
				RobotState::DriveEntry => print!("in "),
				RobotState::DriveFollow => print!("fo "),
				RobotState::DriveExit => print!("ex "),
				_ => {},
			}
		}

		let distance = bot.distance.get_distance()?;

		if let Some(distance) = distance {
			// when run with 10.0 had a distance of about 8 cm
			if distance < self.stop_distance && self.state == RobotState::DriveExit {
				bot.left.stop()?;
				bot.right.stop()?;

				println!("stopping because dst was: {distance:?}, which is less than {:?}", self.stop_distance);

				return self.next_state(bot, RobotState::Exit);
			}

			if distance < self.distance_center && self.state == RobotState::DriveEntry {
				self.state = RobotState::DriveFollow;

				if self.rotate_arm {
					bot.top_arm.start()?;

					// force the arm to start up
					bot.top_arm.set_speed(50.0)?;
					//std::thread::sleep(Duration::from_millis(100));
					//bot.top_arm.set_speed(self.rotate_arm_speed)?;
				}
			}

			if distance > self.distance_trigger && self.state == RobotState::DriveFollow {
				self.state = RobotState::DriveExit;
				bot.top_arm.stop()?;
			}
		}

		// -0.5 ..= 0.5, default 0
		let speed_correction = if let Some(distance) = distance {
			if self.state == RobotState::DriveFollow {
				// we are actually following the wall

				if distance < self.distance_trigger {
					let speed_correction = self.distance_pid.update(distance - self.distance_center) / 100.0;

					if self.log {
						print!("{distance:>5.1} => {speed_correction:>5.1} -- ");
					}

					speed_correction
				} else {
					if self.log {
						print!("{distance:>5.1} =>no trig-- ");
					}
					0.0
				}
			} else {
				if self.log {
					print!("{distance:>5.1} =>wrong st- ");
				}
				0.0
			}
		} else {
			if self.log {
				print!("no useful dist -- ");
			}
			0.0
		};

		let reflection = bot.color.get_color()?;

		// 2 * this = delta between left and right
		let line_correction = self.line_pid.update(reflection - self.line_center) / 1000.0;

		let line_after_correction = if self.state == RobotState::DriveFollow {
			self.robot_wheel_width / self.diameter
		} else {
			0.0
		};

		let l = self.speed * (1.0 + speed_correction) * (1.0 + line_correction + line_after_correction);
		let r = self.speed * (1.0 + speed_correction) * (1.0 - line_correction - line_after_correction);

		if self.log {
			print!("ref: {reflection:>5.1} -> l: {l:>5.1} r: {r:>5.1}");

			if reflection < 17.0 {
				print!(" low ref!");
			}

			println!();
		}

		bot.left.set_speed(l)?;
		bot.right.set_speed(r)?;

		Ok(())
	}

	fn end_drive(&self, bot: &Robot) -> Result<()> {
		bot.left.stop()?;
		bot.right.stop()?;
		bot.top_arm.stop()?;

		Ok(())
	}

	fn tick(&mut self, bot: &Robot) -> Result<bool> {
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
				self.drive(bot)?;
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
				self.end_drive(bot)
					.context("Failed to end line drive")?;
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
		let mut n = 0;
		loop {
			let start = Instant::now();

			if self.tick(bot).context("Failed to tick robot")? {
				break;
			}

			let end = start.elapsed();

			if n == 0 && self.log {
				println!("tick took: {:?}", end);
			}
			n += 1;
			n %= 20;

			if let Some(dur) = tick_time.checked_sub(end) {
				std::thread::sleep(dur)
			}
		}

		Ok(())
	}
}