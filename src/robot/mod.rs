use std::time::Duration;
use anyhow::{Context, Result};
use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort, TouchSensor, UltrasonicSensor};
use crate::robot::button::Buttons;
use crate::robot::motor::Motor;

pub(crate) mod state;
pub(crate) mod motor;
pub(crate) mod button;

#[derive(Debug)]
pub(crate) struct Robot {
	pub(crate) buttons: Buttons,

	pub(crate) color: ColorSensor,
	//pub(crate) gyro: GyroSensor,
	pub(crate) distance: UltrasonicSensor,
	pub(crate) touch: TouchSensor,

	pub(crate) left: Motor,
	pub(crate) right: Motor,

	pub(crate) top_arm: MediumMotor,
}

impl Robot {
	pub(crate) fn new() -> Result<Robot> {
		Ok(Robot {
			buttons: Buttons::new()
				.context("Failed to get the robot buttons")?,

			color: ColorSensor::get(SensorPort::In1)
				.context("Failed to get the color sensor")?,
			//gyro: GyroSensor::get(SensorPort::In4)
			//	.context("Failed to get the gyro sensor")?,
			distance: UltrasonicSensor::get(SensorPort::In3)
				.context("Failed to get the ultrasonic sensor")?,
			touch: TouchSensor::get(SensorPort::In2)
				.context("Failed to get the touch sensor")?,

			left: {
				let motor = LargeMotor::get(MotorPort::OutB)
					.context("Failed to get the left motor")?;
				//motor.set_polarity(LargeMotor::POLARITY_INVERSED)?;
				motor.set_stop_action(LargeMotor::STOP_ACTION_BRAKE)?;
				motor.set_speed_sp(motor.get_max_speed()?)?;
				Motor::new(motor, "left")
			},
			right: {
				let motor = LargeMotor::get(MotorPort::OutA)
					.context("Failed to get the right motor")?;
				//motor.set_polarity(LargeMotor::POLARITY_INVERSED)?;
				motor.set_stop_action(LargeMotor::STOP_ACTION_BRAKE)?;
				motor.set_speed_sp(motor.get_max_speed()?)?;
				Motor::new(motor, "right")
			},

			top_arm: {
				let motor = MediumMotor::get(MotorPort::OutC)
					.context("Failed to get the medium motor")?;
				motor.set_polarity(MediumMotor::POLARITY_INVERSED)?;
				motor.set_stop_action(MediumMotor::STOP_ACTION_COAST)?;
				motor.set_speed_sp(motor.get_max_speed()? / 4)?;
				motor
			},
		})
	}

	pub(crate) fn test(&self) -> Result<()> {
		dbg!(&self.left);
		dbg!(&self.right);

		/*
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

		std::thread::sleep(Duration::from_secs(3));

		 */

			self.top_arm.run_forever()?;

			std::thread::sleep(Duration::from_secs(4));

			self.top_arm.stop()?;

		Ok(())
	}
}