use anyhow::{Context, Result};
pub(crate) use ev3dev_lang_rust::sensors::{
	ColorSensor as Ev3ColorSensor,
	TouchSensor as Ev3TouchSensor,
	UltrasonicSensor as Ev3DistanceSensor
};

#[derive(Debug)] // TODO: see Motor for how to impl
pub(crate) struct ColorSensor {
	inner: Ev3ColorSensor,
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

#[derive(Debug)] // TODO: see Motor for how to impl
pub(crate) struct DistanceSensor {
	inner: Ev3DistanceSensor,
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

#[derive(Debug)] // TODO: see Motor for how to impl
pub(crate) struct TouchSensor {
	inner: Ev3TouchSensor,
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