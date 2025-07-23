use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {}", query); 
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("should be able to read the file");
            println!("{:#?}", dbg!(contents));

            println!("{:#?}", dbg!(args));
    println!("Hello, world!");
    let path: &'static str = env!("PATH");
    println!("the $PATH variable at the time of compiling was: {path}");
}
