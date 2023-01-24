pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// #[test]
// fn my_test() {
//     assert_eq!(2, 3);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
