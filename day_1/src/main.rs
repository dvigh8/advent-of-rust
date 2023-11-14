use std::env;
use std::fs;

fn main() {
    
    let contents = fs::read_to_string("day_1_input.txt").expect("Where's my file?");

    println!("This is the text \n{contents}");
}
