use anyhow::{bail, Result};

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum RobotState {
	Exit,
	#[default]
	InMenu,
	Test,
	Measure,

	DriveSimpleOnly, // for testing our PID values without constant sideways drag

	DriveEntry,
	DriveFollow,
	DriveExit,
}

impl RobotState {
	pub(crate) const HELP_TEXT: &'static str =
		"Usage:\
		\n    roborace2023 [<subcommand>]\
		\n\
		\nWhere <subcommand> is one of:\
		\n    exit            Print out this help text and exit\
		\n    menu            Open the menu for selecting any robot state\
		\n    test            Run the quick and dirty test method\
		\n    measure         Measure the line. This has no implementation yet and will panic\
		\n    drive           Start the line driving\
		\n    driveS          Drive simple only, for testing PID values\
		\n    l|r DEGREE      Move the corresponding motor a tiny bit, depending on DEGREE\
		\n    print           Print the robot struct out, for debugging.\
		\n\
		\nIf no subcommand is given, the robot will go into menu mode";

	pub(crate) const ALL: &'static [(&'static str, RobotState)] = &[
		("exit", RobotState::Exit),
		("menu", RobotState::InMenu),
		("test", RobotState::Test),
		("measure", RobotState::Measure),
		("drive entry", RobotState::DriveEntry),
		("drive follow", RobotState::DriveFollow),
		("drive exit", RobotState::DriveExit),
		("drive sim", RobotState::DriveSimpleOnly),
	];
}