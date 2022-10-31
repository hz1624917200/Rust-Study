fn main() {
	assert_eq!(Some(6), plus_one(Some(5)));
	assert_eq!(None, plus_one(None));
	println!("Bingo!")
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}