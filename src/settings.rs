use anyhow::{Context, Result};
use std::path::Path;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::menu;
use crate::pid::Pid;
use crate::robot::Robot;
use crate::robot::state::RobotState;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Settings {
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


	pub(crate) fn measure(&self, bot: &Robot) -> Result<()> {
		// TODO: measure
		// We want to drive over the line to figure out if min/max value are actually fine
		// and not only print them (all of the values), but also use them to find the perfect middle
		// to "ride" on. We should need to actively accept the values, so they don't end up in the
		// config file by accident.

		Ok(())
	}

	pub(crate) fn prepare_drive(&mut self, bot: &Robot) -> Result<()> {
		bot.distance.set_mode_us_dist_cm().context("Failed to set distance mode")?;
		//bot.gyro.set_mode_gyro_ang().context("Failed to set gyro mode")?;
		bot.color.set_mode_col_reflect().context("Failed to set color mode")?;

		let last = bot.color.get_color()? as f64 - self.line_center;
		self.line_pid.set_last(last);
		println!("set last to {last:?}");

		self.distance_pid.set_last(0.0); // TODO: is this good?

		bot.left.start()?;
		bot.right.start()?;

		bot.left.set_speed(self.speed).context("Failed to set duty cycle left")?;
		bot.right.set_speed(self.speed).context("Failed to set duty cycle right")?;

		bot.top_arm.run_forever()?;

		Ok(())
	}

	/// Return `Ok(true)` to stop the robot
	pub(crate) fn drive(&mut self, bot: &Robot) -> Result<bool> {
		let distance = bot.distance.get_distance_centimeters()?;
		let distance = if distance == 255.0 { None } else { Some(distance as f64) };

		if let Some(distance) = distance {
			// when run with 10.0 had a distance of about 8 cm
			if false && distance < 25.0 { // TODO: disabled for now
				bot.left.stop()?;
				bot.right.stop()?;

				return Ok(true);
			}

			if false && distance < 8.0 { // TODO: remove, this is for testing
				bot.left.stop()?;
				bot.right.stop()?;

				return Ok(true);
			}
		}

		// delta for both
		let delta_speed_both = if let Some(distance) = distance {
			let delta_speed = self.distance_pid.update(distance - self.distance_center);

			print!("{distance:>5.1} => {delta_speed:>5.1} -- ");

			delta_speed
		} else {
			print!("      =>       -- ");

			0.0
		};

		let delta_speed_both = 0.0;

		let reflection = bot.color.get_color()? as f64;

		// 2 * this = delta between left and right
		let delta_speed = self.line_pid.update(reflection - self.line_center);

		if false {
			println!("{reflection:>5.1} -> l: {:>5.1} r: {:>5.1}", self.speed + delta_speed, self.speed - delta_speed);
		}

		let delta_speed = 0.0;
		println!();

		bot.left .set_speed(self.speed + delta_speed_both + delta_speed)?;
		bot.right.set_speed(self.speed + delta_speed_both - delta_speed)?;

		Ok(false)
	}

	pub(crate) fn end_drive(&self, bot: &Robot) -> Result<()> {
		bot.left.stop()?;
		bot.right.stop()?;

		bot.top_arm.stop()?;

		Ok(())
	}



	fn path() -> &'static Path {
		Path::new("./robot_settings.toml")
	}

	pub(crate) fn get() -> Result<Settings> {
		let path = Self::path();

		if path.exists() {
			let string = std::fs::read_to_string(path)
				.context("Failed to read settings file")?;
			toml::from_str(&string)
				.context("Failed to parse settings")
		} else {
			println!("No settings file found, writing new settings file to {path:?}");

			let settings = Settings::default();

			let string = toml::to_string_pretty(&settings)
				.context("Failed to serialize the settings")?;
			std::fs::write(path, string)
				.context("Failed to write settings file")?;

			Ok(settings)
		}
	}

	pub(crate) fn write(self) -> Result<()> {
		let path = Self::path();

		let out = toml::to_string_pretty(&self)
			.context("Failed to serialize the settings")?;
		let read = std::fs::read_to_string(path).unwrap_or_else(|_| String::new());

		if out != read {
			println!("Updating settings file {path:?}");
			std::fs::write(path, out)
				.context("Failed to write settings file")?;
		}

		Ok(())
	}

	/// Return `Ok(true)` to end the program, `Ok(false)` otherwise.
	pub(crate) fn tick(&mut self, bot: &Robot) -> Result<bool> {
		bot.buttons.update();
		if bot.buttons.is_left() {
			std::thread::sleep(Duration::from_millis(300));
			self.next_state(bot, RobotState::InMenu)?;
		}

		match self.state {
			RobotState::Exit => {
				// try to stop the motors
				let _ = bot.left.stop();
				let _ = bot.right.stop();
				let _ = bot.top_arm.stop();

				return Ok(true)
			},
			RobotState::InMenu => {
				if let Some(new_state) = menu::select(bot)? {
					self.next_state(bot, new_state)?;
				}
			},
			RobotState::Test => {
				bot.test()?;
				self.next_state(bot, RobotState::Exit)?;
			},

			RobotState::LineMeasure => {
				self.measure(bot)?;
				self.next_state(bot, RobotState::InMenu)?;
			},
			RobotState::LineDrive => {
				if self.drive(bot)? {
					self.next_state(bot, RobotState::Exit)?;
				}
			},
		}

		Ok(false)
	}

	pub(crate) fn next_state(&mut self, bot: &Robot, new_state: RobotState) -> Result<()> {
		match self.state {
			RobotState::LineDrive => {
				self.end_drive(bot)
					.context("Failed to end line drive")?;
			},
			_ => {},
		}

		match (&self.state, &new_state) {
			(_, RobotState::LineMeasure) => {},
			(_, RobotState::LineDrive) => {
				self.prepare_drive(bot)
					.context("Failed to prepare for line drive")?;
			},
			(_, _) => {},
		}

		self.state = new_state;
		Ok(())
	}
}