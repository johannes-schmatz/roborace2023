use anyhow::{Context, Result};
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::pid::Pid;

#[derive(Deserialize, Serialize)]
pub struct Settings {
	pub gradient_pid: Pid,
}

impl Default for Settings {
	fn default() -> Self {
		Settings {
			gradient_pid: Pid::new(0f64, 0f64, 0f64),
		}
	}
}

impl Settings {
	fn path() -> &'static Path {
		Path::new("./robot_settings.toml")
	}

	pub fn get() -> Result<Settings> {
		let path = Self::path();

		if path.exists() {
			let string = std::fs::read_to_string(path)
				.context("Failed to read settings file")?;
			toml::from_str(&string)
				.context("Failed to parse settings")
		} else {
			println!("No settings file found, writing new settings file to {path:?}");

			let settings = Settings::default();

			let string = toml::to_string_pretty(&settings)
				.context("Failed to serialize the settings")?;
			std::fs::write(path, string)
				.context("Failed to write settings file")?;

			Ok(settings)
		}
	}

	pub fn write(self) -> Result<()> {
		let path = Self::path();

		let out = toml::to_string_pretty(&self)
			.context("Failed to serialize the settings")?;
		let read = std::fs::read_to_string(path).unwrap_or_else(|_| String::new());

		if out != read {
			println!("Updating settings file {path:?}");
			std::fs::write(path, out)
				.context("Failed to write settings file")?;
		}

		Ok(())
	}
}