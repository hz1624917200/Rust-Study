// vector iterator
fn main() {
	let mut a = vec![String::from("123"),String::from("123"), String::from("456")];
	// let b = a[0];

	for i in &mut a {
		i.push_str(" hello");
		println!("{}", i);
	}
	println!("{}", a[2]);
}