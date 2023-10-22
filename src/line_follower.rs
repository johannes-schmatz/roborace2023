use anyhow::Result;
use ev3dev_lang_rust::sensors::ColorSensor;
use crate::motor::Motor;
use crate::settings::Settings;

#[derive(Debug)]
pub struct LineFollower {
	color: ColorSensor,
	left: Motor,
	right: Motor,
}

impl LineFollower {
	pub fn new(settings: &Settings, color: ColorSensor, left: Motor, right: Motor) -> LineFollower {
		LineFollower { color, left, right }
	}

	pub fn tick(&mut self) -> Result<()> {
		Ok(())
	}
}