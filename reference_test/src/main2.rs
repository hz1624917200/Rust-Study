fn main() {
	let mut s = String::from("Hello");
	add_string(&mut s);
	s.push_str("string");
	println!("String is {}", s);
}

fn add_string(s: &mut String) {
	s.push_str(", world. ");
}