#![feature(binary_heap_into_iter_sorted)]

mod day01;

fn main() {
    let input = include_str!("../inputs/day01.in");
    println!("{}", day01::part_one(input));
    println!("{}", day01::part_two(input));
}
