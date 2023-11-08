use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum RobotState {
	Exit,
	#[default]
	InMenu,
	Test,
	LineMeasure,
	LineDrive,
}

impl RobotState {
	fn create(string: &str) -> Option<RobotState> {
		match string {
			"exit" => Some(RobotState::Exit),
			"menu" => Some(RobotState::InMenu),
			"test" => Some(RobotState::Test),
			"line-measure" => Some(RobotState::LineMeasure),
			"line" => Some(RobotState::LineDrive),
			_ => None,
		}
	}

	const HELP_TEXT: &'static str =
		"Usage:\
		\n    roborace2023 [<subcommand>]\
		\n\
		\nWhere <subcommand> is one of:\
		\n    exit            Print out this help text and exit\
		\n    menu            Open the menu for selecting any robot state\
		\n    test            Run the quick and dirty test method\
		\n    line-measure    Measure the line. This has no implementation yet and will panic\
		\n    line            Start the line driving\
		\n\
		\nIf no subcommand is given, the robot will go into menu mode";

	pub(crate) const ALL: &'static [(&'static str, RobotState)] = &[
		("exit", RobotState::Exit),
		("menu", RobotState::InMenu),
		("test", RobotState::Test),
		("line-measure", RobotState::LineMeasure),
		("line", RobotState::LineDrive),
	];


	/// `Ok(None)` indicates to terminate the program
	pub(crate) fn get_initial() -> Result<RobotState> {
		if let Some(arg) = std::env::args().skip(1).next() {
			if arg == "help" {
				eprintln!("{}", RobotState::HELP_TEXT);

				std::process::exit(0)
			}

			Self::create(&arg)
				.ok_or_else(|| {
					eprintln!("{}", RobotState::HELP_TEXT);

					anyhow!("No sub-command {arg:?} known")
				})
		} else {
			Ok(RobotState::InMenu)
		}
	}
}