use std::collections::HashSet;

#[derive(Debug)]
struct Input {
    points: HashSet<(i32, i32)>,
    folds: Vec<(bool, i32)>,
}

fn parse_input() -> Input {
    let mut points = HashSet::new();
    let mut folds = Vec::new();
    for line in crate::input!(13).lines().filter(|l| !l.is_empty()) {
        if !line.starts_with('f') {
            let mut split = line.split(',');
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();

            points.insert((x, y));
        } else {
            let mut split = line.split('=');
            let x = split.next().unwrap().ends_with('x');
            let coord = split.next().unwrap().parse().unwrap();

            folds.push((x, coord));
        }
    }
    Input { points, folds }
}

fn fold(input: &mut Input, fold: usize) {
    let (x, coord) = input.folds[fold];

    for point in input.points.clone() {
        if x && point.0 > coord {
            input.points.remove(&point);
            input.points.insert((coord - (point.0 - coord), point.1));
        } else if !x && point.1 > coord {
            input.points.remove(&point);
            input.points.insert((point.0, coord - (point.1 - coord)));
        }
    }
}

// 781
pub fn part1() {
    let mut input = parse_input();

    fold(&mut input, 0);

    println!("{}", input.points.len());
}

// PERCGJPB
pub fn part2() {
    let mut input = parse_input();

    for i in 0..input.folds.len() {
        fold(&mut input, i);
    }

    let highest_x = input.points.iter().max_by_key(|p| p.0).unwrap().0;
    let highest_y = input.points.iter().max_by_key(|p| p.1).unwrap().1;

    for y in 0..=highest_y {
        for x in 0..=highest_x {
            if input.points.contains(&(x, y)) {
                print!("0");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
