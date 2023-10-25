use std::time::Duration;
use anyhow::{Context, Result};
use ev3dev_lang_rust::Button as Buttons;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, SensorPort};
use crate::menu::{Menu, MenuItem};
use crate::motor::Motor;
use crate::settings::Settings;

#[derive(Debug)]
pub struct Robot {
	pub(crate) buttons: Buttons,

	pub(crate) menu: Menu,

	pub color: ColorSensor,
	//pub gyro: GyroSensor,
	pub left: Motor,
	pub right: Motor,
}

impl Robot {
	pub fn new() -> Result<Robot> {
		let buttons = Buttons::new()
			.context("Failed to get the robot buttons")?;

		Ok(Robot {
			buttons: buttons.clone(),

			menu: Menu::new(buttons, vec![
				MenuItem::new("Line Measure", RobotState::LineMeasure),
				MenuItem::new("Line Start", RobotState::LineDrive),
				MenuItem::new("Gradient Measure", RobotState::GradientMeasure),
				MenuItem::new("Gradient Start", RobotState::GradientDrive),
			]),

			color: ColorSensor::get(SensorPort::In1)
				.context("Failed to get the color sensor")?,
			//gyro: GyroSensor::get(SensorPort::In2)
			//	.context("Failed to get the gyro sensor")?,

			left: {
				let left = LargeMotor::get(MotorPort::OutA)
					.context("Failed to get the left motor")?;
				left.set_polarity(LargeMotor::POLARITY_INVERSED)?;
				Motor::new(left, "left")
			},
			right: {
				let right = LargeMotor::get(MotorPort::OutB)
					.context("Failed to get the right motor")?;
				right.set_polarity(LargeMotor::POLARITY_INVERSED)?;
				Motor::new(right, "right")
			},
		})
	}

	pub fn test(&self) -> Result<()> {
		self.left.inner.set_stop_action(LargeMotor::STOP_ACTION_BRAKE).context("hold")?;
		self.right.inner.set_stop_action(LargeMotor::STOP_ACTION_BRAKE).context("hold")?;

		self.left.inner.set_speed_sp(1000).context("speed sp")?;
		self.right.inner.set_speed_sp(1000).context("speed sp")?;

		self.left.set_speed(100f64)?;
		self.right.set_speed(100f64)?;

		self.left.start()?;
		self.right.start()?;

		std::thread::sleep(Duration::from_secs(3));

		self.left.set_speed(-100f64)?;
		self.right.set_speed(-100f64)?;

		std::thread::sleep(Duration::from_secs(4));

		self.left.set_speed(100f64)?;
		self.right.set_speed(100f64)?;

		std::thread::sleep(Duration::from_secs(1));

		self.left.stop()?;
		self.right.stop()?;

		Ok(())
	}
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum RobotState {
	Exit,

	#[default]
	InMenu,
	Test,

	LineMeasure,
	LineDrive,

	GradientMeasure,
	GradientDrive,
}
