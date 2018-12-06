use std::collections::HashMap;
use std::io::Read;

fn main() {
    // read input
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Error reading input");


    let solution = solve(&input);
    println!("{}", solution);
}


fn solve(input: &str) -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    for id in input.lines() {
        let mut counter = HashMap::new();

        for c in id.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }

        if counter.values().find(|&&x| x == 2).is_some() { twos += 1 };
        if counter.values().find(|&&x| x == 3).is_some() { threes += 1 };
    }

    twos * threes
}