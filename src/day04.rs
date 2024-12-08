fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

pub fn part_one(input: &str) -> usize {
    const WORD: &[u8; 4] = b"XMAS";
    const DIRS: [(isize, isize); 8] = [
        (-1, 1),
        (1, -1),
        (-1, -1),
        (1, 1),
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];

    let grid = parse(input);
    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            for (dx, dy) in DIRS {
                for i in 0..WORD.len() {
                    let nx = x as isize + dx * i as isize;
                    let ny = y as isize + dy * i as isize;
                    if let Some(nrow) = grid.get(ny as usize) {
                        if let Some(ncol) = nrow.get(nx as usize) {
                            if *ncol == WORD[i] {
                                if i == WORD.len() - 1 {
                                    count += 1;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

pub fn part_two(input: &str) -> usize {
    const WORD: &[u8; 3] = b"MAS";

    type Dir = (isize, isize);
    const PRIM_DIAG: Dir = (1, 1);
    const PRIM_DIAG_REV: Dir = (-1, -1);
    const SEC_DIAG: Dir = (-1, 1);
    const SEC_DIAG_REV: Dir = (1, -1);

    let grid = parse(input);

    let is_match = |y: usize, x: usize, dir: (isize, isize)| {
        for i in 0..WORD.len() as isize {
            let (nx, ny) = (x as isize + (dir.0 * i), y as isize + (dir.1 * i));
            if let Some(c) = grid.get(ny as usize).and_then(|r| r.get(nx as usize)) {
                if *c == WORD[i as usize] {
                    if i as usize == WORD.len() - 1 {
                        return true;
                    }
                } else {
                    break;
                }
            }
        }
        false
    };

    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            // check M - M
            // below
            if is_match(y, x, PRIM_DIAG) && is_match(y, x + 2, SEC_DIAG) {
                count += 1;
            }
            // up
            if is_match(y, x, SEC_DIAG_REV) && is_match(y, x + 2, PRIM_DIAG_REV) {
                count += 1;
            }
            // check M left and right
            //       |
            //       M
            // left
            if is_match(y, x, SEC_DIAG) && is_match(y + 2, x, PRIM_DIAG_REV) {
                count += 1;
            }
            // right
            if is_match(y, x, PRIM_DIAG) && is_match(y + 2, x, SEC_DIAG_REV) {
                count += 1;
            }
        }
    }
    count
}
