use std::{fs, str::Chars};
fn main() {
    let binding = fs::read_to_string("src/input.txt").expect("File not found");
    let input = binding.split("\n").into_iter().collect::<Vec<&str>>();
    let mut vis_count = 0;
    let total_lines = input.len();
    let mut visible_trees: Vec<Vec<bool>> = vec![vec![false;total_lines];total_lines];
    visible_trees[0] = vec![true;total_lines];
    visible_trees[total_lines-1] = vec![true;total_lines];

    let heights = input.iter().map(|f| f.chars().map(|f| f.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();




    println!("ttl: {:?}", visible_trees);
    // let size = &input.clone().count();
    let mut count = 0;
    for line in &input {
        // The first and last tree are always visible.
        println!("count: {}", count);
        if count == 0 {
            vis_count += line.len();
        }
        else if count == total_lines - 1  {
            vis_count += line.len();
        }
        else {
            let mut highest:i32 = -1;
            for (char_count, char) in line.chars().enumerate() {
                if (char.to_digit(10).expect("test")) as i32 > highest  {
                    // println!("{} From left is visible: {}", char, highest);
                    visible_trees[count][char_count] = true;
                    vis_count +=1;
                    highest = char.to_digit(10).expect("test1") as i32;
                }
            }
            let mut highest:i32 = -1;
            for (char_count, char) in line.chars().rev().enumerate() {
                let len = line.chars().as_str().len();
                if (char.to_digit(10).expect("test")) as i32 > highest {
                    // println!("{} From right is visible: {}", char, highest);
                    visible_trees[count][len - (char_count+1)] = true;
                    vis_count +=1;
                    highest = char.to_digit(10).expect("test1") as i32;
                }
            }
        }
        count+=1;
    }

    let mut row_count = 0;

    while row_count < total_lines {
        let mut column_count = 0;
        let mut highest:i32 = -1;
        while column_count < total_lines {
            let h =  heights[column_count][row_count] as i32;
            if(h > highest) {
                visible_trees[column_count][row_count] = true;
                highest = h;
            }
            column_count +=1;
        }
        row_count+=1;
    }

    let mut row_count = 0;

    while row_count < total_lines {
        let mut column_count = 0;
        let mut highest:i32 = -1;
        while column_count < total_lines {
            let h =  heights[total_lines - (column_count + 1)][row_count] as i32;
            println!("Element: {} on row {}", h, column_count);
            if(h > highest) {
                println!("Highest element seen: {}", h);
                visible_trees[total_lines - (column_count + 1)][row_count] = true;
                highest = h;
            }
            column_count +=1;
        }
        row_count+=1;
    }

    let mut tree_count = 0;
    for element in visible_trees.iter_mut().flat_map(|r| r.iter_mut()) {
        if *element {
            tree_count +=1;
        }
    }

    println!("visible count: {}", tree_count);
    // println!("visible trees: {:?}", visible_trees);
}

struct Tree {
    visible:bool,
    height: i32
}

// fn check_visible(test: &mut Vec<Vec<bool>>) {
//     if()
// }
