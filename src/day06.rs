use std::collections::HashSet;

type Coord = (usize, usize);

struct Grid {
    values: Vec<Vec<bool>>,
    height: usize,
    width: usize,
    guard: Coord,
}

impl Grid {
    fn in_bounds(&self, pos: Coord) -> bool {
        (0..self.width).contains(&pos.0) && (0..self.height).contains(&pos.1)
    }
}

fn parse(input: &str) -> Grid {
    let lines = input.lines().collect::<Vec<_>>();
    let values = lines
        .iter()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let guard = lines
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.chars().position(|c| c == '^').map(|x| (x, y)))
        .unwrap();

    Grid {
        values,
        height: lines.len(),
        width: lines.first().map_or(0, |line| line.len()),
        guard,
    }
}

pub fn part_one(input: &str) -> usize {
    let grid = parse(input);
    // clockwise: ^ > v < : x, y
    const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut positions = HashSet::new();
    let mut pos = grid.guard;
    'main: loop {
        for dir in DIRS {
            loop {
                if !grid.in_bounds(pos) {
                    break 'main;
                }
                positions.insert(pos);

                let nx = (pos.0 as isize + dir.0) as usize;
                let ny = (pos.1 as isize + dir.1) as usize;
                let next = (nx, ny);
                if grid.in_bounds(next) && grid.values[ny][nx] {
                    break;
                }
                pos = next;
            }
        }
    }

    positions.len()
}

pub fn part_two(input: &str) -> usize {
    let mut grid = parse(input);
    let mut result = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.values[y][x] {
                continue;
            }
            grid.values[y][x] = true;
            if has_cycle(&grid, grid.guard) {
                result += 1;
            }

            grid.values[y][x] = false;
        }
    }
    result
}

fn has_cycle(grid: &Grid, mut pos: Coord) -> bool {
    // clockwise: ^ > v < : x, y
    const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut positions: Vec<bool> = vec![false; grid.height * grid.width * DIRS.len()];

    loop {
        for (dir_index, dir) in DIRS.iter().enumerate() {
            loop {
                if !grid.in_bounds(pos) {
                    return false;
                }

                let position_index =
                    (DIRS.len() * grid.width * pos.1) + (DIRS.len() * pos.0) + dir_index;
                if let Some(has_old_dir) = positions.get(position_index) {
                    if *has_old_dir {
                        return true;
                    }
                }

                positions[position_index] = true;

                let nx = (pos.0 as isize + dir.0) as usize;
                let ny = (pos.1 as isize + dir.1) as usize;
                let next = (nx, ny);
                if grid.in_bounds(next) && grid.values[ny][nx] {
                    break;
                }

                pos = next;
            }
        }
    }
}
