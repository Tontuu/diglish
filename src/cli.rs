use clap::{arg, Command};

pub fn parse_arguments() -> Command {
    Command::new("git").
	about("Cambridge dictionary")
	.subcommand_required(false)
	.arg_required_else_help(false)
	.subcommand(
	    Command::new("clip")
		.about("Copy mainly meaning to clipboard using xclip")
		.arg_required_else_help(false)
	)
}
