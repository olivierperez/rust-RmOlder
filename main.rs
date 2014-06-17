extern crate debug;
extern crate getopts;
extern crate time;

extern crate rmolder;

use getopts::{optopt,optflag,usage,short_usage};

use std::os;

use rmolder::Args;
use rmolder::RmOlder;

fn main() {
	// Retrive the command line arguments
	let args = Args::new_from_args();
	
	if args.help {
		show_usage();
		return;
	}
	
	// Build RmOlder
	let rmolder = RmOlder::new(&args.directory, args.age);
	
	// Run or Dry-run
	let (count_deleted, count_total) = match args.dry {
		true => rmolder.dry_run(),
		false => rmolder.run()
	};
	
	// Display the result
	println!("Files deleted : {}/{}", count_deleted, count_total);
}

fn show_usage() {
	let osargs = os::args();
	let program = osargs.get(0).clone();
	let opts = [
		optopt("a", "age", "Set the age of files to remove", "(required)"),
		optopt("d", "directory", "Set the workding directory", ""),
		optflag("", "dry", "Dry run"),
		optflag("h", "help", "Print this menu")
	];
	println!("{}", short_usage(program.as_slice(), opts));
	println!("{}", usage("", opts));
}

