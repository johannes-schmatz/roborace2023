use anyhow::Result;
use ev3dev_lang_rust::motors::LargeMotor;

#[derive(Debug)]
pub struct LineFollower {
}

impl LineFollower {
	pub fn new(left: LargeMotor, right: LargeMotor) -> LineFollower {
		LineFollower { }
	}

	pub fn tick(&mut self) -> Result<()> {
		Ok(())
	}
}