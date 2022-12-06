use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let puzzle = input.split("\n\n").collect::<Vec<&str>>();

    let crates_input = puzzle[0].lines();

    let size = 3;
    
    let mut arr = [&mut Vec::<char>::new(),&mut Vec::<char>::new(),&mut Vec::<char>::new(),&mut Vec::<char>::new(),&mut Vec::<char>::new(),&mut Vec::<char>::new(),&mut Vec::<char>::new(),&mut Vec::<char>::new(),&mut Vec::<char>::new()];

    for (count_arr,line) in crates_input.enumerate() {

        if count_arr == 8 {
            break;
        }
        
        let mut count = 0;
        let mut vec = Vec::<char>::new();
        for chunk in line.char_indices(){
            if((chunk.0 + 1) % 4 == 0 && chunk.0 != 0) {
                continue;
            }
            vec.push(chunk.1);

        }

        // let reversed = vec.into_iter().rev().collect::<Vec<char>>();
        let mut local_count = 0;
        for chunk in vec.chunks(3) {
            arr[local_count].push(chunk[1]);
            local_count +=1;
        }

    }

    let mut stacks = arr.map(|f| f.iter().rev().filter(|f| **f != ' ').collect::<Vec<&char>>());

    let instructions = puzzle[1].lines().collect::<Vec<&str>>();

    for instruction in instructions {
        follow_instructions_2(&mut stacks, instruction);
    }


    println!("Final {:?}",stacks);
    let mut c: char = '\0';
    let mut answer = String::new();
    for stack in stacks {
        answer.push(**stack.last().unwrap_or(&&c));
    } 
    println!("Final 2 {}", answer);


}

fn follow_instructions(stacks: &mut [Vec<&char>] , instruction: &str) {
    println!("{:?}", stacks);
    let test = instruction.split_whitespace().filter(|s| s.parse::<i32>().is_ok() ).map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let (count, src,dest) = (test[0],test[1],test[2]);

    for n in 0..count {
        let item = stacks[src-1].pop().unwrap();
        stacks[dest-1].push(item);
    }
}

fn follow_instructions_2(stacks: &mut [Vec<&char>] , instruction: &str) {

    let test = instruction.split_whitespace().filter(|s| s.parse::<i32>().is_ok() ).map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let (count, src,dest) = (test[0],test[1],test[2]);

    let mut vec:Vec<&char> = Vec::new();

    for n in 0..count {
        let item = stacks[src-1].pop().unwrap();
        vec.push(item);
    }

    let vec2 = vec.into_iter().rev().collect::<Vec<&char>>();

    for c in vec2 {
        stacks[dest-1].push(&c);
    }

}

