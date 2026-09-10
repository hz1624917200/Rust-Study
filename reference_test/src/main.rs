use std::io;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    // println!("You entered: {}", input);
	let slice_1 = &input[0..=1];
	// let mut slice_2 = &mut input[1..=2];
	println!("The first 5 characters are: {}", slice_1);
	// println!("The second 5 characters are: {}", slice_2);

	// lifetime test
	let result;
	{
		// let slice_3 = &input[0..=2];			// slice of `input` has lifetime tied to `input`, which is dropped at the end of main function
		let slice_3 = "Hello, world!";		// literal string has 'static lifetime
		// let string_2 = String::from("Hello, world!");	// does not live long enough, because it is dropped at the end of this block
		// let slice_3 = &string_2[0..=4];
		result = longest(slice_1, slice_3);
	}
	println!("The longest string is: {}", result);
}