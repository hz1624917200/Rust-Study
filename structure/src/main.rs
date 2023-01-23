#[derive(Debug)]
#[derive(Default)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

fn main() {
	let rect_default: Rectangle = Default::default();
	print!("{:#?}", rect_default);
	let rect1 = Rectangle {
		width: 30,
		height: 20,
	};
	println!("Size of {:?} is {}", rect1, rect1.area());
}