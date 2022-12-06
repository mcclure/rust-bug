// Simulate a crane robot based on a drawing and a series of instructions.
// This is adapted from Advent of Code 2022 day 5.
// It contains three bugs, each individually simple, which together result in the outre bug
// "cyclic type of infinite size"
// Search within this file for "XXX" for comments explaining how this went wrong. 

use std::io::{BufRead, BufReader, Error, ErrorKind, Stdin, stdin};
use std::fs::File;
use either::Either;

use regex::Regex;

// XXX Notice this program returns a result, and that the potential error type is std::io::Error.
fn main() -> Result<(), Error> {
    // Load file from command-line argument or (if none) stdin
	let filename = std::env::args().fuse().nth(1);
	let input: Either<BufReader<Stdin>, BufReader<File>> = match &filename {
		None => either::Left(BufReader::new(stdin())),
		Some(x) => either::Right(BufReader::new(std::fs::File::open(x)?))
	};

	let mut lines = input.lines();

	// XXX: I don't want to write out my error messages inline, so I made a few shorthand functions.
	// XXX: Out of laziness, I used closures, so I wouldn't have to write types.
	// XXX: A couple of these come in "e" and non-"e" forms.
	// XXX: The "e" form returns an Error and the non-"e" form returns a Result (an Err(Error)).
	// XXX: I did not plan it this way on purpose. Had I planned it I might have done this differently.
	let invalid =   || { Err(Error::new(ErrorKind::InvalidInput, "Did not find expected ascii art diagram")) };
	let invalide2 = || { Error::new(ErrorKind::InvalidInput, "Expected sentence like 'move x from y to z'") };
	let invalid2 =  || { Err(invalide2()) };
	let invalide3 = || { Error::new(ErrorKind::InvalidInput, "Out of crates") };

	// Series of either three spaces or [W], separated by spaces. Will capture W or S (for Word or Space)
	let separator_re = Regex::new(r"^\p{gc:Zs}").unwrap();
	let blank_re = Regex::new(r"^\p{gc:Zs}{3}").unwrap();
	let crate_re = Regex::new(r"^\[(\w)\]").unwrap();
	let numbers_re = Regex::new(r"^[\s\d]+$").unwrap();
	let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

	// Returns rest of string after match
	fn match_next<'a>(m:regex::Captures, s:&'a str) -> &'a str {
		return &s[m.get(0).unwrap().end()..]
	}

	// Returns first match group, rest of string after match
	fn match_next_get<'a, 'b>(m:regex::Captures<'a>, s:&'b str) -> (&'a str, &'b str) {
		return (m.get(1).unwrap().as_str(), match_next(m, s))
	}

	// Assumes b != c
	fn index_two<T>(a:& mut[T], b:usize, c:usize) -> (&mut T, &mut T) {
		let ordered = b < c;
		let (low_idx, high_idx) = if ordered { (b,c) } else { (c,b) };
		let (low_slice, high_slice) = a.split_at_mut(high_idx);
		let (low, high) = (&mut low_slice[low_idx], &mut high_slice[0]);
		return if ordered { (low, high) } else { (high, low) }
	}

	let mut data:Vec<Vec<char>> = Vec::new();

	// Scan file
	for line in lines.by_ref() {
		let line = line?;
		let mut rest = line.as_str();
		println!("Line");

		// Note: Moves to phase 2 on first empty line, ignores number "comment"
		// Does NOT check accuracy of number "comment"
		if rest.is_empty() { break }
		if let Some(_) = numbers_re.captures(rest) { continue }

		let mut column = 0;
		loop {
			// Blank space before crate
			if column > 0 {
				if let Some(capture) = separator_re.captures(rest) {
					rest = match_next(capture, rest)
				} else {
					break // End of string
				}
			}
			// No crate
			if let Some(capture) = blank_re.captures(rest) {
				rest = match_next(capture, rest);
			// Crate
			} else if let Some(capture) = crate_re.captures(rest) {
				let tag:&str;
				(tag, rest) = match_next_get(capture, rest);
				while data.len() <= column
					{ data.push(Vec::new()) }
				let tag_ch = tag.chars().next().unwrap();
				data[column].push(tag_ch);
				println!("Column {} tag {}", column, tag_ch);
			} else {
				return invalid();
			}
			column += 1
		}
	}

	if data.len() == 0 { return invalid() }

	// Reverse all columns of data
	// Note column not of same type as before
	for column in data.iter_mut() {
		column.reverse()
	}

	for line in lines {
		let line = line?;
		println!("Command: {} On: {:?}", line, data);

		if let Some(capture) = move_re.captures(&line) {
			let v = capture.iter().skip(1)
				.map(|x| match x {
					// XXX: Bug 1: This should be Err(invalide2()).
					// XXX: Notice Err(invalide2()) is the *entire contents of invalid2()*,
					// XXX: So the correct code is basically inlining invalid2(). However,
					// XXX: inlining it makes a big difference because *the T in Result<T, E>
					// XXX: is different in this position than where invalid2() is supposed to be called.*
					None => invalid2(),
					// XXX This line contains two bugs.
					// XXX Bug 2: map() should be map_err().
					// XXX Bug 3: invalid2() should be invalide2().
					Some(x) => x.as_str().parse::<usize>().map(|_|invalid2())
				}).collect::<Result<Vec<usize>, Error>>()?;

			// XXX So, what just happened?
			// XXX Because these are the first uses of invalid2() in the file, invalid2's type
			// XXX is set to whatever it is inferred to be in this map() closure.
			// XXX However, invalid2() is used in a contradictory way in this closure
			// XXX (because I mixed up Error and Err(Error)).
			// XXX Although this is contrived, I think this is worth addressing because
			// XXX 1. Mixing up Error and Err(Error) is SUCH AN EASY MISTAKE TO MAKE.
			// XXX 2. I didn't assign invalid2() an infinite type. Rust did that.
			// XXX    Rust *inferred an impossible type* it invented itself,
			// XXX    then complained the type it invented was impossible.

			let [a,b,c] = <[usize; 3]>::try_from(v).ok().unwrap();

			if b == 0 || c == 0 { return invalid2() }
			if b != c {
				let (column_from, column_to) = index_two(&mut data, b-1, c-1);

				let column_from_n = column_from.len();
				let column_from_post_n = column_from_n-a;
				column_to.extend_from_slice(&column_from[column_from_post_n..column_from_n]);
				column_from.truncate(column_from_post_n);
			}
		} else {
			return invalid2()
		}
	}

	// Debug, print entire tree
	println!("Final: {:?}", data);

	// Result code
	for column in data {
		print!("{}", column.last().unwrap_or(&' '));
	}
	println!("");

	Ok(())
}
