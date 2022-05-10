use crate::color::get_foreground_color;
use crate::config::{Colors, Options};
use crate::debug::{d, Dt};

pub fn get_letter_space(letter_space: &[String], options: &Options) -> Vec<String> {
	d("chars::get_letter_space()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"chars::get_letter_space()\nletter_space:{:?}\noptions.letter_spacing:{:?}",
			letter_space, options.letter_spacing
		),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let mut output = Vec::new();

	for letter_space_line in letter_space {
		let space = match letter_space_line.len() {
			0 => String::from(" ").repeat(options.letter_spacing as usize),
			_ => letter_space_line.repeat(options.letter_spacing as usize),
		};

		output.push(space);
	}

	d(&format!("chars::get_letter_space() -> {:?}", output), 2, Dt::Log, options, &mut std::io::stdout());
	output
}

pub fn add_line(output: &mut Vec<String>, font_lines: usize, options: &Options) -> Vec<String> {
	d("chars::add_line()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::add_line()\noutput:{:?}\nfont_lines:{:?}", output, font_lines),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	for _ in 0..font_lines {
		output.push(String::new());
	}

	d(&format!("chars::add_line() -> {:?}", output), 2, Dt::Log, options, &mut std::io::stdout());
	output.to_vec()
}

// pub fn add_letter(letter: &Vec<String>, output: &Vec<String>, options: &Options) -> Vec<String> {}

pub fn get_longest_line_len(output: &[String], font_lines: usize, options: &Options) -> usize {
	d("chars::get_longest_line_len()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::get_longest_line_len()\noutput:{:?}\nfont_lines:{:?}", output, font_lines),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let size = output.iter().rev().take(font_lines).fold(0, |acc, item| if item.len() > acc { item.len() } else { acc });

	d(&format!("chars::get_longest_line_len() -> {:?}", size), 2, Dt::Log, options, &mut std::io::stdout());
	size
}

pub fn get_letter_length(letter: &[String], font_color_count: usize, options: &Options) -> usize {
	d("chars::get_char_length()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::get_char_length()\nchar:{:?}\nfont_color_count:{:?}", letter, font_color_count),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let size = letter.iter().fold(0, |acc, item| {
		let mut stripped_item = item.clone();
		// first we remove color annotations
		// but fonts that support only a single color don't have color annotations
		if font_color_count > 1 {
			for i in 1..=font_color_count {
				let open = format!("<c{}>", i);
				let close = format!("</c{}>", i);
				stripped_item = stripped_item.replace(&open, "").replace(&close, "");
			}
		}

		if stripped_item.len() > acc {
			stripped_item.len()
		} else {
			acc
		}
	});

	d(&format!("chars::get_char_length() -> {:?}", size), 2, Dt::Log, options, &mut std::io::stdout());
	size
}

pub fn paint_letter(letter: &[String], colors: &[Colors], font_color_count: usize, options: &Options) -> Vec<String> {
	d("chars::paint_letter()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"chars::paint_letter()\nletter:{:?}\ncolors:{:?}\nfont_color_count:{:?}",
			letter, colors, font_color_count
		),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let painted_letter = letter
		.iter()
		.map(|line| {
			let mut new_line = line.clone();
			for i in 1..=font_color_count {
				let color_name = colors.get(i - 1).unwrap_or(&Colors::System);
				let (color_start, color_end) = match color_name {
					Colors::System => (String::from(""), String::from("")),
					color => get_foreground_color(color),
				};
				let open = format!("<c{}>", i);
				let close = format!("</c{}>", i);
				new_line = new_line.replace(&open, &color_start).replace(&close, &color_end);
			}
			new_line
		})
		.collect();

	d(&format!("chars::paint_letter() -> {:?}", painted_letter), 2, Dt::Log, options, &mut std::io::stdout());
	painted_letter
}

// pub fn align_last_line(output: &Vec<String>, font_lines: &usize, options: &Options) -> Vec<String> {}
