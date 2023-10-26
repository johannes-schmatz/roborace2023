use std::time::Duration;
use anyhow::{anyhow, Context, Result};
use ev3dev_lang_rust::Button as Buttons;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, SensorPort};
use crate::menu::{Menu, MenuItem};
use crate::motor::Motor;

#[derive(Debug)]
pub(crate) struct Robot {
	pub(crate) buttons: Buttons,

	pub(crate) menu: Menu,

	pub(crate) color: ColorSensor,
	//pub gyro: GyroSensor,
	pub(crate) left: Motor,
	pub(crate) right: Motor,
}

impl Robot {
	pub(crate) fn new() -> Result<Robot> {
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
				left.set_stop_action(LargeMotor::STOP_ACTION_BRAKE)?;
				left.set_speed_sp(left.get_max_speed()?)?;
				Motor::new(left, "left")
			},
			right: {
				let right = LargeMotor::get(MotorPort::OutB)
					.context("Failed to get the right motor")?;
				right.set_polarity(LargeMotor::POLARITY_INVERSED)?;
				right.set_stop_action(LargeMotor::STOP_ACTION_BRAKE)?;
				right.set_speed_sp(right.get_max_speed()?)?;
				Motor::new(right, "right")
			},
		})
	}

	pub(crate) fn test(&self) -> Result<()> {
		dbg!(&self.left);
		dbg!(&self.right);

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

macro_rules! create_robot_state {
	(
		#[$attr:meta]
		$vis:vis enum $name:ident {
			$(
				$(#[$variant_attr:meta])?
				$variant:ident = $variant_name:literal: ($variant_padding:literal) $variant_desc:literal,
			)*
		}
	) => {
		#[$attr]
		$vis enum $name {
			$(
				$(#[$variant_attr])?
				$variant,
			)*
		}

		impl $name {
			fn create(string: &str) -> Option<$name> {
				match string {
					$(
						$variant_name => Some($name::$variant),
					)*
					_ => None,
				}
			}

			fn help_text() -> &'static str {
				concat!(
					"Usage:", '\n',
					'\t', "roborace2023 [<subcommand>]", '\n',
					'\n',
					"Where <subcommand> is one of:", '\n',
					$(
						'\t', $variant_name, $variant_padding, $variant_desc, '\n',
					)*
					'\n',
					"If no subcommand is given, the robot will go into menu mode."
				)
			}
		}
	}
}

create_robot_state!(
	#[derive(Debug, Clone, Default, PartialEq)]
	pub(crate) enum RobotState {
		Exit = "exit": ("\t\t")
			"Print out this help text and exit.",

		#[default]
		InMenu = "menu": ("\t\t")
			"Open the menu for selecting any robot state",
		Test = "test": ("\t\t")
			"Run the quick and dirty test method",

		LineMeasure = "line-measure": ("\t")
			"Measure the line. This has no implementation yet and will panic.",
		LineDrive = "line": ("\t\t")
			"Start the line driving.",

		GradientMeasure = "grad-measure": ("\t")
			"Measure the gradient. For this, position the robot in a right angle to the drive lane",
		GradientDrive = "grad": ("\t\t")
			"Start the gradient driving.",
	}
);

impl RobotState {
	/// `Ok(None)` indicates to terminate the program
	pub(crate) fn get_initial() -> Result<RobotState> {
		if let Some(arg) = std::env::args().skip(1).next() {
			if arg == "help" {
				let str = RobotState::help_text();
				eprintln!("{}", str);

				std::process::exit(0)
			}

			Self::create(&arg)
				.ok_or_else(|| {
					let str = RobotState::help_text();
					eprintln!("{}", str);

					anyhow!("No sub-command {arg:?} known")
				})
		} else {
			Ok(RobotState::InMenu)
		}
	}
}