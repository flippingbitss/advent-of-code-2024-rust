#![feature(binary_heap_into_iter_sorted)]
#![feature(array_windows)]

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let input = include_str!("../inputs/day04.in");
    println!("{}", day04::part_one(input));
    println!("{}", day04::part_two(input));
}
