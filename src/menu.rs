use anyhow::{Context, Result};
use crate::robot::button::Button;
use crate::robot::Robot;
use crate::robot::state::RobotState;

#[derive(Debug, Clone)]
pub(crate) struct Menu {
	items: Vec<(&'static str, RobotState)>,
}

impl Menu {
	pub(crate) fn new() -> Menu {
		let items = RobotState::get_menu_items();
		Menu { items }
	}

	#[cfg(not(feature = "menu"))]
	pub(crate) fn select(&self, bot: &Robot) -> Result<Option<RobotState>> {
		let mut cursor = 0;

		loop {
			std::process::Command::new("clear").status()
				.context("Failed to clear screen")?;
			for (name, _) in &self.items {
				println!("- {}", name);
			}

			println!("selected: {:?}", self.items.get(cursor).map(|x| x.0));

			cursor = match bot.buttons.await_press() {
				Button::Enter => {
					return Ok(Some(self.items[cursor].1.clone()));
				},
				Button::Left => {
					return Ok(Some(RobotState::Exit));
				},
				Button::Down if cursor + 1 >= self.items.len() => 0,
				Button::Down => cursor + 1,
				Button::Up if cursor == 0 => self.items.len() - 1,
				Button::Up => cursor - 1,
				_ => cursor,
			};
		}
	}

	#[cfg(feature = "menu")]
	/// Return `Ok(Some(new_state))` to set a new robot state, `Ok(None)` to not change it.
	pub(crate) fn select(&self, bot: &Robot) -> Result<Option<RobotState>> {
		let mut lcd = crate::lcd::Lcd::new()
			.context("Failed to create lcd")?;

		let mut cursor = 0;
		let mut top_item = 0;

		loop {
			lcd.clear();
			let height = 8.min(self.items.len());
			for line in 0..height {
				let index = top_item + line;
				if let Some((name, _)) = self.items.get(index) {
					let sel_char = if index == cursor { '>' } else { ' ' };

					lcd.draw_char(sel_char, 0, line);
					lcd.draw_str(name, 1, line);
				}
			}
			lcd.update();

			// The cursor is always above or equals to the top item.
			// The cursor is always below the top item + the height of the display.
			assert!(top_item <= cursor);
			assert!(cursor < top_item + height);

			// The cursor is always at a valid position in the items list.
			assert!(cursor < self.items.len());


			(cursor, top_item) = match bot.buttons.await_press() {
				Button::Enter => {
					let new_state = self.items.get(cursor)
						.ok_or_else(|| anyhow::anyhow!("Index out of bounds: cursor is {cursor}, but items length is {}", self.items.len()))?
						.1.clone();
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