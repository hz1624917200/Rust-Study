// rust reference and index test
fn main() {
	// let a = vec![1, 2, 3];
	let a = vec![String::from("123"),String::from("123"), String::from("123")];
	let a2 = &a[2];
	let b = "hello";
	let c = a.get(2);
	println!("{}", a[2]);
	println!("{}", a[2]);
}