use anyhow::{Context, Result};
use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, SensorPort, TouchSensor, UltrasonicSensor};
use crate::robot::button::Buttons;
use crate::robot::motor::Motor;
use crate::robot::sensors::{Color, Distance, Touch};

pub(crate) mod state;
pub(crate) mod motor;
pub(crate) mod button;
pub(crate) mod sensors;

#[derive(Debug)]
pub(crate) struct Robot {
	pub(crate) buttons: Buttons,

	pub(crate) color: Color,
	pub(crate) distance: Distance,
	pub(crate) touch: Touch,

	pub(crate) left: Motor,
	pub(crate) right: Motor,

	pub(crate) top_arm: MediumMotor,
}

impl Robot {
	pub(crate) fn new() -> Result<Robot> {
		Ok(Robot {
			buttons: Buttons::new()
				.context("Failed to get the robot buttons")?,

			color: {
				let color = ColorSensor::get(SensorPort::In1)
					.context("Failed to get the color sensor")?;
				color.set_mode_col_reflect().context("Failed to set color mode")?;
				Color::new(color)
			},
			distance: {
				let distance = UltrasonicSensor::get(SensorPort::In3)
					.context("Failed to get the ultrasonic sensor")?;
				distance.set_mode_us_dist_cm().context("Failed to set distance mode")?;
				Distance::new(distance)
			},
			touch: {
				let touch = TouchSensor::get(SensorPort::In2)
					.context("Failed to get the touch sensor")?;
				Touch::new(touch)
			},

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
}