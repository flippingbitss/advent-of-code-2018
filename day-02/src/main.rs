use std::io::Read;
use std::collections::{HashSet, HashMap};
use std::time::{Instant,Duration};

type PairStr<'a> = (&'a str, &'a str);

fn main() {
    let mut raw_input = String::new();
    std::io::stdin().read_to_string(&mut raw_input).unwrap();

    let input = raw_input.lines().collect::<Vec<_>>();
    let lines = input.as_slice();

    let sol_p1 = solve_part_1(&lines);
    let sol_p2 = solve_part_2_pairwise(&lines).unwrap();

    println!("{}", sol_p1);
    println!("{:?}", sol_p2);

    bench(&lines);
}

fn solve_part_1(lines: &[&str]) -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    for id in lines {
        let mut counter = HashMap::new();

        for c in id.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }

        if counter.values().any(|&x| x == 2) { twos += 1 };
        if counter.values().any(|&x| x == 3) { threes += 1 };
    }

    twos * threes
}

// O(n^2)
fn solve_part_2_pairwise<'a>(lines: &'a [&str]) -> Option<PairStr<'a>> {
    for (i, k1) in lines.iter().enumerate() {
        for k2 in lines.iter().skip(i + 1) {
            if k1
                .chars()
                .zip(k2.chars())
                .map(|(a,b)| (a != b) as i32)
                .sum::<i32>() == 1 {
                return Some((k1,k2));
            }
        }
    }
    None
}

// https://www.reddit.com/r/adventofcode/comments/a2damm/2018_day2_part_2_a_linear_time_solution/
// 0(k^2 * n) where k is length of id string
fn solve_part_2_near_linear<'a>(lines:  &'a [&str]) -> Option<PairStr<'a>> {
    let range = lines.first()?.len() - 1;
    let mut set = HashSet::new();

    for index in 0..range {
        for line in lines {
            let line = (&line[..index], &line[index + 1..]);
            if !set.insert(line) {
                return Some(line)
            }
        }
        set.clear();
    }
    None
}


fn bench(input: &[&str]){
    let num_iter: u32 = 1000;

    let mut durations = Vec::with_capacity(num_iter as usize);

    for _ in 0..num_iter {
        let start = Instant::now();

        let sol_p2_1 = solve_part_2_pairwise(&input).unwrap();
        let mid = start.elapsed();


        let sol_p2_2 = solve_part_2_near_linear(&input).unwrap();
        let end = start.elapsed() - mid;

        durations.push((mid,end));
    }

    let sol_1_dur = durations.iter().map(|&x| x.0).sum::<Duration>() / num_iter;
    let sol_2_dur = durations.iter().map(|&x| x.1).sum::<Duration>() / num_iter;

    println!("{:?}, {:?}", sol_1_dur, sol_2_dur)
}


//#[cfg(test)]
//mod tests {
//    use test::Bencher;
//
//    #[bench]
//    fn bench_pairwise(bencher: &mut Bencher) {
//
//    }
//}





