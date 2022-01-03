
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut s = String::new();
}