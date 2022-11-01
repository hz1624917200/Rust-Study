fn main() {
	let a = vec![1, 2, 3];
	for i in 0..4	{
		match a.get(i) {
			Some(x) => println!("a_{} = {}", i, x),
			None => {
				println!("out of range!");
				break;
			},
		}
	}
}