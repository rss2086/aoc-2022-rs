use std::{fs, collections::HashMap};

use itertools::Itertools;



fn main() {
    let instruction_times = HashMap::from([("addx",2),("noop",1)]);

    let input = fs::read_to_string("src/input.txt").expect("file not found");
    let instructions = input.split("\n").into_iter().map(|f| f.split(" ").collect_vec()).collect_vec();
    let mut cycles = 0;
    let mut register = 1;
    let mut important_vec:Vec<i32> = vec![];
    let mut q_vec:Vec<&str> = vec![];
    for instruction in instructions {
        match instruction[0] {
            "addx" => {
                q_vec.push(instruction[1]);
                q_vec.push("pass");
            } ,
            "noop" => {
                q_vec.push("pass");
            },
            _ => println!("Something else found"),
        }

        while q_vec.len() != 0 {
            cycles+=1;
            match cycles {
                20 => important_vec.push(20 * register),
                60 => important_vec.push(60 * register),
                100 => important_vec.push(100 * register),
                140 => important_vec.push(140 * register),
                180 => important_vec.push(180 * register),
                220 => important_vec.push(220 * register),
                _ => ()
            }
            let item = q_vec.pop();
            match item {
                Some("pass") => println!("pass"),
                None => println!("None"),
                Some(_) => register += item.unwrap().parse::<i32>().expect("test"),
            }

        }

        println!("Register value {} at cycle {}", register, cycles)
    }

    println!("Register important values as {:?}", important_vec.iter().sum::<i32>())
    // println!()

}
