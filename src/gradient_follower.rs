use anyhow::{Context, Result};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor};
use crate::menu::Button;
use crate::motor::Motor;
use crate::pid::Pid;

#[derive(Debug)]
pub struct GradientFollower {
	color: ColorSensor,
//	gyro: GyroSensor,

	left: Motor,
	right: Motor,

	center: f64,

	pid: Pid,

	speed: f64,
}

impl GradientFollower {
	pub fn new(settings: &Settings, color: ColorSensor,/* gyro: GyroSensor,*/ left: Motor, right: Motor) -> GradientFollower {
		GradientFollower {
			color,/* gyro,*/ left, right,

			center: 0.0,

			pid: settings.gradient_pid.clone(),

			speed: 50.0,
		}
	}

	pub fn measure(&mut self) -> Result<()> {
		let line_width = 37.5;

		let btns = ev3dev_lang_rust::Button::new()?;
		loop {
			let value = self.color.get_color().context("while reading color")?;
			println!("println test");
			eprintln!("{value}");

			if let Button::Left = Button::await_press(&btns) {
				println!("break!!!");
				break;
			}
		}

		Ok(())
	}

	pub fn prepare_drive(&mut self) -> Result<()> {
		//self.gyro.set_mode_gyro_ang().context("Failed to set gyro mode")?;
		self.color.set_mode_col_reflect().context("Failed to set color mode")?;

		self.pid.set_last(0.0); // TODO: measure this??

		self.left.start()?;
		self.right.start()?;

		self.left.set_speed(self.speed as i32).context("Failed to set duty cycle left")?;
		self.right.set_speed(self.speed as i32).context("Failed to set duty cycle right")?;

		Ok(())
	}

	/// Regelstrecke: Wo ist der Roboter, links-rechts
	/// Regelgröße: Wie grau ist der Boden: y(t)
	/// Führungsgröße: Die Grauheit in der Mitte: w(t), konstant -> Sollwert
	/// Eingangssignal e(t) = w(t) - y(t)
	/// Ausgangssignal u(t)
	pub fn drive(&mut self) -> Result<()> {
		// w(t)
		let reflection = self.color.get_color()? as f64;

		// e(t) = w(t) - y(t)
		let error = self.center - reflection;

		// u(t)
		let correction = self.pid.update(error);

		self.left.set_speed(self.speed as i32 + correction as i32)?;
		self.right.set_speed(self.speed as i32 - correction as i32)?;

		Ok(())
	}
}
