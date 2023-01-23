// Error handling test program 1
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = match File::open("text.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => File::create("text.txt").unwrap_or_else(|_| panic!("Create file error")),
            other_error => panic!("Unexpected error occurred: {:?}", other_error),
        },
    };
}
