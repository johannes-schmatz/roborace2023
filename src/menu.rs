use anyhow::Result;
use crate::robot::button::Button;
use crate::robot::Robot;
use crate::robot::state::RobotState;

pub(crate) fn select(bot: &Robot) -> Result<Option<RobotState>> {
	let items = RobotState::ALL;

	for (name, _) in items {
		println!("- {}", name);
	}

	let mut cursor = 0;

	loop {
		println!("selected: {:?}", items.get(cursor).map(|x| x.0));

		cursor = match bot.buttons.await_press() {
			Button::Enter => {
				return Ok(Some(items[cursor].1.clone()));
			},
			Button::Left => {
				return Ok(Some(RobotState::Exit));
			},
			Button::Down if cursor + 1 >= items.len() => 0,
			Button::Down => cursor + 1,
			Button::Up if cursor == 0 => items.len() - 1,
			Button::Up => cursor - 1,
			_ => cursor,
		};
	}
}