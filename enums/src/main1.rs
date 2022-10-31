enum MyIpAddr {
	V4,
	V6,
}

enum Message {
	Quit,
	Move {x: i32, y: i32},
	Write(String),
	ChangeColor(i32, i32, i32, i32),
}

fn main() {
    let mode = 'q';
	
	match mode {
		'w' => todo!(),
		'r' => todo!(),
		'q' => return,
		_ => {println!("Input error!");},
	};
}
