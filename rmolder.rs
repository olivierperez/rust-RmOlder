#![crate_id = "rmolder#0.42"]

use std::path::posix::Path;
#[cfg(test)]
use std::io::fs::File;
use std::io::fs;

/* --- Struct --- */

pub struct RmOlder<'s> {
	dir: &'s Path
}

/* --- Impl --- */

impl<'s> RmOlder<'s> {
	pub fn new(scan_dir: &'s Path) -> RmOlder<'s> {
		RmOlder {
			dir: scan_dir
		}
	}
	
	pub fn find(&self) -> Vec<Path> {
		match fs::readdir(self.dir) {
			Ok(f) => f,
			Err(_) => vec!()
		}
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
	let rmolder = RmOlder::new(&path);
}

#[test]
fn should_list_files() {
	// Before
	let one = create_file("one");
	let two = create_file("two");
	
	// Given
	let path = Path::new(TEST_DIR);
	let rmolder = RmOlder::new(&path);
	
	// When
	let files = rmolder.find();
	
	// Then
	assert!(files.contains(one.path()));
	assert!(files.contains(two.path()));
	
	// After
	delete_file(&one);
	delete_file(&two);
}

