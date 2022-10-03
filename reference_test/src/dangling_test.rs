fn dangling_test() -> &String {
	let s = String::from("Hello world");
	&s
}