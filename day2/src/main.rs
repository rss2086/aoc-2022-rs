use std::{fs, collections::HashMap} ;



fn main() {
    let map = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]
    );

    let result_map = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6),
    ]
    );  

    let mut total_score = 0;

    
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let results = input.lines();
    results.for_each(|result| {
        let split: Vec<&str> = result.split(" ").collect();

        // let pick_score = map.get(split[1]).ok_or(0).unwrap();
        let result_score = result_map.get(split[1]).ok_or(0).unwrap(); 
        let pick_score = calculate_pick_score(split);
        // let win_score = calculate_win_score(split);

        // println!("Matchup {:?}, Score: {}", result, pick_score + win_score);
        total_score = total_score + (pick_score + result_score);
    });

    println!("total score {}", total_score);

}

fn calculate_pick_score(scores:Vec<&str> ) -> i32 {

    let win_map = HashMap::from([
        ("A", "B"),
        ("B", "C"),
        ("C", "A"),
    ]
    );

    let lose_map = HashMap::from([
        ("A", "C"),
        ("B", "A"),
        ("C", "B"),
    ]
    );


    let map = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3),
    ]
    );


    let them = scores[0];
    let result = scores[1];

    if(result.eq("Y")) {
        return *map.get(them).ok_or(0).unwrap();
    } else if (result.eq("Z")) {
        let test = *map.get(win_map.get(them).ok_or(0).unwrap()).ok_or(0).unwrap();
       return test
    } else {
        return *map.get(lose_map.get(them).ok_or(0).unwrap()).ok_or(0).unwrap();
    }

}

fn calculate_win_score(scores:Vec<&str> ) -> i32 {
    let equals_map = HashMap::from([
        ("X", "A"),
        ("Y", "B"),
        ("Z", "C"),
    ]
    );

    let them = scores[0];
    let you = scores[1];
    let same_val = equals_map.get(&you).unwrap();
    
    let concat = [them,same_val].join("");

    let win_list = ["AB", "BC", "CA"];
    // let lose_list = ["BA", "CB", "AC"];

    
    if (them.eq(*same_val)) {
        return 3;
    }
    else if (win_list.contains(&concat.as_str())) {
        return 6;
    }
    else {
        return 0
    }

}
