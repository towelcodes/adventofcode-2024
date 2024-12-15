mod solutions;

use solutions::*;
use std::fs;

fn main() {
    println!("Day 1: {:?}", day1(fs::read_to_string("inputs/day1.txt").expect("failed to read file")));
}