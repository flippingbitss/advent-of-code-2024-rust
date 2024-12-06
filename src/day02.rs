fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let all_levels = parse(input);

    all_levels
        .iter()
        .filter(|levels| {
            let is_gradual = [-1isize, 1isize].into_iter().any(|sign| {
                levels
                    .array_windows()
                    .all(|&[a, b]| (a as isize - b as isize).signum() == sign)
            });

            let is_diff_in_range = levels
                .array_windows()
                .all(|&[a, b]| (1..4).contains(&a.abs_diff(b)));

            is_gradual && is_diff_in_range
        })
        .count()
}

pub fn part_two(input: &str) -> usize {
    let all_levels = parse(input);
    all_levels
        .into_iter()
        .filter(|levels| {
            let levels = levels.clone();
            let mut safe = is_safe(&levels);
            if !safe {
                for ex in 0..levels.len() {
                    let left = &levels[0..ex];
                    let right = &levels[(ex + 1)..levels.len()];
                    let reduced_level = left
                        .iter()
                        .chain(right.iter())
                        .cloned()
                        .collect::<Vec<usize>>();
                    safe = safe || is_safe(&reduced_level)
                }
            }
            safe
        })
        .count()
}

fn is_safe(levels: &Vec<usize>) -> bool {
    let is_inc = all_match(&levels, |a, b| a < b);
    let is_dec = all_match(&levels, |a, b| a > b);
    let is_diff_in_range = all_match(&levels, |a, b| (1..4).contains(&a.abs_diff(b)));
    (is_inc || is_dec) && is_diff_in_range
}

fn all_match<F>(levels: &Vec<usize>, predicate: F) -> bool
where
    F: Fn(usize, usize) -> bool,
{
    levels.array_windows().all(|&[a, b]| predicate(a, b))
}
