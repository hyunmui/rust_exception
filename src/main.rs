use std::{vec, fs::File};

fn main() {
    // panic!("crash and burn!");

    // let v = vec![1,2,3];
    // v[99];   // raise panic by out of index

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open file: {:?}", error);
        }
    };
}
