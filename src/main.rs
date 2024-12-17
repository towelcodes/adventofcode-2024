mod solutions;

use solutions::*;
use std::fs;

fn main() {
    println!("Day 1: {:?}", day1(fs::read_to_string("inputs/day1.txt").expect("failed to read file")));
    // println!("Day 2: {:?}", day2(fs::read_to_string("inputs/day2.txts").expect("failed to read file")));
    println!("Day 3: {:?}", day3(fs::read_to_string("inputs/day3.txt").expect("failed to read file")));
    println!("Day 4: {:?}", day4(fs::read_to_string("inputs/day4.txt").expect("failed to read file")));
}