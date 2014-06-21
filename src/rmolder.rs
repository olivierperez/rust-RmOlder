#![crate_id = "rmolder#0.42"]
#![crate_type = "rlib"]

extern crate getopts;
extern crate time;

use getopts::{optopt,optflag,getopts};
use std::io::fs;
use std::os;
use std::path::posix::Path;

#[cfg(test)]
use std::io::fs::File;

/* --- Struct --- */
/// RmOlder is a tool which delete too old files into a directory.<br/>
/// Just specify a working directory and a maximum age, then let him do the thing.
pub struct RmOlder<'s> {
	dir: &'s Path,
	limit_age: u64
}

/// Args helps to retrieve command line arguments for RmOlder applications.
pub struct Args<'s> {
	/// If true, should show usage of the application.
	pub help: bool,
	/// If true, should not really remove old files, but just show them.
	pub dry: bool,
	/// The age from where the file is reported as old.
	pub age: u64,
	/// The working directory.
	pub directory: Path,
}

/* --- Impl --- */

impl<'s> RmOlder<'s> {
	/// Create a new RmOlder with specifying the directory to work on.
	///
	/// ```rust{.example}
	/// let path = Path::new("./archives/");
	/// let rmolder = RmOlder::new(&path);
	/// ```
	pub fn new(scan_dir: &'s Path, max_age: u64) -> RmOlder<'s> {
		RmOlder {
			dir: scan_dir,
			limit_age: max_age
		}
	}
	
	/// Delete old files. Return the number of files deleted and the total count of files.
	///
	/// ```rust{.example}
	/// let path = Path::new("./archives/");
	/// let rmolder = RmOlder::new(&path);
	/// let (deleted_files, total) = rmolder.run();
	/// ```
	pub fn run(&self) -> (uint, uint) {
		self.inner_run(|path| {
			match fs::unlink(path) {_=>()};
		})
	}
	
	/// List old files, but do not do anything else.
	///
	/// ```rust{.example}
	/// let path = Path::new("./archives/");
	/// let rmolder = RmOlder::new(&path);
	/// let (old_files, total) = rmolder.run();
	/// ```
	pub fn dry_run(&self) -> (uint, uint) {
		self.inner_run(|path| {
			println!("File to delete: {}", path.as_str());
		})
	}
	
	fn inner_run(&self, visit: |p:&Path|->()) -> (uint, uint) {
		let mut count = 0;
		
		// List the old files
		let files = self.find(|path| {
			count += 1;
			self.is_too_old(path.stat().unwrap().modified / 1000)
		});
		
		// Do something on the old files
		for p in files.iter() {
			visit(p);
		}
		
		// Return the counts
		(files.len(), count)
	}
	
	/// Find all the files that match the given predicate
	fn find(&self, predicate: |path:&Path| -> bool) -> Vec<Path> {
		match fs::readdir(self.dir) {
			Ok(mut f) => {f.retain(predicate); f}, // TODO Replace with .iter().filter(...)
			Err(_) => vec!()
		}
	}
	
	/// Checks if a file is old enough to be deleted
	fn is_too_old(&self, file_age: u64) -> bool {
		let now = time::now().to_timespec().sec as u64;
		now - self.limit_age > file_age
	}
}

impl<'s> Args<'s> {
	/// Create a new Args with command line arguments.
	///
	/// ```rust{.example}
	/// let args = Args::new_from_args;
	/// ```
	pub fn new_from_args() -> Args {
		let args = os::args();
		
		let opts = [
			optopt("a", "age", "Set the age of files to remove", "(required)"),
			optopt("d", "directory", "Set the workding directory", ""),
			optflag("", "dry", "Dry run"),
			optflag("h", "help", "Print this menu")
		];
		
		let args = match getopts(args.tail(), opts) {
			Ok(m) => m,
			Err(f) => fail!(f.to_err_msg())
		};
		
		Args {
			help: args.opt_present("h"),
			dry: args.opt_present("dry"),
			age: Args::compute_age(args.opt_str("a")),
			directory: Args::compute_directory(args.opt_str("d"))
		}
	}
	
	fn compute_age(opt: Option<String>) -> u64 {
		match opt {
			Some(a) => 	match std::u64::parse_bytes(a.as_bytes(), 10) {
							Some(x) => x,
							_ => 0
						},
			None => fail!("The max age is not well defined!")
		}
	}
	
	fn compute_directory(opt: Option<String>) -> Path {
		match opt {
			Some(d) => Path::new(d),
			None => fail!("The working directory is not well defined!")
		}
	}
}
/* --- Test --- */

#[cfg(test)]
static TEST_DIR:&'static str = "./run-test";

#[cfg(test)]
fn create_file(filepath : &str) -> File {
	File::create(&Path::new(format!("{}/{}", TEST_DIR, filepath))).unwrap()
}

#[cfg(test)]
fn delete_file(path : &File) {
	match fs::unlink(path.path()) {_=>()};
}

#[test]
fn shoud_create_rmolder() {
	let path = Path::new(TEST_DIR);
	RmOlder::new(&path, 42);
}

#[test]
fn should_list_files() {
	// Before
	let one = create_file("should_list_files_one");
	let two = create_file("should_list_files_two");
	
	// Given
	let path = Path::new(TEST_DIR);
	let rmolder = RmOlder::new(&path, 42);
	
	// When
	let files = rmolder.find(|_| true);
	
	// Then
	assert!(files.contains(one.path()));
	assert!(files.contains(two.path()));
	
	// After
	delete_file(&one);
	delete_file(&two);
}

#[test]
fn should_list_files_with_filter() {
	// Before
	let one = create_file("should_list_files_with_filter_one");
	let two = create_file("should_list_files_with_filter_two");
	
	// Given
	let path = Path::new(TEST_DIR);
	let rmolder = RmOlder::new(&path, 42);
	
	// When
	let files = rmolder.find(|path| path.filename_str().unwrap().contains("on"));
	
	// Then
	assert!(files.contains(one.path()));
	//assert!(!files.contains(two.path()));
	
	// After
	delete_file(&one);
	delete_file(&two);
}

#[test]
fn should_compare_files_ages() {
	// Given
	let path = Path::new(TEST_DIR);
	let limit_age = 1000;
	let rmolder = RmOlder::new(&path, limit_age);
	
	let now = time::now().to_timespec().sec as u64;
	let too_old_age = now - 42000;
	let young_enough_age = now - 42;
	
	// When
	let too_old = rmolder.is_too_old(too_old_age);
	let young_enough = rmolder.is_too_old(young_enough_age);
	
	// Then
	assert!(too_old, "the 'too_old_age' must be too old");
	assert!(!young_enough, "now+42 is not too old");
}

