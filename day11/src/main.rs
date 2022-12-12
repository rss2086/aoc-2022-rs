use std::fs;

use itertools::Itertools;
use queues::{Queue, IsQueue, queue};

use std::cell::RefCell;

struct Monkey {
    inspect_count: i32,
    items: Queue<i32>,
    operation: fn(i32) -> i32, 
    test: fn(i32) -> bool,
    true_throw: usize,
    false_throw:usize,
}

impl Monkey {
    fn inspect(&mut self) ->(i32,usize) {
        let item = self.items.remove().unwrap();
        // println!("Monkey checks item with worry level {}", item);
        let worry_op = (self.operation)(item);
        // println!("Monkey checks item with worry level {}", worry_op);
        let new_worry = worry_op / 3;
        // println!("Monkey gets bored, current level is {}", new_worry);
        self.inspect_count +=1;
        let mut check_res:usize = 0;

        if((self.test)(new_worry)) {
            check_res = self.true_throw;
        } else {
            check_res = self.false_throw;
        }
        // println!("Item thrown to Monkey {}", check_res);
        return (new_worry,check_res);
    }

}
fn main() {
    // let input = fs::read_to_string("src/input.txt").expect("File not found");
    // let monkey_rules = input.split("\n\n").map(|chunk| chunk.split("\n").map(|f| f.trim()).collect_vec()).collect_vec();

    let mut monkeys = Vec::<Monkey>::new();

    let mut m0 = Monkey{inspect_count:0,items:queue![89,74],operation: |x| x*5, test: |x| x%17 == 0,true_throw:4,false_throw:7};
    let mut m1 = Monkey{inspect_count:0,items:queue![75,69,87,57,84,90,66,50],operation: |x| x+3, test: |x| x%7 == 0,true_throw:3,false_throw:2};
    let mut m2 = Monkey{inspect_count:0,items:queue![55],operation: |x| x+7, test: |x| x%13 == 0,true_throw:0,false_throw:7};
    let mut m3 = Monkey{inspect_count:0,items:queue![69,82,69,56,68],operation: |x| x+5, test: |x| x%2 == 0,true_throw:0,false_throw:2};
    let mut m4 = Monkey{inspect_count:0,items:queue![72,97,50],operation: |x| x+2, test: |x| x%19 == 0,true_throw:6,false_throw:5};
    let mut m5 = Monkey{inspect_count:0,items:queue![90,84,56,92,91,91],operation: |x| x*19, test: |x| x%3 == 0,true_throw:6,false_throw:1};
    let mut m6 = Monkey{inspect_count:0,items:queue![63,93,55,53],operation: |x| x*x, test: |x| x%5 == 0,true_throw:3,false_throw:1};
    let mut m7 = Monkey{inspect_count:0,items:queue![50,61,52,58,86,68,97],operation: |x| x+4, test: |x| x%11 == 0,true_throw:5,false_throw:4};

    monkeys.push(m0);
    monkeys.push(m1);
    monkeys.push(m2);
    monkeys.push(m3);
    monkeys.push(m4);
    monkeys.push(m5);
    monkeys.push(m6);
    monkeys.push(m7);

    for rounds in 0..20 {
    for i in 0..(monkeys.len()) {
        // let mon = monkeys
        for j in 0..monkeys[i].items.size() {
        // println!("Working on monkey {}\n", i);
        let (worry, to_monkey) = monkeys[i].inspect();
        monkeys[to_monkey].items.add(worry).expect("should work");
        }
    }
}

    // monkeys.iter_mut().for_each(|f| {
    //     let (worry, to_monkey) = f.inspect();
    //     (worry,to_monkey)
    // });
    let mut inspect_counts = monkeys.iter().map(|f| f.inspect_count).collect_vec();
    inspect_counts.sort_by(|a, b| b.cmp(a)); 
    println!("{:?}", inspect_counts.iter().take(2).product::<i32>());

}



// Phases
// 1 - Monkey pops first item from q
// 2 - Monkey inspects item raising worry by operation of monkey (inspection)
// 3 - Worry level gets divided by 3
// 4 - Monkey tests new worry level after division by 3 with test
// 5 - Result of test decides which monkey gets it.
// 6 - After all monkeys go, this is one round.
// 7 - Need to count inspection amounts across 20 rounds and multiply two highest together
