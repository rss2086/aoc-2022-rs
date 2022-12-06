use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("File not found");

    let starting_char = 13;
    let mut window_start = 0;
    for count in starting_char..input.len() {
        let sub = &input[window_start..window_start+14];
        if(helper(sub)) {
            println!("found it at {}", window_start+14);
            break;
        }
        window_start = window_start + 1;
    }


}

fn helper(s: &str) -> bool {
    let mut set = HashSet::<char>::new();

    for char in s.chars() {
        set.insert(char);
    }
    // println!("substring {}",s);
    // println!("{}",set.len());
    if set.len() == 14 {
        return true
    } else {
        return false
    }


}
