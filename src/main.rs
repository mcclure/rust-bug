use ansi_term::Style;
use ansi_term::Colour::Fixed;
use std::cmp::max;
use itertools::{EitherOrBoth, Itertools};

const STRING1: &str = "[
    1,
    [
        2,
        [
            3,
            [
                4,
                [
                    5,
                    6,
                    7]]]],
    8,
    9]";

const STRING2: &str = STRING1;

fn main() {
	// let _enabled = ansi_term::enable_ansi_support();
	let gray1 = Style::new().on(Fixed(236));
	let gray2 = Style::new().on(Fixed(237));

	fn str_width(s:&str) -> usize { // Width of longest line in s
		let mut x = 0;
		for line in s.lines() { x = max(x, line.len()); }
		x
	}

	let width1 = str_width(&STRING1);
	let width2 = str_width(&STRING2);

	for pass in 1..6 {
		println!("Repetition {}", pass);
		for x in STRING1.lines().zip_longest(STRING2.lines()) {
			let (string1, string2) = match x {
				EitherOrBoth::Both(string1, string2) => (string1, string2),
				EitherOrBoth::Left(string1) => (string1, ""),
				EitherOrBoth::Right(string2) => ("", string2)
			};
			println!("{}{}{}{}", gray1.paint(string1), gray1.paint(" ".repeat(width1 - string1.len())),
			                                gray2.paint(string2), gray2.paint(" ".repeat(width2 - string2.len())));
		}
		println!("{}{}\n", gray1.paint(" ".repeat(width1)), gray2.paint(" ".repeat(width2)));
	}
}
