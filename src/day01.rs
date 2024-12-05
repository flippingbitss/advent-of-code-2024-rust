use std::collections::{BinaryHeap, HashMap};

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .unzip()
}

pub fn part_one(input: &str) -> usize {
    let (left, right) = parse(input);

    let mut min_heap_a = BinaryHeap::new();
    let mut min_heap_b = BinaryHeap::new();

    min_heap_a.extend(left);
    min_heap_b.extend(right);

    min_heap_a
        .into_iter_sorted()
        .zip(min_heap_b.into_iter_sorted())
        .map(|(a, b)| (a - b).abs() as usize)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let (left, right) = parse(input);

    let mut right_counts = HashMap::with_capacity(right.len());
    for num in right {
        right_counts
            .entry(num)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
    }

    left.iter()
        .map(|a| a * right_counts.get(a).cloned().unwrap_or(0))
        .map(|a| a as usize)
        .sum()
}
