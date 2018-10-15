// Module for processing command line arguments
use std::env;

/// Processor for parsing command line arguments
pub fn parse_args(args: env::Args) -> Vec<String> {
	
	let mut files: Vec<String> = Vec::new();

	for arg in args.skip(1) {
	    println!("{}\n", arg);
	    files.push(arg);
	}

	files
}