// 3847100
pub fn part1() {
    let mut zeros_by_column = [0u32; 12];
    let mut rows = 0;

    for line in include_str!("../input/day3.txt").lines() {
        for (i, char) in line.bytes().enumerate() {
            if char == b'0' {
                zeros_by_column[i] += 1;
            }
        }
        rows += 1;
    }

    let gamma_threshold = rows / 2;

    let mut gamma = 0u16;
    let mut epsilon = 0u16;

    for (i, v) in zeros_by_column.iter().rev().enumerate() {
        if v > &gamma_threshold {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    let answer = gamma as u32 * epsilon as u32;

    println!("{answer}");
}

// 4105235
pub fn part2() {
    fn get_rating(numbers: &mut Vec<u16>, o2: bool) -> u16 {
        for i_col in 0..12 {
            let bit = 1 << (11 - i_col);
            let zeros = numbers.iter().filter(|x| (*x & bit) == 0).count();
            let zero_most_common = if o2 { zeros > numbers.len() / 2 } else { zeros <= numbers.len() / 2 };

            let last = numbers.last().unwrap().to_owned();

            numbers.retain(|n| (n & bit) == 0 && zero_most_common || (n & bit) != 0 && !zero_most_common);

            if numbers.len() == 0 {
                return last;
            }

            if numbers.len() == 1 {
                return numbers[0];
            }
        }

        panic!("no value found!");
    }

    // Parse numbers from input
    let numbers: Vec<u16> = include_str!("../input/day3.txt")
        .lines()
        .map(|l| u16::from_str_radix(l, 2).unwrap())
        .collect();
    
    // Get ratings and multiply them together
    let o2_rating = get_rating(&mut numbers.clone(), true);
    let co2_rating = get_rating(&mut numbers.clone(), false);
    let answer = o2_rating as u32 * co2_rating as u32;

    println!("{answer}");
}
