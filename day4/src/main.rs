use std::fs;



fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap_or("Error Reading File".to_string());
    let lines:Vec<&str> = input.lines().into_iter().collect();

    let p1 = part_one(&lines);
    let p2 = part_two(&lines);
    // let mut test = lines.clone();
    // let lines = 3;
    println!("{:?} p2:{:?}",p1,p2)
}

fn part_one(inputs:&Vec<&str>) -> i32 {
    let mut sum = 0;
    for input in inputs {
        let ranges:Vec<&str> = input.split(",").collect();
        let first_range = ranges[0];
        let second_range = ranges[1];
        let vec_1:Vec<i32> = first_range.split("-").map(|f| f.parse().unwrap()).collect();
        let vec_2:Vec<i32> = second_range.split("-").map(|f| f.parse().unwrap()).collect();

        if(test_boundaries(vec_1,vec_2)) {
            sum = sum + 1;
        }
    }
    return sum;
}

fn part_two(inputs:&Vec<&str>) -> i32 {
    let mut sum = 0;
    for input in inputs {
        let ranges:Vec<&str> = input.split(",").collect();
        let first_range = ranges[0];
        let second_range = ranges[1];
        let vec_1:Vec<i32> = first_range.split("-").map(|f| f.parse().unwrap()).collect();
        let vec_2:Vec<i32> = second_range.split("-").map(|f| f.parse().unwrap()).collect();

        if(check_overlap(vec_1,vec_2)) {
            sum = sum + 1;
        }
    }
    return sum;
}

fn test_boundaries(vec1: Vec<i32>, vec2: Vec<i32>) -> bool {
    if(vec1[0] <= vec2[0] && vec1[1] >= vec2[1]) {
        return true;
    } else if (vec2[0] <= vec1[0] && vec2[1] >= vec1[1]) {
        return true;
    } else {
        return false;
    }
}

fn check_overlap(vec1: Vec<i32>, vec2: Vec<i32>) -> bool {
    if(vec1[1] < vec2[0] || vec2[1] < vec1[0]) {
        return false;
    } else {
        return true;
    }
}