fn main() {
	let s = String::from("Hello world");
	let mut a = [1, 2, 3];
	let b = &mut a[..];
	b[0] = 0;
	// a[0] = 3;	// error, mutable borrow twice
	println!("{}, {}", s, b[0]);
	
}