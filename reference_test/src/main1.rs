fn main() {
    let s1 = String::from("Hello");
    
    println!("length of string {} is {}", s1, calc_string_len(&s1));
}

fn calc_string_len(s: &String) -> usize {
    *s.len()
}