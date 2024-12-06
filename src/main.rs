#![feature(binary_heap_into_iter_sorted)]
#![feature(array_windows)]

mod day01;
mod day02;
mod day03;

fn main() {
    let input = include_str!("../inputs/day03.in");
    println!("{}", day03::part_one(input));
    println!("{}", day03::part_two(input));
}
