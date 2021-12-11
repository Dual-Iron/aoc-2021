// 512
pub fn part1() {
    let mut unique_digits = 0;

    for line in crate::input!(8).lines() {
        let digits = line.split_at(61).1.split_whitespace();
        for digit in digits {
            unique_digits += match digit.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        }
    }

    println!("{unique_digits}")
}

fn to_bits(input: &str) -> u8 {
    let mut result = 0u8;
    for b in input.bytes() {
        // a is 97 in utf-8
        result |= 1 << (b - 97);
    }
    result
}

fn nonzero_count(input: u8) -> u8 {
    let mut result = 0u8;
    for i in 0..8 {
        if input & (1 << i) != 0 {
            result += 1;
        }
    }
    result
}

fn get_output(line: &str, displays: &[u8; 10]) -> u32 {
    let mut ret = 0;
    let scrambled_digits = get_display_flags(line.split_at(61).1);

    // Enumerate in reverse because that's how base 10 works
    for (idx, scrambled) in scrambled_digits.iter().rev().enumerate() {
        // Which display corresponds to this scrambled digit?
        let display = displays.iter().position(|d| d == scrambled).unwrap();
        ret += display as u32 * 10u32.pow(idx as u32);
    }
    
    ret
}

fn get_solved(line: &str, displays: &mut [u8; 10]) {
    let mut scrambled = get_display_flags(line.split_at(58).0);
    scrambled.sort_unstable_by_key(|b| nonzero_count(*b));

    // We know these
    displays[1] = scrambled[0];
    displays[4] = scrambled[2];
    displays[7] = scrambled[1];
    displays[8] = scrambled[9];
    for d in &scrambled[3..6] {
        if d & displays[1] == displays[1] {
            // If the digit has both segs from 1, it's 1.
            displays[3] = *d;
        } else if d & (displays[4] & !displays[1]) == displays[4] & !displays[1] {
            // If the digit has segs from 4 that aren't in 1, it's 5.
            displays[5] = *d;
        } else {
            // Otherwise, it's 2.
            displays[2] = *d;
        }
    }
    // Derive these displays from mixing and matching segments
    displays[6] = displays[5] | (displays[8] & !displays[1]);
    displays[9] = displays[5] | displays[1];
    // 0 is whatever's left
    for i in scrambled.iter() {
        if !displays.contains(i) {
            displays[0] = *i;
        }
    }
}

fn get_display_flags(input: &str) -> Vec<u8> {
    let scrambled_str = input.split_whitespace();
    let scrambled: Vec<u8> = scrambled_str.map(|s| to_bits(s)).collect();
    scrambled
}

// 1091165
pub fn part2() {
    let mut output_sum = 0;
    let mut displays = [0u8; 10];

    for line in crate::input!(8).lines() {
        get_solved(line, &mut displays);
        output_sum += get_output(line, &displays);
    }

    println!("{output_sum}")
}
