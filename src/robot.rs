use anyhow::{Context, Result};
use ev3dev_lang_rust::Button as Buttons;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};
use crate::gradient_follower::GradientFollower;
use crate::line_follower::LineFollower;
use crate::menu::{Menu, MenuItem};
use crate::motor::Motor;

#[derive(Debug)]
pub struct Robot {
	state: RobotState,

	buttons: Buttons,

	menu: Menu,

	line_follower: LineFollower,
	gradient_follower: GradientFollower,

	color: ColorSensor,
	//gyro: GyroSensor,
	left: Motor,
	right: Motor,
}

impl Robot {
	pub fn new() -> Result<Robot> {
		let buttons = Buttons::new()
			.context("Failed to get the robot buttons")?;

		let left = LargeMotor::get(MotorPort::OutA)
			.context("Failed to get the left motor")?;
		left.set_polarity(LargeMotor::POLARITY_INVERSED)?;
		let left = Motor::new(left, "left");

		let right = LargeMotor::get(MotorPort::OutB)
			.context("Failed to get the right motor")?;
		right.set_polarity(LargeMotor::POLARITY_INVERSED)?;
		let right = Motor::new(right, "right");

		let color = ColorSensor::get(SensorPort::In1)
			.context("Failed to get the color sensor")?;

		//let gyro = GyroSensor::get(SensorPort::In2)
		//	.context("Failed to get the gyro sensor")?;

		Ok(Robot {
			state: RobotState::InMenu,

			buttons: buttons.clone(),

			menu: Menu::new(buttons, vec![
				MenuItem::new("Line Measure", RobotState::LineMeasure),
				MenuItem::new("Line Start", RobotState::LineDrive),
				MenuItem::new("Gradient Measure", RobotState::GradientMeasure),
				MenuItem::new("Gradient Start", RobotState::GradientDrive),
			]),

			line_follower: LineFollower::new(color.clone(), left.clone(), right.clone()),
			gradient_follower: GradientFollower::new(color.clone(), /*gyro.clone(), */left.clone(), right.clone()),

			color, /*gyro, */left, right,
		})
	}

	pub fn test(&mut self) -> Result<()> {
		self.left.inner.set_stop_action(LargeMotor::STOP_ACTION_BRAKE).context("hold")?;
		self.right.inner.set_stop_action(LargeMotor::STOP_ACTION_BRAKE).context("hold")?;

		self.left.inner.set_speed_sp(1000).context("speed sp")?;
		self.right.inner.set_speed_sp(1000).context("speed sp")?;

		self.left.set_speed(100)?;
		self.right.set_speed(100)?;

		self.left.start()?;
		self.right.start()?;

		std::thread::sleep(Duration::from_secs(3));

		self.left.set_speed(-100)?;
		self.right.set_speed(-100)?;

		std::thread::sleep(Duration::from_secs(4));

		self.left.set_speed(100)?;
		self.right.set_speed(100)?;

		std::thread::sleep(Duration::from_secs(1));

		self.left.stop()?;
		self.right.stop()?;

		Ok(())
	}

	/// Return `Ok(true)` to end the program, `Ok(false)` otherwise.
	pub fn tick(&mut self) -> Result<bool> {
		self.buttons.process();
		if self.buttons.is_left() {
			std::thread::sleep(Duration::from_millis(400));
			self.next_state(RobotState::InMenu)?;
		}

		match self.state {
			RobotState::Exit => return Ok(true),
			RobotState::InMenu => {
				if let Some(new_state) = self.menu.select()? {
					self.next_state(new_state)?;
				}
			},
			RobotState::LineMeasure => {
				todo!();
				self.next_state(RobotState::InMenu)?;
			},
			RobotState::LineDrive => todo!(),
			RobotState::GradientMeasure => {
				self.gradient_follower.measure()?;
				self.next_state(RobotState::InMenu)?;
			},
			RobotState::GradientDrive => {
				self.gradient_follower.drive()?;
			},
		}

		Ok(false)
	}

	fn next_state(&mut self, new_state: RobotState) -> Result<()> {
		match (&self.state, &new_state) {
			(_, RobotState::LineMeasure) | (_, RobotState::LineDrive) => {
				todo!()
			},
			(_, RobotState::GradientMeasure) => {},
			(_, RobotState::GradientDrive) => {
				self.gradient_follower.prepare_drive()
					.context("Failed to prepare for gradient drive")?
			},
			(RobotState::LineDrive, _) | (RobotState::GradientDrive, _) => {
				self.left.stop()?;
				self.right.stop()?;
			}
			(_, _) => {},
		}
		self.state = new_state;
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum RobotState {
	Exit,
	InMenu,
	LineMeasure,
	LineDrive,
	GradientMeasure,
	GradientDrive,
}
