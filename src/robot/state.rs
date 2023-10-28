use anyhow::{anyhow, Result};

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

			const HELP_TEXT: &str = concat!(
				"Usage:", '\n',
				'\t', "roborace2023 [<subcommand>]", '\n',
				'\n',
				"Where <subcommand> is one of:", '\n',
				$(
					'\t', $variant_name, $variant_padding, $variant_desc, '\n',
				)*
				'\n',
				"If no subcommand is given, the robot will go into menu mode."
			);

			pub(crate) const ALL: &[(&'static str, RobotState)] = &[
				$(
					($variant_name, $name::$variant),
				)*
			];
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
	}
);

impl RobotState {
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