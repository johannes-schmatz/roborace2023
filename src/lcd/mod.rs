use anyhow::{Context, Result};
use ev3dev_lang_rust::Screen;
use crate::lcd::font::Font;
use image::Rgb;

pub mod font;

pub struct Lcd {
	screen: Screen,
	font: &'static Font,
}

impl Lcd {
	pub fn new() -> Result<Lcd> {
		Ok(Lcd {
			screen: Screen::new().with_context(|| "Failed to get screen")?,
			font: &Font::MEDIUM,
		})
	}

	pub fn clear(&mut self) {
		self.screen.clear();
	}
	pub fn update(&mut self) {
		self.screen.update();
	}

	pub fn draw_char(&mut self, ch: char, column: usize, line: usize) {
		for font_x in 0..self.font.width {
			for font_y in 0..self.font.height {
				let x = self.font.width * column + font_x;
				let y = self.font.height * line + font_y;

				if let Some(pixel) = self.screen.image.get_pixel_mut_checked(x as u32, y as u32) {
					if let Ok(true) = self.font.get_pixel(ch, font_x, font_y) {
						*pixel = Rgb([0, 0, 0]);
					} else {
						// on failure, we clean the pixel
						*pixel = Rgb([255, 255, 255]);
					}
				}
				// we ignore the failure
			}
		}
	}
	pub fn draw_str(&mut self, string: &str, column: usize, line: usize) {
		for (i, ch) in string.chars().enumerate() {
			self.draw_char(ch, column + i, line);
		}
	}
}

