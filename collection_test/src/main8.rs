// string iterator test
fn main() {
	let a = "Hello, 你好".to_string();
	// let b =  " world".to_string();
	for i in a.chars() {
		println!("{}", i);
	}
	for i in a.bytes() {
		println!("{}", i);
	}
}