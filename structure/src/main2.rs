#[derive(Debug)]
struct User {
	id: u64,
	name: String,
	email: String,
	active: bool,
}

fn main() {
	let user1 = User {
		id: 1u64,
		name: String::from("Zheng"),
		email: String::from("888888@outlook.com"),
		active: true,
	};
	println!("{:#?}", user1);
	let mut user2 = new_user(2u64, String::from("Huang"), String::from("123@qq.com"));
	let user3 = User {
		id: 3u64,
		..user2
	};
	println!("{:#?}", user3);
	println!("user2: {:p}; user3: {:p}", &user2.id, &user3);
}	

fn new_user(id: u64, name: String, email: String) -> User {
	User { 
		id, 
		name, 
		email, 
		active: true 
	}
}