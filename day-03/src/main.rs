extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io::Read;

type Map = HashMap<(i32, i32), i32>;

#[derive(Default)]
struct Rect {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl<'a> From<&'a Vec<i32>> for Rect {
    fn from(vec: &'a Vec<i32>) -> Self {
        match vec.as_slice() {
            &[id, x, y, w, h] => Rect { id, x, y, w, h },
            _ => Rect::default(),
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let digits_pattern = Regex::new(r"\d+").unwrap();

    let mut map = HashMap::new();
    let mut rects = Vec::new();

    for line in input.lines() {
        let fields = digits_pattern
            .find_iter(line)
            .map(|x| x.as_str().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let r = Rect::from(&fields);

        for x in r.x..(r.x + r.w) {
            for y in r.y..(r.y + r.h) {
                *map.entry((x, y)).or_default() += 1;
            }
        }

        rects.push(r);
    }

    let sol_p1 = solve_part_1(&map);
    let sol_p2 = solve_part_2(&map, &rects).unwrap();

    println!("{}", sol_p1);
    println!("{}", sol_p2);
}

fn solve_part_1(map: &Map) -> i32 {
    map.values()
        .fold(0, |acc, &c| acc + (c > 1) as i32)
}

fn solve_part_2(map: &Map, rects: &[Rect]) -> Option<i32> {
    'rect: for r in rects.iter() {
        for x in r.x..(r.x + r.w) {
            for y in r.y..(r.y + r.h) {
                if map[&(x, y)] != 1 {
                    continue 'rect;
                }
            }
        }
        return Some(r.id);
    }
    None
}


//impl Iterator for Rect {
//    type Item = (i32, i32);
//
//    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
//        for x in self.x..(self.x + self.w) {
//            for y in self.y..(self.y + self.h) {
//                return (x,y)
//            }
//        }
//    }
//}
