struct Grid {
    v: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn xy(&self, i: usize) -> (usize, usize) {
        let y = i / self.width;
        (i - y * self.width, y)
    }

    fn at(&self, x: usize, y: usize) -> u8 {
        self.v[x + y * self.width]
    }

    fn low_points(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        for i in 0..self.v.len() {
            let y = i / self.width;
            let x = i - y * self.width;
    
            if self.is_low_point(x, y) {
                ret.push(i);
            }
        }
        ret
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let n = self.at(x, y);
    
        (x == 0 || n < self.at(x - 1, y))
            && (x == self.width - 1 || n < self.at(x + 1, y))
            && (y == 0 || n < self.at(x, y - 1))
            && (y == self.height - 1 || n < self.at(x, y + 1))
    }

    fn get_basin_points(&self, x: usize,  y: usize, idxs: &mut Vec<usize>) {
        let idx = x + y * self.width;

        if !idxs.contains(&idx) {
            idxs.push(idx);
        }

        let next = (self.v[idx] + 1)..9;
    
        if x > 0 && next.contains(&self.at(x - 1, y)) {
            self.get_basin_points(x - 1, y, idxs);
        }
        if x < self.width - 1 && next.contains(&self.at(x + 1, y)) {
            self.get_basin_points(x + 1, y, idxs);
        }
        if y > 0 && next.contains(&self.at(x, y - 1)) {
            self.get_basin_points(x, y - 1, idxs);
        }
        if y < self.height - 1 && next.contains(&self.at(x, y + 1)) {
            self.get_basin_points(x, y + 1, idxs);
        }
    }
}

fn get_grid() -> Grid {
    let mut width = 0;
    let mut gvec = Vec::with_capacity(512);
    for line in crate::input!(9).lines() {
        if width == 0 {
            width = line.len();
        }
        for c in line.chars() {
            gvec.push(c.to_digit(10).unwrap() as u8);
        }
    }
    Grid {
        width,
        height: gvec.len() / width,
        v: gvec
    }
}

// 545
pub fn part1() {
    let grid = get_grid();

    println!(
        "{}",
        grid.low_points()
            .iter()
            .map(|&p| 1 + grid.v[p] as u32)
            .sum::<u32>()
    );
}

// 950600
pub fn part2() {
    let grid = get_grid();

    let mut basin_pts = Vec::new();
    let mut largest_basins = [0usize; 3];

    for i in grid.low_points() {
        let (x, y) = grid.xy(i);

        grid.get_basin_points(x, y, &mut basin_pts);

        let size = basin_pts.len();
        let min = largest_basins.iter_mut().min().unwrap();

        if *min < size {
            *min = size;
        }

        basin_pts.clear();
    }

    println!("{}", largest_basins.iter().product::<usize>());
}
