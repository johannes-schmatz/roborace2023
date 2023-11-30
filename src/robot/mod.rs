use anyhow::{Context, Result};
use ev3dev_lang_rust::motors::MotorPort;
use ev3dev_lang_rust::sensors::SensorPort;
use crate::robot::button::Buttons;
use crate::robot::motor::{Ev3LargeMotor, Ev3SmallMotor, LargeMotor, SmallMotor};
use crate::robot::sensors::{ColorSensor, DistanceSensor, Ev3ColorSensor, Ev3DistanceSensor, Ev3TouchSensor, TouchSensor};

pub(crate) mod motor;
pub(crate) mod button;
pub(crate) mod sensors;

#[derive(Debug)]
pub(crate) struct Robot {
	pub(crate) buttons: Buttons,

	pub(crate) color: ColorSensor,
	pub(crate) distance: DistanceSensor,
	pub(crate) touch: TouchSensor,

	pub(crate) left: LargeMotor,
	pub(crate) right: LargeMotor,

	pub(crate) top_arm: SmallMotor,
}

impl Robot {
	pub(crate) fn new() -> Result<Robot> {
		Ok(Robot {
			buttons: Buttons::new()
				.context("Failed to get the robot buttons")?,

			color: {
				let color = Ev3ColorSensor::get(SensorPort::In1)
					.context("Failed to get the color sensor")?;
				color.set_mode_col_reflect().context("Failed to set color mode")?;
				ColorSensor::new(color)
			},
			distance: {
				let distance = Ev3DistanceSensor::get(SensorPort::In3)
					.context("Failed to get the ultrasonic sensor")?;
				distance.set_mode_us_dist_cm().context("Failed to set distance mode")?;
				DistanceSensor::new(distance)
			},
			touch: {
				let touch = Ev3TouchSensor::get(SensorPort::In2)
					.context("Failed to get the touch sensor")?;
				TouchSensor::new(touch)
			},

			left: {
				let motor = Ev3LargeMotor::get(MotorPort::OutB)
					.context("Failed to get the left motor")?;
				//motor.set_polarity(Ev3LargeMotor::POLARITY_INVERSED)?;
				motor.set_stop_action(Ev3LargeMotor::STOP_ACTION_BRAKE)?;
				motor.set_speed_sp(motor.get_max_speed()?)?;
				LargeMotor::new(motor, "left")
			},
			right: {
				let motor = Ev3LargeMotor::get(MotorPort::OutA)
					.context("Failed to get the right motor")?;
				//motor.set_polarity(Ev3LargeMotor::POLARITY_INVERSED)?;
				motor.set_stop_action(Ev3LargeMotor::STOP_ACTION_BRAKE)?;
				motor.set_speed_sp(motor.get_max_speed()?)?;
				LargeMotor::new(motor, "right")
			},

			top_arm: {
				let motor = Ev3SmallMotor::get(MotorPort::OutC)
					.context("Failed to get the medium motor")?;
				motor.set_polarity(Ev3SmallMotor::POLARITY_INVERSED)?;
				motor.set_stop_action(Ev3SmallMotor::STOP_ACTION_COAST)?;
				motor.set_speed_sp(motor.get_max_speed()?)?;
				SmallMotor::new(motor, "top")
			},
		})
	}

	pub(crate) fn beep(&self) -> Result<()> {
		ev3dev_lang_rust::sound::beep()?;
		Ok(())
	}
}