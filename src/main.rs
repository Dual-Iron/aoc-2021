#![feature(format_args_capture)]

fn main() {
    week2::day13::part1();
    week2::day13::part2();
}

mod week2 {
    pub mod day13;
}

macro_rules! input {
    ($i:literal) => {
        include_str!(concat!("../../input/day", $i, ".txt"))
    };
}

// I dunno why the `as input` makes clippy shut up, but it does.
pub(crate) use input as input;
