#![feature(binary_heap_into_iter_sorted)]
#![feature(array_windows)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let input = include_str!("../inputs/day06.in");
    println!("{}", day06::part_one(input));
    println!("{}", day06::part_two(input));
}
