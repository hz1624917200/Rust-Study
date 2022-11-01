// tests for vector update
fn main() {
    let mut a = vec![1, 2, 3];

    a.push(4);
    println!("pop a = {}", a.pop().unwrap_or(-1));
    println!("now, len(a) = {}", a.len());
}
