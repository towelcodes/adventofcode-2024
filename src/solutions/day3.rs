use iter_tools::Itertools;
use regex::Regex;

pub fn day3(input: String) -> i32 {
    let matches: Vec<&str> = Regex::new(r"(do|don't)\(\)|mul\(\d*,\d*\)").unwrap().find_iter(&input).map(|m| m.as_str()).collect();
    let mut enabled = true;
    let muls: Vec<(i32, i32)> = matches.iter()
        .filter(|&m| {
            let cmd = m.split("(").next().unwrap();
            match cmd {
                "don't" => {
                    enabled = false;
                    false
                }
                "do" => {
                    enabled = true;
                    false
                }
                "mul" => enabled,
                _ => { println!("unknown command: {}", cmd); false }
            }
        })
        .map(|&m| {
            let numbers = &m[4..m.len()-1];
            numbers.split(",").map(|n| n.parse::<i32>().unwrap()).collect_tuple().unwrap()
    }).collect();
    let mut product = 0;
    muls.iter().for_each(|(a, b)| product += a*b);
    product
}