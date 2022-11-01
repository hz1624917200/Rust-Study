// tests for vector update while referenced
fn main() {
    let mut a = vec![1, 2, 3];
    a[2] = 4;
    let b = &mut a[2];
    a.pop();
    println!("{}", b);
    println!("{}", a[2]);
}
