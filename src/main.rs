fn main() {
    week2::day16::part1();
    week2::day16::part2();
}

mod week2 {
    pub mod day16;
}

macro_rules! input {
    ($i:literal) => {
        include_str!(concat!("../../input/day", $i, ".txt"))
    };
}

// I dunno why the `as input` makes clippy shut up, but it does.
pub(crate) use input as input;
