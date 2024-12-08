#![feature(binary_heap_into_iter_sorted)]
#![feature(array_windows)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let input = include_str!("../inputs/day05.in");
    println!("{}", day05::part_one(input));
    println!("{}", day05::part_two(input));
}
