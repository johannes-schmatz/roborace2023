use std::fmt::{Debug, Formatter};
use anyhow::{Context, Result};
pub(crate) use ev3dev_lang_rust::sensors::{
	ColorSensor as Ev3ColorSensor,
	TouchSensor as Ev3TouchSensor,
	UltrasonicSensor as Ev3DistanceSensor
};

fn fmt<'a, T: Debug, E>(value: &'a Result<T, E>) -> &'a dyn Debug {
	if let Ok(v) = value {
		v
	} else {
		&""
	}
}

pub(crate) struct ColorSensor {
	inner: Ev3ColorSensor,
}

impl Debug for ColorSensor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Color")
			.field("color", fmt(&self.inner.get_color()))
			.field("red", fmt(&self.inner.get_red()))
			.field("green", fmt(&self.inner.get_green()))
			.field("blue", fmt(&self.inner.get_blue()))
			.finish()
	}
}

impl ColorSensor {
	pub(crate) fn new(inner: Ev3ColorSensor) -> ColorSensor {
		ColorSensor { inner }
	}

	pub(crate) fn get_color(&self) -> Result<f64> {
		let color = self.inner.get_color()
			.context("Failed to get color from sensor")?;
		Ok(color as f64)
	}
}

pub(crate) struct DistanceSensor {
	inner: Ev3DistanceSensor,
}

impl Debug for DistanceSensor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Distance")
			.field("distance", fmt(&self.inner.get_distance_centimeters()))
			.finish()
	}
}

impl DistanceSensor {
	pub(crate) fn new(inner: Ev3DistanceSensor) -> DistanceSensor {
		DistanceSensor { inner }
	}

	/// Gets the distance in `cm`, or [None] if either too far away or too close.
	/// `0 ..= 254.0`
	pub(crate) fn get_distance(&self) -> Result<Option<f64>> {
		let distance = self.inner.get_distance_centimeters()
			.context("Failed to get the distance from sensor")?;
		if distance == 255.0 {
			Ok(None)
		} else {
			Ok(Some(distance as f64))
		}
	}
}

pub(crate) struct TouchSensor {
	inner: Ev3TouchSensor,
}

impl Debug for TouchSensor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Touch")
			.field("pressed", fmt(&self.inner.get_pressed_state()))
			.finish()
	}
}

impl TouchSensor {
	pub(crate) fn new(inner: Ev3TouchSensor) -> TouchSensor {
		TouchSensor { inner }
	}

	pub(crate) fn is_pressed(&self) -> Result<bool> {
		self.inner.get_pressed_state()
			.context("Failed to get press state from sensor")
	}
}