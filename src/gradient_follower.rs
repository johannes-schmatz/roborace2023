use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use crate::robot::button::Button;
use crate::pid::Pid;
use crate::robot::Robot;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct GradientFollower {
	pid: Pid,
	center: f64,
	speed: f64,
}

impl Default for GradientFollower {
	fn default() -> Self {
		GradientFollower {
			pid: Pid::new(0.1, 0.0, 0.1),
			center: 0.0,
			speed: 50.0,
		}
	}
}

impl GradientFollower {
	pub(crate) fn measure(&self, bot: &Robot) -> Result<()> {
		let line_width = 37.5; // cm

		loop {
			let value = bot.color.get_color().context("while reading color")?;
			println!("println test");
			eprintln!("{value}");

			if let Button::Left = bot.buttons.await_press() {
				println!("break!!!");
				break;
			}
		}

		Ok(())
	}

	pub(crate) fn prepare_drive(&mut self, bot: &Robot) -> Result<()> {
		//bot.gyro.set_mode_gyro_ang().context("Failed to set gyro mode")?;
		bot.color.set_mode_col_reflect().context("Failed to set color mode")?;

		self.pid.set_last(0.0); // TODO: measure this??

		bot.left.start()?;
		bot.right.start()?;

		bot.left.set_speed(self.speed).context("Failed to set duty cycle left")?;
		bot.right.set_speed(self.speed).context("Failed to set duty cycle right")?;

		Ok(())
	}

	/// Regelstrecke: Wo ist der Roboter, links-rechts
	/// Regelgröße: Wie grau ist der Boden: y(t)
	/// Führungsgröße: Die Grauheit in der Mitte: w(t), konstant -> Sollwert
	/// Eingangssignal e(t) = w(t) - y(t)
	/// Ausgangssignal u(t)
	pub(crate) fn drive(&mut self, bot: &Robot) -> Result<()> {
		// w(t)
		let reflection = bot.color.get_color()? as f64;

		// e(t) = w(t) - y(t)
		let error = self.center - reflection;

		// u(t)
		let correction = self.pid.update(error);

		bot.left.set_speed(self.speed + correction)?;
		bot.right.set_speed(self.speed - correction)?;

		Ok(())
	}
}
