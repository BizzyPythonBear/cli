use crate::args;
use std::io::Error;
use crate::cmds::*;

pub fn match_cmds(args: args::Arguments) -> anyhow::Result<()> {
	let cmd = &args.action;
	let name = args.clone().name;
	let readonly = args.clone().readonly();
	let michael = args.clone().michael();
	match &*cmd.to_lowercase() {
		"simple" => simple(name, readonly)?,
		"fat" => fat(name, michael)?,
		_ => {
			return Err(anyhow::Error::new(Error::new(
				std::io::ErrorKind::InvalidInput,
				"Unkown Command",
			)))
		}
	}
	Ok(())
}
