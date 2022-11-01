// hash map intro
use std::collections::HashMap;

fn main() {
	let mut scores = HashMap::new();
	
	scores.insert("B", 10);
	scores.insert("R", 50);
	let a = scores.entry("B").or_insert(1);
	*a +=1;
	println!("{}", scores.get("G").unwrap_or(&1));
	// scores.try_insert((key), value)
}