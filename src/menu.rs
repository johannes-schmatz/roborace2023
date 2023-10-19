use std::time::Duration;
use anyhow::{anyhow, Context, Result};
use ev3dev_lang_rust::{Button as Buttons};
use crate::lcd::Lcd;
use crate::robot::RobotState;

#[derive(Debug)]
pub struct Menu {
	buttons: Buttons,
	items: Vec<MenuItem>,
}

impl Menu {
	pub fn new(buttons: Buttons, items: Vec<MenuItem>) -> Menu {
		assert!(!items.is_empty());
		Menu { buttons, items }
	}

	/// Return `Ok(Some(new_state))` to set a new robot state, `Ok(None)` to not change it.
	pub fn select(&self) -> Result<Option<RobotState>> {
		let mut lcd = Lcd::new()
			.with_context(|| "Failed to create lcd")?;

		let mut cursor = 0;
		let mut top_item = 0;

		loop {
			lcd.clear();
			let height = 8.min(self.items.len());
			for line in 0..height {
				let index = top_item + line;
				if let Some(item) = self.items.get(index) {
					let sel_char = if index == cursor { '>' } else { ' ' };

					lcd.draw_char(sel_char, 0, line);
					lcd.draw_str(item.name, 1, line);
				}
			}
			lcd.update();

			// The cursor is always above or equals to the top item.
			// The cursor is always below the top item + the height of the display.
			assert!(top_item <= cursor);
			assert!(cursor < top_item + height);

			// The cursor is always at a valid position in the items list.
			assert!(cursor < self.items.len());


			(cursor, top_item) = match Button::await_press(&self.buttons) {
				Button::Enter => {
					let new_state = self.items.get(cursor)
						.ok_or_else(|| anyhow!("Index out of bounds: cursor is {cursor}, but items length is {}", self.items.len()))?
						.new_state.clone();
					return Ok(Some(new_state));
				},
				Button::Left => {
					// Backspace exits the robot program
					return Ok(Some(RobotState::Exit));
				},

				// Going down from right before the end jumps to the very start
				Button::Down if cursor + 1 >= self.items.len() => (0, 0),

				// Going down above the displayed length moves the cursor, but also the list
				Button::Down if cursor + 1 == top_item + height => (cursor + 1, top_item + 1),

				// Otherwise don't move the list
				Button::Down => (cursor + 1, top_item),

				// Going up from zero means jumping at the end
				Button::Up if cursor == 0 => (self.items.len() - 1, 0.max(self.items.len() - 8)),

				// Going up below the displayed items moves the cursor below, and also the list
				Button::Up if cursor == top_item => (cursor - 1, top_item - 1),

				// Otherwise don't move the list
				Button::Up => (cursor - 1, top_item),

				// Any other button has no effect
				_ => (cursor, top_item),
			};
		}
	}
}

#[derive(Debug)]
pub struct MenuItem {
	name: &'static str,
	/// The new robot state to set when this item is selected.
	new_state: RobotState,
}

impl MenuItem {
	pub fn new(name: &'static str, new_state: RobotState) -> MenuItem {
		MenuItem { name, new_state }
	}
}

#[derive(Debug)]
pub enum Button { // TODO: move to own mod
	Up, Down, Left, Right, Enter
}

macro_rules! button_function {
	($buttons:ident, $name:ident, $ret:path) => {
		if $buttons.$name() {
			while $buttons.$name() {
				std::thread::sleep(Duration::from_millis(10));
				$buttons.process();
			}
			return $ret;
		}
	}
}

impl Button {
	fn await_press(buttons: &Buttons) -> Button {
		loop {
			buttons.process();

			button_function!(buttons, is_up,        Button::Up       );
			button_function!(buttons, is_down,      Button::Down     );
			button_function!(buttons, is_left,      Button::Left     );
			button_function!(buttons, is_right,     Button::Right    );
			button_function!(buttons, is_enter,     Button::Enter    );

			std::thread::sleep(Duration::from_millis(10));
		}
	}
}