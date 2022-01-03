use std::{fs::{File, self}, vec, io::{self, Read}, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn!");

    // OutOfRange
    // let v = vec![1,2,3];
    // v[99];   // raise panic by out of range index

    // 파일 에러 처리
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) => match error.kind() {
    //         std::io::ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Failed to create a file: {:?}", e),
    //         },
    //         other_error => panic!("Failed to open a file: {:?}", other_error),
    //     }
    // };

    // 파일 에러 처리 with unwrap
    // let f = File::open("hello.txt").unwrap();

    // 파일 에러 처리 with except
    // let f = File::open("hello.txt").expect("Failed to open a file");
    let tmp = read_username_from_file();
    match tmp {
        Ok(value) => println!("{}", value),
        Err(error) => panic!("Failed to read file: {:?}", error),
    };

    let f = File::open("hello.txt")?;        // raise error with ? operator but main function has return type, you can write this.
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}