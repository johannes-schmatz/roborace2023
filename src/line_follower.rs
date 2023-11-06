use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use crate::pid::Pid;
use crate::robot::Robot;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct LineFollower {
	pid: Pid,
	center: f64,

	speed_pid: Pid,
	speed: f64,
}

impl Default for LineFollower {
	fn default() -> Self {
		LineFollower {
			pid: Pid::new(-1.0, 0.0, 0.0),
			center: 50.0,

			speed_pid: Pid::new(1.0, 0.0, 0.0),
			speed: 50.0,
		}
	}
}

impl LineFollower {
	#[allow(unused_variables)]
	pub(crate) fn measure(&self, bot: &Robot) -> Result<()> {
		todo!()
	}

	pub(crate) fn prepare_drive(&mut self, bot: &Robot) -> Result<()> {
		bot.distance.set_mode_us_dist_cm().context("Failed to set distance mode")?;
		//bot.gyro.set_mode_gyro_ang().context("Failed to set gyro mode")?;
		bot.color.set_mode_col_reflect().context("Failed to set color mode")?;

		let last = bot.color.get_color()? as f64 - self.center;
		self.pid.set_last(last);
		println!("set last to {last:?}");

		self.speed_pid.set_last(0.0); // TODO: is this good?

		bot.left.start()?;
		bot.right.start()?;

		bot.left.set_speed(self.speed).context("Failed to set duty cycle left")?;
		bot.right.set_speed(self.speed).context("Failed to set duty cycle right")?;

		bot.top_arm.run_forever()?;

		Ok(())
	}

	pub(crate) fn drive(&mut self, bot: &Robot) -> Result<()> {
		let distance = bot.distance.get_distance_centimeters()?;
		let distance = if distance == 255.0 { None } else { Some(distance as f64) };

		if let Some(distance) = distance {
			// when run with 10.0 had a distance of about 8 cm
			if false && distance < 25.0 { // TODO: disabled for now
				bot.left.stop()?;
				bot.right.stop()?;
			}

			if distance < 8.0 { // TODO: remove, this is for testing
				bot.left.stop()?;
				bot.right.stop()?;
				panic!();
			}

			let distance = self.speed_pid.update(distance);

			// TODO: impl speed distance keeping

			print!("{distance:?}\t\t\t");
		}

		let reflection = bot.color.get_color()? as f64;

		let delta_speed = self.pid.update(reflection - self.center);

		println!("{reflection:?} -> {:?} {:?}", self.speed + delta_speed, self.speed - delta_speed);

		bot.left.set_speed(self.speed + delta_speed)?;
		bot.right.set_speed(self.speed - delta_speed)?;

		Ok(())
	}

	pub(crate) fn end_drive(&self, bot: &Robot) -> Result<()> {
		bot.left.stop()?;
		bot.right.stop()?;

		bot.top_arm.stop()?;

		Ok(())
	}
}