// variable life circle test
use std::io;

fn main () {
	let a = 1;
	let mut buf = String::new();
	io::stdin().read_line(&mut buf)
		.expect("Reading error");
	let guess: u32 = buf.trim().parse().unwrap_or(0);
	if guess > 10 {
		let a = 10u32;
		println!("{}", a);
	}
	println!("{}", a);
}