use std::collections::BinaryHeap;

struct Grid(Vec<i32>, usize);

fn xy(grid: &Grid, i: usize) -> (usize, usize) {
    (i % grid.1, i / grid.1)
}

fn parse_input() -> Grid {
    let mut nodes = Vec::new();
    let mut side_len = 0;
    for line in crate::input!(15).lines() {
        side_len = line.len();

        for c in line.chars() {
            nodes.push(c.to_digit(10).unwrap() as i32);
        }
    }
    Grid(nodes, side_len)
}

fn neighbors(grid: &Grid, i: usize) -> Vec<usize> {
    let mut ret = Vec::with_capacity(4);
    let (x, y) = xy(grid, i);

    if x > 0 { ret.push(i - 1) }
    if x < grid.1 - 1 { ret.push(i + 1) }
    if y > 0 { ret.push(i - grid.1) }
    if y < grid.1 - 1 { ret.push(i + grid.1) }

    ret
}

fn dijkstra(grid: &Grid) -> Vec<usize> {
    let mut q = BinaryHeap::new();
    let mut visited = vec![false; grid.0.len()];
    let mut dist = vec![i32::MAX; grid.0.len()];
    let mut prev = vec![0; grid.0.len()];

    dist[0] = 0;
    q.push((0, 0));

    while let Some(cur_node) = q.pop() {
        let cur = cur_node.1;
        if cur == grid.0.len() {
            break;
        }

        visited[cur] = true;

        for nxt in neighbors(grid, cur) {
            if !visited[nxt] {
                let alt = dist[cur] + grid.0[nxt];
                if alt < dist[nxt] {
                    dist[nxt] = alt;
                    prev[nxt] = cur;

                    q.push((-dist[nxt], nxt));
                }
            }
        }
    }

    let mut ret = Vec::new();
    let mut cur_i = grid.0.len() - 1;
    while cur_i != 0 {
        ret.push(cur_i);
        cur_i = prev[cur_i];
    }
    ret
}

// 687
pub fn part1() {
    let grid = parse_input();
    let path = dijkstra(&grid);

    println!("{}", path.iter().map(|i| grid.0[*i]).sum::<i32>());
}

// 2957
pub fn part2() {
    let mut grid = parse_input();
    let side_len = grid.1;
    grid.1 *= 5;
    
    let mut grid_ext = Vec::with_capacity(grid.0.len() * 25);
    for y in 0..grid.1 {
        for x in 0..grid.1 {
            let extra = (x / side_len + y / side_len) as i32;
            let i = x % side_len + y % side_len * side_len;
            let n = (grid.0[i] + extra - 1) % 9 + 1;
            grid_ext.push(n);
        }
    }
    grid.0 = grid_ext;

    let path = dijkstra(&grid);

    println!("{:?}", path.iter().map(|i| grid.0[*i]).sum::<i32>());
}
