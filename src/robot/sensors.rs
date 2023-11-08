use anyhow::{Context, Result};
use ev3dev_lang_rust::sensors::{ColorSensor, TouchSensor, UltrasonicSensor};

#[derive(Debug)] // TODO: see Motor for how to impl
pub(crate) struct Color {
	inner: ColorSensor,
}

impl Color {
	pub(crate) fn new(inner: ColorSensor) -> Color {
		Color { inner }
	}

	pub(crate) fn get_color(&self) -> Result<f64> {
		let color = self.inner.get_color()
			.context("Failed to get color from sensor")?;
		Ok(color as f64)
	}
}

#[derive(Debug)] // TODO: see Motor for how to impl
pub(crate) struct Distance {
	inner: UltrasonicSensor,
}

impl Distance {
	pub(crate) fn new(inner: UltrasonicSensor) -> Distance {
		Distance { inner }
	}

	/// Gets the distance in `cm`, or [None] if either too far away or too close.
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
pub(crate) struct Touch {
	inner: TouchSensor,
}

impl Touch {
	pub(crate) fn new(inner: TouchSensor) -> Touch {
		Touch { inner }
	}

	pub(crate) fn is_pressed(&self) -> Result<bool> {
		self.inner.get_pressed_state()
			.context("Failed to get press state from sensor")
	}
}