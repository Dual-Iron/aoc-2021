use std::{ops::RangeInclusive, collections::HashMap};
use regex::Regex;

#[derive(PartialEq, Eq, Hash)]
struct Point(i32, i32);

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

fn fix_range(range: RangeInclusive<i32>) -> RangeInclusive<i32> {
    if range.end() > range.start() {
        range
    } else {
        *range.end()..=*range.start()
    }
}

fn get_lines() -> Vec<Line> {
    let regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").expect("valid regex");
    let mut vec = Vec::with_capacity(256);

    for line in include_str!("../input/day5.txt").lines() {
        for capture in regex.captures(line) {
            vec.push(Line {
                x1: capture[1].parse().unwrap(),
                y1: capture[2].parse().unwrap(),
                x2: capture[3].parse().unwrap(),
                y2: capture[4].parse().unwrap()
            });
        }
    }

    vec
}

fn get_vent_points(lines: Vec<Line>, include_diagonal: bool) -> HashMap<Point, u32> {
    let mut vents = HashMap::<Point, u32>::new();

    for line in lines {
        if line.x1 == line.x2 {
            for y in fix_range(line.y1..=line.y2) {
                *vents.entry(Point(line.x1, y)).or_default() += 1;
            }
        } else if line.y1 == line.y2 {
            for x in fix_range(line.x1..=line.x2) {
                *vents.entry(Point(x, line.y1)).or_default() += 1;
            }
        } else if include_diagonal && (line.x1 - line.x2).abs() == (line.y1 - line.y2).abs() {
            let diff = (line.x1 as i32 - line.x2 as i32).abs();

            for i in 0..=diff {
                let p = Point(
                    line.x1 + i * (line.x2 - line.x1).signum(),
                    line.y1 + i * (line.y2 - line.y1).signum()
                );
                *vents.entry(p).or_default() += 1;
            }
        }
    }

    vents
}

fn do_part(part2: bool) {
    let lines = get_lines();
    let vents = get_vent_points(lines, part2);

    println!("{:?}", vents.iter().filter(|kvp| *kvp.1 > 1).count())
}

// 4655
pub fn part1() {
    do_part(false);
}

// 20500
pub fn part2() {
    do_part(true);
}
