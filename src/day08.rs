type Vec2 = (isize, isize);

struct Grid {
    positions: Vec<Vec<Vec2>>,
    height: usize,
    width: usize,
}

fn parse(input: &str) -> Grid {
    let size = 126 as usize;
    let mut positions = vec![vec![]; size];

    let lines: Vec<_> = input.lines().map(|line| line.bytes()).collect();
    let height = lines.len();
    let width = lines.first().map_or(0, |l| l.len());

    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.into_iter().enumerate() {
            if c != b'.' {
                positions[c as usize].push((x as isize, y as isize));
            }
        }
    }

    Grid {
        positions,
        height,
        width,
    }
}

pub fn part_one(input: &str) -> usize {
    let grid = parse(input);

    let mut antinodes = vec![vec![false; grid.width]; grid.height];
    let mut result = 0;
    for group in grid.positions {
        for curr in group.iter() {
            for next in group.iter() {
                if curr != next {
                    let diff_x = next.0 - curr.0;
                    let diff_y = next.1 - curr.1;

                    let anti_x = curr.0 + diff_x * 2;
                    let anti_y = curr.1 + diff_y * 2;

                    let existing = antinodes
                        .get_mut(anti_y as usize)
                        .and_then(|xs| xs.get_mut(anti_x as usize));

                    match existing {
                        Some(val @ false) => {
                            result += 1;
                            *val = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    result
}

pub fn part_two(input: &str) -> usize {
    let grid = parse(input);

    let mut antinodes = vec![vec![false; grid.width]; grid.height];

    let mut place_antinode = |x: usize, y: usize| {
        let existing = antinodes.get_mut(y).and_then(|xs| xs.get_mut(x));
        match existing {
            Some(val @ false) => {
                *val = true;
                true
            }
            _ => false,
        }
    };
    let mut result = 0;
    for group in grid.positions {
        for start in group.iter() {
            if group.len() - 1 >= 2 {
                if place_antinode(start.0 as usize, start.1 as usize) {
                    result += 1;
                }
            }
            for end in group.iter() {
                if start != end {
                    let diff_x = end.0 - start.0;
                    let diff_y = end.1 - start.1;
                    let mut curr = *start;
                    let mut next = *end;

                    while (curr.0 as usize) < grid.width && (curr.1 as usize) < grid.height {
                        let anti_x = curr.0 + diff_x * 2;
                        let anti_y = curr.1 + diff_y * 2;

                        if place_antinode(anti_x as usize, anti_y as usize) {
                            result += 1;
                        }

                        curr = next;
                        next = (anti_x, anti_y);
                    }
                }
            }
        }
    }

    result
}
