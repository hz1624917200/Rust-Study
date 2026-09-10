fn dangling_test() -> &String {
	let s = String::from("Hello world");
	&s
}

fn main() {
	let temp = dangling_test();
}

