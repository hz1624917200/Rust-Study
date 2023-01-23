use std::{error::Error, io::{self, Read}, fs::File};

fn read_string() -> Result<String, io::Error> {
	let mut s = String::new();
	File::open("text.txt")?.read_to_string(&mut s)?;
	Ok(s)
}


fn main() -> Result<(), Box<dyn Error>> {
	let s = read_string()?;
	println!("{}", s);
	Ok(())
}