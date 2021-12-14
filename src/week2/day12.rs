#![feature(let_chains)]

use std::collections::HashMap;

type Map = HashMap<&'static str, Vec<&'static str>>;
type Path = Vec<&'static str>;

// This looks an awful lot like doing permutations of a list.
// Because of that, this came in handy! https://github.com/Dual-Iron/wykraken/blob/master/src/main.rs

fn map() -> Map {
    let mut ret = HashMap::new();
    for line in crate::input!(12).lines() {
        let mut split = line.split('-');
        let from = split.next().unwrap();
        let to = split.next().unwrap();

        ret.entry(from)
            .and_modify(|v: &mut Vec<&str>| v.push(to))
            .or_insert_with(|| vec![to]);
        ret.entry(to)
            .and_modify(|v: &mut Vec<&str>| v.push(from))
            .or_insert_with(|| vec![from]);
    }
    ret
}

fn is_small(input: &str) -> bool {
    input.chars().next().unwrap().is_lowercase()
}

fn pathfind(paths: &mut Vec<Path>, stack: &mut Path, map: &Map) {
    let current = stack.last().unwrap();

    if current == &"end" {
        paths.push(stack.to_vec());
        return;
    }

    for next in &map[current] {
        if !(is_small(next) && stack.contains(next)) {
            stack.push(next);
            pathfind(paths, stack, map);
            stack.pop();
        }
    }
}

// 3497
pub fn part1() {
    let map = map();
    let mut paths = Vec::new();
    let mut stack = vec!["start"];

    pathfind(&mut paths, &mut stack, &map);

    println!("{}", paths.len());
}

fn pathfind_part2(paths: &mut Vec<Path>, stack: &mut Path, map: &Map) {
    let current = stack.last().unwrap();

    if current == &"end" {
        paths.push(stack.to_vec());
        return;
    }

    let extra_cave_exists = stack
        .iter()
        .any(|s| is_small(s) && stack.iter().filter(|s2| s2 == &s).count() > 1);

    for next in &map[current] {
        if !(is_small(next) && stack.contains(next) && (next == &"start" || extra_cave_exists)) {
            stack.push(next);
            pathfind_part2(paths, stack, map);
            stack.pop();
        }
    }
}

// 93686
pub fn part2() {
    let map = map();
    let mut paths = Vec::new();
    let mut stack = vec!["start"];

    pathfind_part2(&mut paths, &mut stack, &map);

    println!("{}", paths.len());
}
