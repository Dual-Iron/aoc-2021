// The most efficient position to move to is going to be the *median* of the data set.
// I don't know how I know that, but it makes the most intuitive sense to me.

// 356922
pub fn part1() {
    let mut positions: Vec<i32> = crate::input!(7).trim_end().split(',').map(|s| s.parse().expect("number")).collect();

    positions.sort_unstable();

    let median = positions[positions.len() / 2];
    let fuel: i32 = positions.iter().map(|p| (p - median).abs()).sum();

    println!("{fuel}");
}

// 100347031
pub fn part2() {
    fn fuel_cost(pos: i32, target: i32) -> u32 {
        let mut acc = 0;
        for i in 0..(pos - target).abs() as u32 {
            acc += 1 + i
        }
        acc
    }

    let mut positions: Vec<i32> = crate::input!(7).trim_end().split(',').map(|s| s.parse().expect("number")).collect();

    positions.sort_unstable();

    let middle = positions.len() as i32 / 2;

    // The most efficient position isn't *exactly* the median, but it's probably around it.
    // Search the area around the middle until you find the correct value.
    let mut fuel_costs = Vec::<u32>::new();

    for target in middle - 10..middle + 10 {
        let total_fuel_cost = positions.iter().map(|p| fuel_cost(*p, target)).sum();

        fuel_costs.push(total_fuel_cost);
    }

    // Print the minimum fuel cost
    println!("{}", fuel_costs.iter().min().unwrap());
}
