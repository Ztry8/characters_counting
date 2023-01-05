use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=3 { panic!("More or less than two arguments was given!") }

    let mut count = 0;
    let character = args[2].chars().next().unwrap();
    let content = fs::read_to_string(&args[1])
        .expect("Can't find file!");

    for i in content.chars() {
        if i==character||i==character.to_ascii_uppercase() { count+=1; }
    }

    println!("In this file {} \'{}\' characters.", count, character)
}
