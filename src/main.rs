use std::fs;
use iter_tools::Itertools;
use regex::Regex;

fn day1() -> u32 {
    let contents = fs::read_to_string("inputs/day1.txt").expect("failed to read file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let halves_str: Vec<(&str, &str)> =
        lines.iter()
            .filter(|line| !line.is_empty())
            .map(|line| Regex::new(r"\b\d+\b").unwrap()
                .find_iter(line).map(|li| li.as_str())
                .collect_tuple().expect("more than two halves"))
            .collect_vec();
    
    let mut halves: (Vec<u32>, Vec<u32>) = (vec![], vec![]);
    for (l, r) in halves_str {
        halves.0.push(l.parse::<u32>().unwrap());
        halves.1.push(r.parse::<u32>().unwrap());
    }
    
    halves.0.sort();
    halves.1.sort();
    
    let mut diff = 0u32;
    halves.0.iter().zip(halves.1.iter()).for_each(|(l, r)| {
        diff += l.abs_diff(*r);
    });
    
    diff
}

fn main() {
    println!("Day 1: {}", day1());
}