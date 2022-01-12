mod args;
mod cli;
mod run;
mod cmds;

pub use anyhow::Context;
pub use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
	run::run(args::Arguments::from_args())?;
	Ok(())
}
