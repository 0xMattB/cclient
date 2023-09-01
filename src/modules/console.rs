use std::io::{self, Write, stdout};

use colored::*;

pub enum Color {
	Default,
	Red,
	Yellow,
	Green,
	Blue,
}

pub fn input() -> Result<String, &'static str> {
	let mut read = String::new();
	
	if let Ok(_) = io::stdin().read_line(&mut read) {
		Ok(read)
	} else {
		Err("console::input(): error reading from input")
	}
}

pub fn output(data: &str, newline: bool, color: Color) {
	match color {
		Color::Default => { print!("{}", data);          },
		Color::Red =>     { print!("{}", data.red());    },
		Color::Yellow =>  { print!("{}", data.yellow()); },
		Color::Green =>   { print!("{}", data.green());  },
		Color::Blue =>    { print!("{}", data.blue());   },
	}
	
	if newline {
		println!();
	} else {
		let _ = stdout().flush();
	}
}