// trait test 2
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
	let mut largest = list[0];

	for &item in list.iter() {
		if item > largest {
			largest = item;
		}
	}
	largest
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
	let mut largest = &list[0];
		
	for item in list.iter(){
		if item > largest {
			largest = item;
		}
	}
	largest
}

fn main() {
	let my_list_i32 = [1, 5, 3, 2, 0];
	let my_list_float = [1.0, 5.3, 2.5, 1.1 ,0f64];
	println!("i32 list max: {}", largest(&my_list_i32));
	println!("float list max: {}", largest(&my_list_float));
	println!("i32 list in ref: {}", largest_ref(&my_list_i32));
}