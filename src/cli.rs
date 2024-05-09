use clap::{
	Args,
	Parser,
	Subcommand,
};

#[derive(Debug, Parser)]
pub struct Cli {
	#[arg(default_value_t = true)]
	pub once: bool,

	/// Percentage step when increasing/decreasing volume
	/// Only unsigned 8-bit ints are accepted
	#[arg(short = 's', long = "step")]
	pub step_size: Option<u8>,
}
