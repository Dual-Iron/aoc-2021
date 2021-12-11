// 1215
pub fn part1() {
    let mut increase_count = 0;
    let mut line_cur = 0;
    let mut line_last;

    for line in crate::input!(1).lines() {
        line_last = line_cur;
        line_cur = line.parse().expect("invalid integer");

        if line_last > 0 && line_cur > line_last {
            increase_count += 1;
        }
    }

    println!("{increase_count}");
}

// 1150
pub fn part2() {
    let mut increase_count = 0;
    let mut lines_last = [0; 4];

    for line in crate::input!(1).lines() {
        for i in (1..4).rev() {
            lines_last[i] = lines_last[i - 1];
        }

        lines_last[0] = line.parse().expect("invalid integer");

        if lines_last[3] > 0 && lines_last[0..3].iter().sum::<i32>() > lines_last[1..4].iter().sum() {
            increase_count += 1;
        }
    }

    println!("{increase_count}");
}
