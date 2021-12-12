fn score_p1(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        _ => 25137,
    }
}

fn score_p2(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        _ => 4,
    }
}

fn get_closing_char(open: char) -> char {
    match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => '>',
    }
}

// 311895
pub fn part1() {
    let mut score = 0;
    let mut stack = Vec::new();

    for line in crate::input!(10).lines() {
        stack.clear();
        for c in line.chars() {
            if let '(' | '[' | '{' | '<' = c {
                // Open delimiter
                stack.push(c);
            } else if c != get_closing_char(stack.pop().unwrap()) {
                // End delimiter does not match; line is corrupted
                score += score_p1(c);
                break;
            }
        }
    }

    println!("{score}");
}

// 2904180541
pub fn part2() {
    let mut scores = Vec::new();
    let mut stack = Vec::new();

    'line: for line in crate::input!(10).lines() {
        stack.clear();

        for c in line.chars() {
            if let '(' | '[' | '{' | '<' = c {
                // Open delimiter
                stack.push(c);
            } else if c != get_closing_char(stack.pop().unwrap()) {
                // End delimiter does not match; discard line
                continue 'line;
            }
        }

        // Traverse the stack and add appropriate scores
        let mut stack_score = 0;
        while let Some(c) = stack.pop() {
            stack_score *= 5;
            stack_score += score_p2(c);
        }
        scores.push(stack_score);
    }

    scores.sort_unstable();
    
    println!("{}", scores[scores.len() / 2]);
}
