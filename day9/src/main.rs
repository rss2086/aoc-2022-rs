use std::{fs, collections::HashSet};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("file not found");
    let instructions = input.split("\n").into_iter().map(|f| f.split(" ").collect_vec()).collect_vec();
    // println!("Hello, world!");
    let mut head = (10000,10000);
    let mut tail1 = (10000,10000);
    let mut tail2 = (10000,10000);
    let mut tail3 = (10000,10000);
    let mut tail4 = (10000,10000);
    let mut tail5 = (10000,10000);
    let mut tail6 = (10000,10000);
    let mut tail7 = (10000,10000);
    let mut tail8 = (10000,10000);
    let mut tail9 = (10000,10000);

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
            update_tail(&mut tail1, &mut head);
            update_tail(&mut tail2, &mut tail1);
            update_tail(&mut tail3, &mut tail2);
            update_tail(&mut tail4, &mut tail3);
            update_tail(&mut tail5, &mut tail4);
            update_tail(&mut tail6, &mut tail5);
            update_tail(&mut tail7, &mut tail6);
            update_tail(&mut tail8, &mut tail7);
            update_tail(&mut tail9, &mut tail8);
            set.insert(format!("X{}Y{}",tail9.0,tail9.1));
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