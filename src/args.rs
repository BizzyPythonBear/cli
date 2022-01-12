// args.rs - Arg Parsing
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq, Clone)]

pub struct Arguments {
	// Parse Commands
	pub action: String,
	// Args
	pub arguments: Vec<String>,
	// Name
	#[structopt(long, short)]
	pub name: String,
	// Readonly Mode
	#[structopt(long, short)]
	pub(crate) readonly: Option<Option<bool>>,

	// Michael
	#[structopt(long, short)]
	pub(crate) michael: Option<Option<bool>>,
}

impl Arguments {
	pub fn readonly(&self) -> bool {
		match self.readonly {
			None => false,
			Some(None) => true,
			Some(Some(a)) => a,
		}
	}

	pub fn michael(&self) -> bool {
		match self.michael {
			None => false,
			Some(None) => true,
			Some(Some(a)) => a,
		}
	}
}
