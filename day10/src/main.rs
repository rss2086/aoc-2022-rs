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
    let mut crt = vec![String::from(""),String::from(""),String::from(""),String::from(""),String::from(""),String::from("")];
    let mut sprite_pixels = [0,1,2];
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
            println!("Sprites at {:?} for cycle {}", sprite_pixels, cycles);
            if sprite_pixels.contains(&(cycles%40)) {
                match cycles {
                    0..=40 => crt[0].push_str("#"),
                    41..=80 => crt[1].push_str("#"),
                    81..=120 => crt[2].push_str("#"),
                    121..=160 => crt[3].push_str("#"),
                    161..=200 => crt[4].push_str("#"),
                    201..=240 => crt[5].push_str("#"),
                    _ => (),
                };
            }
            else {
                match cycles {
                    0..=40 => crt[0].push_str("."),
                    41..=80 => crt[1].push_str("."),
                    81..=120 => crt[2].push_str("."),
                    121..=160 => crt[3].push_str("."),
                    161..=200 => crt[4].push_str("."),
                    201..=240 => crt[5].push_str("."),
                    _ => (),
                };
            }
            
            let item = q_vec.pop();
            match item {
                Some("pass") => println!("pass"),
                None => println!("None"),
                Some(_) => {
                    register += item.unwrap().parse::<i32>().expect("test");
                    sprite_pixels = [register, register+1, register+2]
                },
            }

        }

        println!("Register value {} at cycle {}", register, cycles)
    }

    println!("Register important values as {:?}", important_vec.iter().sum::<i32>());
    for line in crt {
        println!("{:?}", line);
    }

}
