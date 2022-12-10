use std::{fs, io::Lines};


fn main() {
    let mut CURRENT_DIR = String::from("/");
    let input = fs::read_to_string("src/input.txt").expect("file not found");
    for line in input.lines() {
        println!("{}",is_command(line));
    }

}

fn is_command(line:&str) -> bool {
    return line.starts_with("$");
}