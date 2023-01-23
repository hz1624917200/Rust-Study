use std::{io::{self, Read}, fs::File};

fn read_string() -> Result<String, io::Error> {
	let f = File::open("text.txt");

	let mut f = match f {
		Ok(f) => f,
		Err(e) => return Err(e),
	};

	let mut s = String::new();
	
	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

fn main () {
	let s = read_string().unwrap_or_else(|err| panic!("IO error: {:?}", err));
	println!("{}", s);
}