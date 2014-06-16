extern crate debug;
extern crate getopts;
extern crate time;

extern crate rmolder;

use getopts::{optopt,optflag,getopts,usage,short_usage};

use std::os;

use std::path::posix::Path;

use rmolder::RmOlder;


fn main() {
	
	let args = os::args();
	let program = args.get(0).clone();
	
	let opts = [
		optopt("a", "age", "Set the age of files to remove", "(required)"),
		optopt("d", "directory", "Set the workding directory", ""),
		optflag("", "dry", "Dry run"),
		optflag("h", "help", "Print this menu")
	];
	
	let matches = match getopts(args.tail(), opts) {
		Ok(m) => m,
		Err(f) => fail!(f.to_err_msg())
	};
	
	if matches.opt_present("h") {
		println!("{}", short_usage(program.as_slice(), opts));
		println!("{}", usage("", opts));
		return;
	}
	
	if !matches.opt_present("a") {
		println!("Option 'a' is required");
		println!("{}", short_usage(program.as_slice(), opts));
		println!("{}", usage("", opts));
		return;
	}
	
	let opt_directory = matches.opt_str("d");
	let opt_age = matches.opt_str("a");
	
	//println!("opt_age:  {}\nopt_directory: {}", opt_age, opt_directory);
	
	let age = compute_opt_age(&opt_age.unwrap());
	let directory = Path::new(opt_directory.unwrap());
	//println!("age:  {}\ndirectory: {}", age, directory.as_str());
	
	let rmolder = RmOlder::new(&directory, age);
	
	// Select dry-run or not
	let (count_deleted, count_total) = match matches.opt_present("dry") {
		true => rmolder.dry_run(),
		false => rmolder.run()
	};
	
	println!("Files deleted : {}/{}", count_deleted, count_total);
}

fn compute_opt_age(opt_age: &String) -> u64 {
	match std::u64::parse_bytes(opt_age.as_bytes(), 10) {
		Some(x) => x,
		_ => 0
	}
}

