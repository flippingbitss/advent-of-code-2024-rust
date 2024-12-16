type Coord = (usize, usize);

struct Grid {
    values: Vec<Vec<Option<usize>>>,
    trailheads: Vec<Coord>,
    height: usize,
    width: usize,
}

impl Grid {
    fn get(&self, coord: Coord) -> Option<usize> {
        self.values
            .get(coord.1)
            .and_then(|row| row.get(coord.0).and_then(|c| *c))
    }
}

fn parse(input: &str) -> Grid {
    let values: Vec<Vec<Option<usize>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|x| x as usize))
                .collect()
        })
        .collect();
    let height = values.len();
    let width = values.first().map_or(0, |row| row.len());

    let mut trailheads = Vec::new();
    for (y, row) in values.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            if col.is_some_and(|c| c == 0) {
                trailheads.push((x, y));
            }
        }
    }

    Grid {
        values,
        trailheads,
        height,
        width,
    }
}

fn trail_count(grid: &Grid, pos: (usize, usize), visited: &mut [bool]) -> (usize, usize) {
    const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    const TARGET: usize = 9;

    let mut total_trails = 0;
    let mut distinct_trails = 0;
    if let Some(curr) = grid.get(pos) {
        if curr == TARGET {
            return (1, 1);
        }

        for dir in DIRS {
            let (nx, ny) = (
                (pos.0 as isize + dir.0) as usize,
                (pos.1 as isize + dir.1) as usize,
            );
            let next = (nx, ny);
            if let Some(neigh) = grid.get(next) {
                let index = ny * grid.width + nx;
                if !visited[index] && neigh == curr + 1 {
                    visited[index] = true;
                    distinct_trails += trail_count(grid, next, visited).0;
                }
                if neigh == curr + 1 {
                    visited[index] = true;
                    total_trails += trail_count(grid, next, visited).1;
                }
            }
        }
    }

    (distinct_trails, total_trails)
}

fn solve(input: &str) -> (usize, usize) {
    let grid = parse(input);

    let mut visited = vec![false; grid.width * grid.height];
    for (x, y) in grid.trailheads.iter() {
        visited[y * grid.width + x] = true;
    }

    grid.trailheads
        .iter()
        .map(|&pos| trail_count(&grid, pos, &mut visited.clone()))
        .fold((0, 0), |(xa, xb), (a, b)| (xa + a, xb + b))
}

pub fn part_one(input: &str) -> usize {
    solve(input).0
}

pub fn part_two(input: &str) -> usize {
    solve(input).1
}
