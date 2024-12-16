use std::collections::HashMap;

fn parse(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

fn digits(mut num: usize) -> usize {
    let mut res = 0;
    while num > 0 {
        num /= 10;
        res += 1;
    }
    res
}

fn split_num_in_half(num: usize, mid: usize) -> (usize, usize) {
    let mut temp = num;
    let mut right: usize = 0;
    for i in 0..mid {
        right += (10 as usize).pow(i as u32) * (temp % 10);
        temp /= 10;
    }
    let mut left: usize = 0;
    for i in 0..mid {
        left += (10 as usize).pow(i as u32) * (temp % 10);
        temp /= 10;
    }
    (left, right)
}

fn solve(input: &str, ticks: usize) -> usize {
    let stones = parse(input);

    let mut stones_count: HashMap<usize, usize> = HashMap::new();
    for stone in stones {
        *stones_count.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..ticks {
        let mut next: HashMap<usize, usize> = HashMap::new();
        for (&stone, &count) in stones_count.iter() {
            match stone {
                0 => {
                    *next.entry(1).or_insert(0) += count;
                }
                s => {
                    let size = digits(s);
                    if size % 2 == 0 {
                        let (left, right) = split_num_in_half(s, size / 2);
                        *next.entry(left).or_insert(0) += count;
                        *next.entry(right).or_insert(0) += count;
                    } else {
                        let n = s * 2024;
                        *next.entry(n).or_insert(0) += count;
                    }
                }
            }
        }
        stones_count = next;
    }

    stones_count.values().sum()
}

pub fn part_one(input: &str) -> usize {
    solve(input, 25)
}
pub fn part_two(input: &str) -> usize {
    solve(input, 75)
}
