use anyhow::{Context, Result};
use std::path::Path;
use crate::program::Program;

fn path() -> &'static Path {
	Path::new("./robot_settings.toml")
}

pub(crate) fn read() -> Result<Program> {
	let path = path();

	if path.exists() {
		let string = std::fs::read_to_string(path)
			.context("Failed to read settings file")?;
		toml::from_str(&string)
			.context("Failed to parse settings")
	} else {
		println!("No settings file found, writing new settings file to {path:?}");

		let settings = Program::default();

		let string = toml::to_string_pretty(&settings)
			.context("Failed to serialize the settings")?;
		std::fs::write(path, string)
			.context("Failed to write settings file")?;

		Ok(settings)
	}
}

pub(crate) fn write(settings: Program) -> Result<()> {
	let path = path();

	let out = toml::to_string_pretty(&settings)
		.context("Failed to serialize the settings")?;
	let read = std::fs::read_to_string(path).unwrap_or_else(|_| String::new());

	if out != read {
		println!("Updating settings file {path:?}");
		std::fs::write(path, out)
			.context("Failed to write settings file")?;
	}

	Ok(())
}