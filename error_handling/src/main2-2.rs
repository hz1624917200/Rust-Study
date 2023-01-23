use std::{io::{self, Read}, fs::File};

fn read_string() -> Result<String, io::Error> {
	let mut f = File::open("test.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}

fn main() {
	let s = read_string().unwrap_or_else(|err| panic!("IO error: {}", err));
	println!("{}", s);
}