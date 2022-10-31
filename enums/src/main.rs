fn main() {
	let x = Some(3);
	if let Some(i) = x {
		println!("Num is {}", i)
	} else {
		println!("Value is None!")
	}
}