use anyhow::{bail, Result};

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum RobotState {
	Exit,
	#[default]
	InMenu,
	Test,
	Measure,
	Driving,
	ApproachingWall,
}

impl RobotState {
	const HELP_TEXT: &'static str =
		"Usage:\
		\n    roborace2023 [<subcommand>]\
		\n\
		\nWhere <subcommand> is one of:\
		\n    exit            Print out this help text and exit\
		\n    menu            Open the menu for selecting any robot state\
		\n    test            Run the quick and dirty test method\
		\n    measure         Measure the line. This has no implementation yet and will panic\
		\n    drive           Start the line driving\
		\n\
		\nIf no subcommand is given, the robot will go into menu mode";

	pub(crate) const ALL: &'static [(&'static str, RobotState)] = &[
		("exit", RobotState::Exit),
		("menu", RobotState::InMenu),
		("test", RobotState::Test),
		("measure", RobotState::Measure),
		("drive", RobotState::Driving),
	];

	pub(crate) fn get_initial() -> Result<RobotState> {
		if let Some(arg) = std::env::args().skip(1).next() {
			if arg == "help" {
				eprintln!("{}", RobotState::HELP_TEXT);

				std::process::exit(0)
			}

			match arg.as_str() {
				"exit" => Ok(RobotState::Exit),
				"menu" => Ok(RobotState::InMenu),
				"test" => Ok(RobotState::Test),
				"measure" => Ok(RobotState::Measure),
				"drive" => Ok(RobotState::ApproachingWall),
				_ => {
					eprintln!("{}", RobotState::HELP_TEXT);

					bail!("No sub-command {arg:?} known");
				}
			}
		} else {
			Ok(RobotState::InMenu)
		}
	}
}