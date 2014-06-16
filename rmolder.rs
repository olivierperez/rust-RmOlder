#![crate_id = "rmolder#0.42"]

extern crate time;

use std::path::posix::Path;
use std::io::fs;

#[cfg(test)]
use std::io::fs::File;

/* --- Struct --- */
/// RmOlder is a tool which delete too old files into a directory.<br/>
/// Just specify a working directory and a maximum age, then let him do the thing.

pub struct RmOlder<'s> {
	dir: &'s Path,
	limit_age: u64
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
	
	/// Find all the files that match the given predicate
	fn find(&self, predicate: |path:&Path| -> bool) -> Vec<Path> {
		match fs::readdir(self.dir) {
			Ok(mut f) => {f.retain(predicate); f},
			Err(_) => vec!()
		}
	}
	
	/// Checks if a file is old enough to be deleted
	fn is_too_old(&self, file_age: u64) -> bool {
		let now = time::now().to_timespec().sec as u64;
		now + self.limit_age < file_age
	}
}

/* --- Test --- */

#[cfg(test)]
static TEST_DIR:&'static str = "./test";

#[cfg(test)]
fn create_file(filepath : &str) -> File {
	File::create(&Path::new(format!("{}/{}", TEST_DIR, filepath))).unwrap()
}

#[cfg(test)]
fn delete_file(path : &File) {
	fs::unlink(path.path());
}

#[test]
fn shoud_create_RmOlder() {
	let path = Path::new(TEST_DIR);
	let rmolder = RmOlder::new(&path, 42);
}

#[test]
fn should_list_files() {
	// Before
	let one = create_file("one");
	let two = create_file("two");
	
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
	let one = create_file("one");
	let two = create_file("two");
	
	// Given
	let path = Path::new(TEST_DIR);
	let rmolder = RmOlder::new(&path, 42);
	
	// When
	let files = rmolder.find(|path| path.filename_str().unwrap().contains("on"));
	
	// Then
	assert!(files.contains(one.path()));
	assert!(!files.contains(two.path()));
	
	// After
	delete_file(&one);
	delete_file(&two);
}

#[test]
fn should_compare_dates() {
	// Given
	let path = Path::new(TEST_DIR);
	let limit_age = 1000;
	let rmolder = RmOlder::new(&path, limit_age);
	
	let now = time::now().to_timespec().sec as u64;
	let too_old_age = now + 42000;
	let young_enough_age = now + 42;
	
	// When
	let too_old = rmolder.is_too_old(too_old_age);
	let young_enough = rmolder.is_too_old(young_enough_age);
	
	// Then
	assert!(too_old, "the 'too_old_age' must be too old");
	assert!(!young_enough, "now+42 is not too old");
}

