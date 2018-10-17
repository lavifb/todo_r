// Module for processing command line arguments
use std::env;

// TODO: store configuration in Config struct

/// Processor for parsing command line arguments
// TODO: add cli options
// TODO: add cli help
pub fn parse_args(args: env::Args) -> Vec<String> {
	let mut files: Vec<String> = Vec::new();

	for arg in args.skip(1) {
	    files.push(arg);
	}

	files
}