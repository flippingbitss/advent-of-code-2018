use std::io::Read;

fn main() {
    // read input
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Error reading input");

    let solution = solve(&input);

    println!("{}", solution);
}

fn solve(input: &String) -> i32 {
    input.lines()
        .filter_map(|x| x.parse::<i32>().ok())
        .sum()
}
