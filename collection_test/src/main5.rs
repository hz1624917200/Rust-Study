// test element assignment and modify
fn main() {
	// let a = vec![1, 2, 3];
	let mut a = vec![String::from("123"),String::from("123"), String::from("123")];
	a[2] = String::from("456");
	a[2].push_str(" string");
	println!("{}", a[2]);
}