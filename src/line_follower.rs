use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::pid::Pid;
use crate::robot::Robot;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineFollower {
	pub pid: Pid,
}

impl Default for LineFollower {
	fn default() -> Self {
		LineFollower {
			pid: Pid::new(0.1, 0.0, 0.1),
		}
	}
}

impl LineFollower {
	#[allow(unused_variables)]
	pub fn measure(&self, bot: &Robot) -> Result<()> {
		todo!()
	}

	#[allow(unused_variables)]
	pub fn prepare_drive(&mut self, bot: &Robot) -> Result<()> {
		todo!()
	}

	#[allow(unused_variables)]
	pub fn drive(&mut self, bot: &Robot) -> Result<()> {
		todo!()
	}
}