#[derive(Debug, Default, Clone, Copy)]
struct Octo {
    value: u32,
    flashed: bool,
}

type OctoGrid = [Octo; 100];

fn should_flash(octo: &Octo) -> bool {
    octo.value > 9 && !octo.flashed
}

fn at(grid: &mut OctoGrid, x: usize, y: usize) -> &mut Octo {
    &mut grid[x + 10 * y]
}

fn flash(grid: &mut OctoGrid, x: usize, y: usize) {
    at(grid, x, y).flashed = true;

    for i in -1..=1 {
        for j in -1..=1 {
            let x2 = x as isize + i;
            let y2 = y as isize + j;

            if (0..10).contains(&x2) && (0..10).contains(&y2) {
                let other = at(grid, x2 as usize, y2 as usize);

                other.value += 1;

                if should_flash(&other) {
                    flash(grid, x2 as usize, y2 as usize);
                }
            }
        }
    }
}

fn step(grid: &mut OctoGrid) -> u32 {
    let mut flashes = 0;

    for octo in grid.iter_mut() {
        octo.value += 1;
    }

    for x in 0..10 {
        for y in 0..10 {
            if should_flash(at(grid, x, y)) {
                flash(grid, x, y);
            }
        }
    }

    for octo in grid.iter_mut() {
        if octo.flashed {
            octo.value = 0;
            octo.flashed = false;
            flashes += 1;
        }
    }

    flashes
}

fn populate(grid: &mut OctoGrid) {
    let mut index = 0;

    for line in crate::input!(11).lines() {
        for c in line.chars() {
            grid[index] = Octo {
                value: c.to_digit(10).unwrap(),
                flashed: false,
            };
            index += 1;
        }
    }
}

// 1743
pub fn part1() {
    let mut flashes = 0;
    let mut grid = [Default::default(); 100];

    populate(&mut grid);

    for _ in 0..100 {
        flashes += step(&mut grid);
    }

    println!("{flashes}");
}

// 364
pub fn part2() {
    let mut step_count = 0;
    let mut grid = [Default::default(); 100];

    populate(&mut grid);

    let steps = loop {
        step_count += 1;

        if step(&mut grid) == 100 {
            break step_count;
        }
    };

    println!("{steps}");
}
