#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CommandType {
	None,
	Exit,
}

struct Command<'a> {
	ctype: CommandType,
	cstr:  &'a str,
}

const COMMAND_LIST: [Command; 2] = [
	Command {
		ctype: CommandType::None,
		cstr:  "",
	},
	Command {
		ctype: CommandType::Exit,
		cstr:  "!exit",
	},
];

pub fn command(cmd: &str) -> CommandType {
	for c in 1..COMMAND_LIST.len() {
		if cmd == COMMAND_LIST[c].cstr {
			return COMMAND_LIST[c].ctype;
		}
	}
	
	CommandType::None
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn command_exit() {
		assert_eq!(
			command("!exit"),
			CommandType::Exit,
		);
	}
	
	#[test]
	fn command_none() {
		assert_eq!(
			command("abc"),
			CommandType::None,
		);
	}
}