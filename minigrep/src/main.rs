use std::env;
use std::fs;

fn main() {
    
    let args:Vec<String> = env::args().collect();
    // dbg!(args);

    let query = &args[1];
    let path = &args[2];

    // println!("the string {}", query);
    println!("the file path {}", path);

    let content = fs::read_to_string(path).expect("should open the file");

    
    
    println!("the content \n{content}");
    // println!("Current dir: {:?}", std::env::current_dir().unwrap());
}
