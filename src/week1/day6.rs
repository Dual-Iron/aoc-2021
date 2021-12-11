fn sim_day(fish: &mut [u64; 10]) {
    fish[7] += fish[0];
    fish[9] += fish[0];
    
    for i in 1..fish.len() {
        fish[i - 1] = fish[i];
    }

    fish[9] = 0;
}

fn sim_days(days: u16) {
    let mut fish_by_timer = [0u64; 10];
    
    // Get initial conditions
    for substr in crate::input!(6).trim_end().split(",") {
        fish_by_timer[substr.parse::<usize>().expect("number")] += 1;
    }

    // Simulate days
    for _ in 0..days {
        sim_day(&mut fish_by_timer);
    }

    // Print the sum of all fish
    println!("{:?}", fish_by_timer.iter().sum::<u64>());
}

// 362346
pub fn part1() {
    sim_days(80);
}

// 1639643057051
pub fn part2() {
    sim_days(256);
}
