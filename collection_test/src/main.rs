// hash map update test
use std::collections::HashMap;

fn main() {
	let s = "Hello world, this is a test";
	let mut word_count = HashMap::new();
	for c in s.chars() {
		let count = word_count.entry(c).or_insert(0);
		*count += 1;
	}
	word_count.get_mut(&'a');
	// println!("{:?}", word_count)
	for (c, count) in word_count {
		println!("count of {} = {}", c, count);
	}
	// word_count.get_mut(&'b');
	
}