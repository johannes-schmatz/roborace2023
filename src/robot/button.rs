use anyhow::{Context, Result};
use std::time::Duration;

macro_rules! button_function {
	($self:ident, $name:ident, $ret:path) => {
		if $self.inner.$name() {
			while $self.inner.$name() {
				std::thread::sleep(Duration::from_millis(10));
				$self.inner.process();
			}
			return $ret;
		}
	}
}

#[derive(Debug)]
pub(crate) struct Buttons {
	inner: ev3dev_lang_rust::Button,
}

impl Buttons {
	pub(crate) fn new() -> Result<Buttons> {
		Ok(Buttons {
			inner: ev3dev_lang_rust::Button::new()
				.context("Failed to create buttons")?,
		})
	}

	pub(crate) fn await_press(&self) -> Button {
		loop {
			self.inner.process();

			button_function!(self, is_up,        Button::Up       );
			button_function!(self, is_down,      Button::Down     );
			button_function!(self, is_left,      Button::Left     );
			button_function!(self, is_right,     Button::Right    );
			button_function!(self, is_enter,     Button::Enter    );

			std::thread::sleep(Duration::from_millis(10));
		}
	}

	pub(crate) fn update(&self) {
		self.inner.process();
	}

	pub(crate) fn is_left(&self) -> bool {
		self.inner.is_left()
	}
}

#[derive(Debug)]
pub(crate) enum Button {
	Up, Down, Left, Right, Enter
}