use std::time::Duration;
use ev3dev_lang_rust::Button as Buttons;

#[derive(Debug)]
pub(crate) enum Button { // TODO: move to own mod
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
	pub(crate) fn await_press(buttons: &Buttons) -> Button {
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