use std::{fs, collections::HashSet};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("file not found");
    let instructions = input.split("\n").into_iter().map(|f| f.split(" ").collect_vec()).collect_vec();
    // println!("Hello, world!");
    let mut head = (10000,10000);
    let mut tail = (10000,10000);
    let mut set = HashSet::<String>::new();


    for instruction in instructions {
        let direction = instruction[0];
        let amount:i32 = instruction[1].parse().expect("mess");
    
        for i in 0..amount {
            match direction {
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                _ => println!("not sure")
            }
            println!("Updated head to {:?}", head);
            update_tail(&mut tail, &mut head);
            set.insert(format!("X{}Y{}",tail.0,tail.1));
        }
        // update_head(instruction, &mut head, &mut tail);
        // println!("{:?}",instruction);
    }

    println!("Set{:?}", set);
    println!("Set Length: {}", set.len());

}

// fn update_head(instruction:Vec<&str>,head:&mut (i32,i32), tail: &mut (i32,i32)) {
//     let direction = instruction[0];
//     let amount:i32 = instruction[1].parse().expect("mess");

//     for i in 0..amount {
//         match direction {
//             "R" => head.0 += 1,
//             "L" => head.0 -= 1,
//             "U" => head.1 += 1,
//             "D" => head.1 -= 1,
//             _ => println!("not sure")
//         }
//         println!("Updated head to {:?}", head);
//         update_tail(tail, head);

//     }

// }

fn update_tail(tail: &mut (i32,i32),head:&(i32,i32)) {
    if (head.0 - tail.0).abs() < 2 && (head.1 - tail.1).abs() < 2 {
        return;
    }
    else if (head.0 - tail.0).abs() >=2 && (head.1 - tail.1) == 0 {
        if(head.0 > tail.0) {
            tail.0 +=1;
        }
        else {
            tail.0 -= 1;
        }
    }
    else if (head.1 - tail.1).abs() >=2 && (head.0 - tail.0) == 0 {
        if(head.1 > tail.1) {
            tail.1 +=1;
        }
        else {
            tail.1 -= 1;
        }
    }
    else {
        if(head.0 > tail.0) {
            tail.0 +=1;
        }
        else {
            tail.0 -=1;
        }
        if(head.1 > tail.1) {
            tail.1 += 1;
        } else {
            tail.1 -=1;
        }
    }

    println!("Updated tail to {:?}", tail);
}