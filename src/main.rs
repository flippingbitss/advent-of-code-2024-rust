#![feature(binary_heap_into_iter_sorted)]
#![feature(array_windows)]

mod day01;
mod day02;

fn main() {
    let input = include_str!("../inputs/day02.in");
    println!("{}", day02::part_one(input));
    println!("{}", day02::part_two(input));
}
