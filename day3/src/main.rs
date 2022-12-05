use std::{collections::HashSet, fs};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap_or("Error Reading File".to_string());
    let priorities = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    part_two(input, priorities);
}

fn part_one(input: String, priorities: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let len = line.len();
        let first_slice = &line[..len / 2];
        let second_slice = &line[len / 2..];

        let mut first_characters: HashSet<char> = HashSet::new();
        for char in first_slice.chars() {
            first_characters.insert(char);
        }

        let mut second_characters: HashSet<char> = HashSet::new();
        for char in second_slice.chars() {
            second_characters.insert(char);
        }

        for char in second_characters {
            if (!first_characters.insert(char)) {
                println!("{}", priorities.find(char).unwrap_or(200));
                sum = sum + priorities.find(char).unwrap_or(200);
            }
        }
    }
    println!("{}", sum);
}

fn part_two(input: String, priorities: &str) {

    // let mut groups: Vec<Vec<&str>> = Vec::new();
    let mut  sum = 0;
    for (line1,line2,line3) in input.lines().tuples() {
        
        let mut first_characters: HashSet<char> = HashSet::new();
        for char in line1.chars() {
            first_characters.insert(char);
        }
        
        let mut second_characters: HashSet<char> = HashSet::new();
        for char in line2.chars() {
            second_characters.insert(char);
        }

        let mut third_characters: HashSet<char> = HashSet::new();
        for char in line3.chars() {
            third_characters.insert(char);
        }

        // let mut sets:Vec<_> = Vec::new();
        // sets.push(first_characters);
        // sets.push(second_characters);
        // sets.push(third_characters);
        let inter = first_characters.intersection(&second_characters).collect::<HashSet<_>>();
        let inter2 = second_characters.intersection(&third_characters).collect::<HashSet<_>>();

        let val = inter.intersection(&inter2);
        let char = val.into_iter().nth(0).unwrap();
        let number = priorities.find(**char).unwrap();
        sum = sum + number;

        println!("current {number} \n total {sum}");
    
    }

// let slice = ['l', 'o', 'r', 'e', 'm'];
// let iter = slice.chunks(2);
// println!("{:?}",iter)

}
