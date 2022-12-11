use std::fs;

use itertools::Itertools;
use queues::{Queue, IsQueue, queue};

struct Monkey {
    inspect_count: i32,
    items: Queue<i32>,
    operation: fn(i32) -> i32, 
    test: fn(i32) -> bool,
    true_throw: i32,
    false_throw:i32,
}

impl Monkey {
    fn inspect(&mut self) ->(i32,i32) {
        let item = self.items.remove().unwrap();
        let new_worry = (self.operation)(item) / 3;
        self.inspect_count +=1;
        let mut check_res = 0;

        if((self.test)(item)) {
            check_res = self.true_throw;
        } else {
            check_res = self.false_throw;
        }
        return (new_worry,check_res);
    }

}
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("File not found");
    let monkey_rules = input.split("\n\n").map(|chunk| chunk.split("\n").map(|f| f.trim()).collect_vec()).collect_vec();

    let mut monkeys = Vec::<Monkey>::new();

    let mut m0 = Monkey{inspect_count:0,items:queue![79,98],operation: |x| x*19, test: |x| x%23 == 0,true_throw:2,false_throw:3};
    let mut m1 = Monkey{inspect_count:0,items:queue![54,65,75,74],operation: |x| x+6, test: |x| x%19 == 0,true_throw:2,false_throw:0};
    let mut m2 = Monkey{inspect_count:0,items:queue![79,60,97],operation: |x| x*x, test: |x| x%13 == 0,true_throw:1,false_throw:3};
    let mut m3 = Monkey{inspect_count:0,items:queue![74],operation: |x| x+3, test: |x| x%17 == 0,true_throw:0,false_throw:1};
    monkeys.push(m0);
    monkeys.push(m1);
    monkeys.push(m2);
    monkeys.push(m3);

    // for a in range

    // for rule in monkey_rules {
    //     println!("{:?}", rule);
    // }

    println!("Hello, world!");
}



// Phases
// 1 - Monkey pops first item from q
// 2 - Monkey inspects item raising worry by operation of monkey (inspection)
// 3 - Worry level gets divided by 3
// 4 - Monkey tests new worry level after division by 3 with test
// 5 - Result of test decides which monkey gets it.
// 6 - After all monkeys go, this is one round.
// 7 - Need to count inspection amounts across 20 rounds and multiply two highest together
