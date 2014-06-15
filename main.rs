extern crate debug;
extern crate getopts;
extern crate time;

extern crate rmolder;

use getopts::{optopt,optflag,getopts,usage,short_usage};

use std::os;

use std::path::posix::Path;

use std::io;
use std::io::fs;

use rmolder::RmOlder;


fn main() {
	
	let args = os::args();
	let program = args.get(0).clone();
	
	let opts = [
		optopt("a", "age", "Set the age of files to remove", "(required)"),
		optopt("d", "directory", "Set the workding directory", ""),
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
	
	let age = computeOptAge(&opt_age.unwrap());
	let directory = Path::new(opt_directory.unwrap());
	//println!("age:  {}\ndirectory: {}", age, directory.as_str());
	
	let mut count_deleted : uint = 0;
	let mut count_total : uint = 0;
	
	let rmolder = RmOlder::new(&directory);
	
	visit_dir(&directory, |file| {
		println!("file: {}", file.as_str())
		count_total += 1;
		if is_too_old(file, age) {
			println!("File is too old");
			match fs::unlink(file) {
				Ok(_) => {count_deleted += 1;},
				_ => {}
			};
		}
	});
	
	println!("Files deleted : {}/{}", count_deleted, count_total);
}

fn computeOptAge(opt_age: &String) -> u64 {
	match std::u64::parse_bytes(opt_age.as_bytes(), 10) {
		Some(x) => x,
		_ => 0
	}
}

fn visit_dir(dir: &Path, visit: |&Path|) -> io::IoResult<()> {
	if dir.is_dir() {
		let files = try!(fs::readdir(dir));
		
		for file in files.iter() {
			if file.is_dir() {
				try!(visit_dir(file, |p| visit(p)));
			} else {
				visit(file);
			}
		}
		Ok(())
	} else {
		Err(io::standard_error(io::InvalidInput))
	}
}

fn is_too_old(file: &Path, age: u64) -> bool {
	let stat = match file.stat() {
		Ok(s) => s,
		_ => {return false;}
	};
	
	let now = time::now().to_timespec().sec as u64;
	
	//println!("modified date : {}", stat.modified / 1000);
	//println!("now date      : {}", now);
	let diff = now - (stat.modified / 1000);
	println!("File's age : {} secondes", diff);
	
	diff > age
}

