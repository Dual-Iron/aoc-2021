#[derive(Default)]
struct BingoBoard {
    num: [u16; 25],
    marked: [bool; 25],
}

impl BingoBoard {
    fn parse_boards(lines: std::str::Lines) -> Vec<Self> {
        let mut bingo_boards = Vec::with_capacity(16);
        let mut row = 0usize;

        for line in lines.filter(|s| !s.is_empty()) {
            if row == 0 {
                bingo_boards.push(BingoBoard {
                    num: [0; 25],
                    marked: [false; 25],
                });
            }

            for (i, v) in line.split_ascii_whitespace().enumerate() {
                bingo_boards.last_mut().unwrap().num[i + row * 5] =
                    v.parse().expect("expected bingo table");
            }

            row = (row + 1) % 5;
        }

        bingo_boards
    }

    fn mark(&mut self, number: u16) {
        for (i, v) in self.num.iter().enumerate() {
            if *v == number {
                self.marked[i] = true;
                break;
            }
        }
    }

    fn winning(&self) -> bool {
        (0..5).any(|col| (0..5).all(|y| self.marked[col + 5 * y]))
            || (0..5).any(|row| (0..5).all(|x| self.marked[x + 5 * row]))
    }

    fn score(&self, drawn: u16) -> u32 {
        let mut sum = 0;
        for i in 0..self.num.len() {
            if !self.marked[i] {
                sum += self.num[i];
            }
        }
        sum as u32 * drawn as u32
    }
}

fn get_draws(lines: &mut std::str::Lines) -> Vec<u16> {
    let draws: Vec<u16> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().expect("expected bingo draws"))
        .collect();
    draws
}

// 82440
pub fn part1() {
    let mut lines = include_str!("../input/day4.txt").lines();

    let draws = get_draws(&mut lines);
    let mut boards = BingoBoard::parse_boards(lines);

    for draw in draws {
        for board in &mut boards {
            board.mark(draw);

            if board.winning() {
                let answer = board.score(draw);
                println!("{answer}");
                return;
            }
        }
    }
}

// 20774
pub fn part2() {
    let mut lines = include_str!("../input/day4.txt").lines();

    let draws = get_draws(&mut lines);
    let mut boards = BingoBoard::parse_boards(lines);
    let mut last_score = 0;

    for draw in draws {
        for board in &mut boards {
            if !board.winning() {
                board.mark(draw);

                if board.winning() {
                    last_score = board.score(draw);
                }
            }
        }
    }

    println!("{last_score}");
}
