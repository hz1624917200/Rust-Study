// Generic types test
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let integer = Point {x: 5, y: 5};
    let float = Point {x: 5.0, y: 4.0};
    println!("{:#?}\n{:#?}", integer, float);
    println!("Point: ({}, {})", float.x(), float.y());
}