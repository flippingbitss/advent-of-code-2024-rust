#![feature(binary_heap_into_iter_sorted)]
#![feature(array_windows)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    let input = include_str!("../inputs/day11.in");
    println!("{}", day11::part_one(input));
    println!("{}", day11::part_two(input));
}
