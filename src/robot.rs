use anyhow::{Context, Result};
use ev3dev_lang_rust::Button as Buttons;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use crate::menu::{Menu, MenuItem};

#[derive(Debug)]
pub struct Robot {
	state: RobotState,

	buttons: Buttons,

	menu: Menu,

	left: LargeMotor,
	right: LargeMotor,
}

impl Robot {
	pub fn new() -> Result<Robot> {
		let buttons = Buttons::new()
			.context("Failed to get the robot buttons")?;

		Ok(Robot {
			state: RobotState::InMenu,

			buttons: buttons.clone(),

			menu: Menu::new(buttons, vec![
				MenuItem::new("Measure", RobotState::Measure),
				MenuItem::new("Start", RobotState::Driving),
			]),

			left: LargeMotor::get(MotorPort::OutA)
				.context("Failed to get the left motor")?,
			right: LargeMotor::get(MotorPort::OutB)
				.context("Failed to get the right motor")?,
		})
	}

	/// Return `Ok(true)` to end the program, `Ok(false)` otherwise.
	pub fn tick(&mut self) -> Result<bool> {
		match self.state {
			RobotState::Exit => return Ok(true),
			RobotState::InMenu => {
				if let Some(new_state) = self.menu.select()? {
					self.state = new_state;
				}
			},
			RobotState::Measure => {
				todo!()
			},
			RobotState::Driving => {
				todo!()
			},
		}

		Ok(false)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum RobotState {
	Exit,
	InMenu,
	Driving,
	Measure,
}