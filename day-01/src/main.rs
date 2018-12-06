use std::collections::HashSet;
use std::io::Read;

fn main() {
    // read input
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Error reading input");

    // solutions
    let sol_p1 = solve_part_1(&input);
    let sol_p2 = solve_part_2(&input);

    println!("{}", sol_p1);
    println!("{}", sol_p2);
}


fn solve_part_1(input: &String) -> i32 {
    input.lines()
        .filter_map(|x| x.parse::<i32>().ok())
        .sum()
}


fn solve_part_2(input: &String) -> i32 {
    let mut sum = 0;
    let mut seen = HashSet::new();

    for line in input.lines().cycle() {
        let curr = line.parse::<i32>().unwrap();
        sum += curr;

        if seen.contains(&sum) {
            return sum;
        }

        seen.insert(sum);
    }
    sum
}
