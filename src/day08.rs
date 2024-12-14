type Vec2 = (usize, usize);

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
                positions[c as usize].push((x, y));
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
                    let diff_x = next.0.wrapping_sub(curr.0);
                    let diff_y = next.1.wrapping_sub(curr.1);

                    let anti_x = curr.0.wrapping_add(diff_x.wrapping_mul(2));
                    let anti_y = curr.1.wrapping_add(diff_y.wrapping_mul(2));

                    let existing = antinodes.get_mut(anti_y).and_then(|xs| xs.get_mut(anti_x));

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
                if place_antinode(start.0, start.1) {
                    result += 1;
                }
            }
            for end in group.iter() {
                if start != end {
                    let diff_x = end.0.wrapping_sub(start.0);
                    let diff_y = end.1.wrapping_sub(start.1);
                    let mut curr = *start;
                    let mut next = *end;

                    while curr.0 < grid.width && curr.1 < grid.height {
                        let anti_x = curr.0.wrapping_add(diff_x.wrapping_mul(2));
                        let anti_y = curr.1.wrapping_add(diff_y.wrapping_mul(2));

                        if place_antinode(anti_x, anti_y) {
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
