use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);
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

fn parse_config(args: &[String]) -> (&str,&str){
    let query = &args[1];
    let file_path = &args[2];
    (query,file_path)
}
