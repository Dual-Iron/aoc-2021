fn main() {
    week2::day15::part1();
    week2::day15::part2();
}

mod week2 {
    pub mod day15;
}

macro_rules! input {
    ($i:literal) => {
        include_str!(concat!("../../input/day", $i, ".txt"))
    };
}

// I dunno why the `as input` makes clippy shut up, but it does.
pub(crate) use input as input;
