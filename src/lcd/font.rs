use anyhow::{anyhow, bail, Result};

pub struct Font {
	/// The char corresponding to the first glyph of the font.
	first: char,
	/// How many glyphs the font stores.
	count: usize,
	/// The height of a glyph.
	pub width: usize,
	/// The width of a glyph.
	pub height: usize,
	/// The glyphs making up the font.
	glyphs: &'static [u8],
}

impl Font {
	pub const TINY: Font = Self::new(include_bytes!("tiny.raw"), 4, 6, 5, 4, 96, ' ');
	pub const SMALL: Font = Self::new(include_bytes!("small.raw"), 6, 8, 7, 6, 96, ' ');
	pub const MEDIUM: Font = Self::new(include_bytes!("medium.raw"), 10, 16, 14, 10, 96, ' ');
	pub const LARGE: Font = Self::new(include_bytes!("large.raw"), 20, 32, 28, 20, 96, ' ');

	/// Creates a new font. Use the following command to view the font files in your terminal:
	/// ```sh
	/// cat tiny.raw | \
	///   xxd -b -c 1 | \
	///   awk '{print $2}' | \
	///   paste -sd '' | \
	///   sed -Ee 's/(....)(....)/\2\1/g;s/(.)(.)(.)(.)/\4\3\2\1/g;s/([01]{384})/\1\n/g;y/01/ #/' | \
	///   sed -Ee 's/^(.{200}).*/\1/g'
	/// ```
	/// (the last sed makes it so that you only see the first 200 chars in each line)
	const fn new(glyphs: &'static [u8], width: usize, height: usize, _base: usize, _glyph_width: usize, count: usize, first: char) -> Font {
		Font { first, count, width, height, glyphs }
	}

	/// Prints the given input to stdout using the font, representing pixels that are turned on with `#`.
	/// Note that this panics in case of error, so this should not be used for production.
	#[cfg(test)]
	pub fn print_text(&self, value: &str) {
		for y in 0..self.height {
			for ch in value.chars() {
				for x in 0..self.width {
					if self.get_pixel(ch, x, y).unwrap_or(false) {
						print!("#");
					} else {
						print!(" ");
					}
				}
			}
			println!();
		}
	}

	pub fn get_pixel(&self, ch: char, x: usize, y: usize) -> Result<bool> {
		if x >= self.width {
			bail!("x = {x} is out of bounds for {}", self.width);
		}
		if y >= self.height {
			bail!("y = {y} is out of bounds for {}", self.height);
		}

		let ch_pos = (ch as usize).checked_sub(self.first as usize)
			.ok_or_else(|| anyhow!("Char {ch} is out of bounds for {}", self.first))?;

		// the position in bits
		let pos = y * self.count * self.width + ch_pos * self.width + x;

		// the position to the byte and to the bit in that byte
		let byte_pos = pos / 8;
		let bit_pos = pos % 8;

		let byte = self.glyphs.get(byte_pos)
			.ok_or_else(|| anyhow!(
				"Position {byte_pos} (x = {x}, y = {y}, ch = {ch}, count = {}, width {}, height = {}) out of bounds for glyphs",
				self.count, self.width, self.height))?
			.clone();

		let flag = 1 << bit_pos;

		let is_set = byte & flag != 0;

		Ok(is_set)
	}
}



#[cfg(test)]
#[test]
fn test_fonts() -> Result<()> {
	let s = "Hello World!";
	Font::TINY.print_text(s);
	Font::SMALL.print_text(s);
	Font::MEDIUM.print_text(s);
	Font::LARGE.print_text("Hello!");
	return Ok(());
}