use std::{vec, fs::File};

fn main() {
    // panic!("crash and burn!");

    // let v = vec![1,2,3];
    // v[99];   // raise panic by out of index

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create a file: {:?}", e),
            },
            other_error => panic!("Failed to open a file: {:?}", other_error),
        }
    };
}
