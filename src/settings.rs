use anyhow::{Context, Result};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use crate::menu;
use crate::pid::Pid;
use crate::robot::Robot;
use crate::state::RobotState;

/*
how to qualification:
- run ./run.sh measure
- diameter circle!
- PID circle driving!
- set distance center!

also check:
rotate_arm = false
 */

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Settings {
	robot_wheel_width: f64,
	diameter: f64,

	rotate_arm: bool,
	rotate_arm_speed: f64, // 0..=100

	line_pid: Pid,
	line_center: f64,

	distance_pid: Pid,
	distance_center: f64,
	speed: f64,

	#[serde(skip)]
	state: RobotState,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			robot_wheel_width: 14.0, // obtained by measurement
			diameter: 100.0,

			rotate_arm: true,
			rotate_arm_speed: 100.0,

			line_pid: Pid::new(-0.4, 0.0, 0.5),
			line_center: 50.0,

			distance_pid: Pid::new(1.0, 0.0, 0.0),
			distance_center: 20.0,

			speed: 50.0,

			state: RobotState::default(),
		}
	}
}

impl Settings {
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

		//TODO: We want to drive over the line to figure out if min/max value are actually fine
		// and not only print them (all of the values), but also use them to find the perfect middle
		// to "ride" on. We should need to actively accept the values, so they don't end up in the
		// config file by accident.

		Ok(())
	}

	fn prepare_drive(&mut self, bot: &Robot) -> Result<()> {
		let last = bot.color.get_color()? - self.line_center;
		self.line_pid.set_last(last);
		println!("set last to {last:?}");

		self.distance_pid.set_last(2.0 * self.distance_center);

		bot.left.start()?;
		bot.left.set_speed(self.speed)?;

		bot.right.start()?;
		bot.right.set_speed(self.speed)?;

		if self.rotate_arm {
			bot.top_arm.start()?;

			// force the arm to start up
			bot.top_arm.set_speed(50.0)?;
			std::thread::sleep(Duration::from_millis(100));
			bot.top_arm.set_speed(self.rotate_arm_speed)?;
		}

		Ok(())
	}

	fn drive(&mut self, bot: &Robot) -> Result<()> {
		self.state = RobotState::Driving;

		let distance = bot.distance.get_distance()?;

		if let Some(distance) = distance {
			// when run with 10.0 had a distance of about 8 cm
			if false && distance < 25.0 { // TODO: disabled for now, as qualification doesn't need it
				bot.left.stop()?;
				bot.right.stop()?;

				return self.next_state(bot, RobotState::Exit);
			}

			if false && distance < 30.0 && self.state == RobotState::ApproachingWall {
				self.state = RobotState::Driving;
				println!("Switched to following wall");
			}
		}

		// 0.5 ..= 1, default 1
		let speed_correction = if let Some(distance) = distance {
			// distance from 0 to 255
			if true || self.state == RobotState::Driving {
				// we are actually following the wall

				if distance < 2.0 * self.distance_center {
					let speed_correction = self.distance_pid.update(distance - self.distance_center);

					print!("{distance:>5.1} => {speed_correction:>5.1} -- ");

					1.0 + speed_correction
					// + 1 to make it adjust on top of the base speed and not stop the robot
					// when the error here is getting to 0
				} else {
					1.0
				}
			} else {
				print!("follow wall => -- ");
				1.0
			}
		} else {
			print!("no useful dist -- ");
			1.0
		};

		let reflection = bot.color.get_color()?;

		// 2 * this = delta between left and right
		let line_correction = self.line_pid.update(reflection - self.line_center) / 1000.0;

		let line_after_correction = self.robot_wheel_width / self.diameter;
		// if self.diameter == 0 then set this to 0!! (allows us to declare no diameter at all
		// or use Option<f64>

		// TODO: cmd line option for running left/right a tiny bit

		let l = self.speed * speed_correction * (1.0 + line_correction + line_after_correction);
		let r = self.speed * speed_correction * (1.0 - line_correction - line_after_correction);

		println!("ref: {reflection:>5.1} -> l: {l:>5.1} r: {r:>5.1}");

		if reflection < 17.0 {
			print!(" low ref!");
		}

		bot.left .set_speed(l)?;
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
				self.next_state(bot, RobotState::Exit)?;
			},

			RobotState::Measure => {
				self.measure(bot)?;
				self.next_state(bot, RobotState::InMenu)?;
			},
			RobotState::ApproachingWall | RobotState::Driving => {
				self.drive(bot)?;
			},
		}

		Ok(false)
	}

	fn next_state(&mut self, bot: &Robot, new_state: RobotState) -> Result<()> {
		match self.state {
			RobotState::Driving => {
				self.end_drive(bot)
					.context("Failed to end line drive")?;
			},
			_ => {},
		}

		match &new_state {
			RobotState::ApproachingWall => {
				self.prepare_drive(bot)
					.context("Failed to prepare for line drive")?;
			},
			_ => {},
		}

		self.state = new_state;
		Ok(())
	}


	pub(crate) fn main(&mut self, bot: &Robot) -> Result<()> {
		let initial_state = RobotState::get_initial().context("Failed to parse command line arguments")?;
		self.next_state(&bot, initial_state)?;

		// we do 100 ticks per second
		let tick_time = Duration::from_millis(10);
		let mut n = 0;
		loop {
			let start = Instant::now();

			if self.tick(&bot).context("Failed to tick robot")? {
				break;
			}

			let end = start.elapsed();

			if n == 0 {
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