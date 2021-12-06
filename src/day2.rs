// 1580000
pub fn part1() {
    let mut pos = 0;
    let mut depth = 0;

    for line in include_str!("../input/day2.txt").lines() {
        if line.starts_with("up") {
            depth -= line[3..].parse::<i32>().expect("invalid integer");
        } else if line.starts_with("down") {
            depth += line[5..].parse::<i32>().expect("invalid integer");
        } else if line.starts_with("forward") {
            pos += line[8..].parse::<i32>().expect("invalid integer");
        }
    }

    let answer = pos * depth;

    println!("{answer}");
}

// 1251263225
pub fn part2() {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in include_str!("../input/day2.txt").lines() {
        if line.starts_with("up") {
            aim -= line[3..].parse::<i32>().expect("invalid integer");
        } else if line.starts_with("down") {
            aim += line[5..].parse::<i32>().expect("invalid integer");
        } else if line.starts_with("forward") {
            let x = line[8..].parse::<i32>().expect("invalid integer");
            pos += x;
            depth += aim * x;
        }
    }

    let answer = pos * depth;

    println!("{answer}");
}
