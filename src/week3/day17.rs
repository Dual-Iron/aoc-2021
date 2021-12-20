use std::collections::HashSet;

struct Rect {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn parse_input() -> Rect {
    let mut split = crate::input!(17).trim_end()[15..].split(", y=");
    let mut x = split.next().unwrap().split("..");
    let mut y = split.next().unwrap().split("..");
    Rect {
        x1: x.next().unwrap().parse().unwrap(),
        x2: x.next().unwrap().parse().unwrap(),
        y1: y.next().unwrap().parse().unwrap(),
        y2: y.next().unwrap().parse().unwrap(),
    }
}

fn additive_fac(n: i32) -> i32 {
    // Equivalent to `n + n-1 + n-2 + ... + 2 + 1`
    n * (n - 1) / 2
}

// 3570
pub fn part1() {
    let bounds = parse_input();
    let max_y_vel = -bounds.y1;
    let max_y_pos = additive_fac(max_y_vel);

    // Reasoning:
    // (1) Because y-pos decreases by 1 every step, the probe will cross the line y=0 exactly once.
    // (2) Max y-vel is achieved when the probe goes from y=0 to "b" (bottom of the target area) in *one* step.
    // (3) Because of (2), max y-vel is simply `-b`.
    // (4) Because of (3) and (1), max y-pos is `(-b) + (-b - 1) + (-b - 2) + ... + 2 + 1`.

    println!("{max_y_pos}");
}

fn lands_in(rect: &Rect, dx: i32, dy: i32) -> bool {
    let (mut x, mut y, mut dx, mut dy) = (0, 0, dx, dy);
    loop {
        x += dx;
        y += dy;
        dx -= dx.signum();
        dy -= 1;

        if x > rect.x2 || y < rect.y1 {
            break false;
        }

        if x >= rect.x1 && y <= rect.y2 {
            break true;
        }
    }
}

// 1919
pub fn part2() {
    let bounds = parse_input();
    let max_y_vel = -bounds.y1;
    let min_y_vel = bounds.y1;
    let max_x_vel = bounds.x2;
    let min_x_vel = (1 + (1.0 + 8.0 + bounds.x1 as f32).sqrt() as i32) / 2;
    // min_x_vel is derived from `n(n - 1)/2 = m`, where n is x_vel and m is the desired position (left side of the rectangle)

    let mut pts = HashSet::new();
    for x in min_x_vel..=max_x_vel {
        for y in min_y_vel..=max_y_vel {
            if lands_in(&bounds, x, y) {
                pts.insert((x, y));
            }
        }
    }

    println!("{}", pts.len());
}
