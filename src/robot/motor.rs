use std::fmt::{Debug, Formatter};
use anyhow::{anyhow, Context, Result};
pub(crate) use ev3dev_lang_rust::motors::LargeMotor as Ev3LargeMotor;
pub(crate) use ev3dev_lang_rust::motors::MediumMotor as Ev3SmallMotor;

fn fmt<'a, T: Debug, E>(value: &'a Result<T, E>) -> &'a dyn Debug {
	if let Ok(v) = value {
		v
	} else {
		&""
	}
}

#[derive(Clone)]
pub(crate) struct LargeMotor {
	inner: Ev3LargeMotor,
	desc: &'static str,
}

impl Debug for LargeMotor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Motor")
			.field("desc", &self.desc)
			.field("position", fmt(&self.inner.get_position()))
			.field("position", fmt(&self.inner.get_position()))
			.field("position_sp", fmt(&self.inner.get_position_sp()))
			.field("speed", fmt(&self.inner.get_speed()))
			.field("speed_sp", fmt(&self.inner.get_speed_sp()))
			.field("max_speed", fmt(&self.inner.get_max_speed()))
			.field("duty_cycle", fmt(&self.inner.get_duty_cycle()))
			.field("duty_cycle_sp", fmt(&self.inner.get_duty_cycle_sp()))
			.field("polarity", fmt(&self.inner.get_polarity()))
			.field("time_sp", fmt(&self.inner.get_time_sp()))
			.field("stop_action", fmt(&self.inner.get_stop_action()))
			// seems to crash here
			//.field("count_per_m", fmt(&self.inner.get_count_per_m()))
			//.field("count_per_rot", fmt(&self.inner.get_count_per_rot()))
			//.field("full_travel_count", fmt(&self.inner.get_full_travel_count()))
			.field("ramp_down_sp", fmt(&self.inner.get_ramp_down_sp()))
			.field("ramp_up_sp", fmt(&self.inner.get_ramp_up_sp()))
			.finish()
	}
}

impl LargeMotor {
	pub(crate) fn new(inner: Ev3LargeMotor, desc: &'static str) -> LargeMotor {
		LargeMotor { inner, desc }
	}

	pub(crate) fn start(&self) -> Result<()> {
		self.inner.run_direct().with_context(|| anyhow!("Failed to run motor {}", self.desc))
	}

	pub(crate) fn set_speed(&self, speed: f64) -> Result<()> {
		let mut speed = speed as i32;
		if speed > 100 {
			speed = 100;
		}
		if speed < -100 {
			speed = -100;
		}
		self.inner.set_duty_cycle_sp(speed).with_context(|| anyhow!("Failed to set speed {speed} for {}", self.desc))
	}

	pub(crate) fn stop(&self) -> Result<()> {
		self.inner.stop().with_context(|| anyhow!("Failed to stop motor {}", self.desc))
	}
}





#[derive(Clone)]
pub(crate) struct SmallMotor {
	inner: Ev3SmallMotor,
	desc: &'static str,
}

impl Debug for SmallMotor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Motor")
			.field("desc", &self.desc)
			.field("position", fmt(&self.inner.get_position()))
			.field("position", fmt(&self.inner.get_position()))
			.field("position_sp", fmt(&self.inner.get_position_sp()))
			.field("speed", fmt(&self.inner.get_speed()))
			.field("speed_sp", fmt(&self.inner.get_speed_sp()))
			.field("max_speed", fmt(&self.inner.get_max_speed()))
			.field("duty_cycle", fmt(&self.inner.get_duty_cycle()))
			.field("duty_cycle_sp", fmt(&self.inner.get_duty_cycle_sp()))
			.field("polarity", fmt(&self.inner.get_polarity()))
			.field("time_sp", fmt(&self.inner.get_time_sp()))
			.field("stop_action", fmt(&self.inner.get_stop_action()))
			// seems to crash here
			//.field("count_per_m", fmt(&self.inner.get_count_per_m()))
			//.field("count_per_rot", fmt(&self.inner.get_count_per_rot()))
			//.field("full_travel_count", fmt(&self.inner.get_full_travel_count()))
			.field("ramp_down_sp", fmt(&self.inner.get_ramp_down_sp()))
			.field("ramp_up_sp", fmt(&self.inner.get_ramp_up_sp()))
			.finish()
	}
}

impl SmallMotor {
	pub(crate) fn new(inner: Ev3SmallMotor, desc: &'static str) -> SmallMotor {
		SmallMotor { inner, desc }
	}

	pub(crate) fn start(&self) -> Result<()> {
		self.inner.run_direct().with_context(|| anyhow!("Failed to run motor {}", self.desc))
	}

	pub(crate) fn set_speed(&self, speed: f64) -> Result<()> {
		let mut speed = speed as i32;
		if speed > 100 {
			speed = 100;
		}
		if speed < -100 {
			speed = -100;
		}
		self.inner.set_duty_cycle_sp(speed).with_context(|| anyhow!("Failed to set speed {speed} for {}", self.desc))
	}

	pub(crate) fn stop(&self) -> Result<()> {
		self.inner.stop().with_context(|| anyhow!("Failed to stop motor {}", self.desc))
	}
}