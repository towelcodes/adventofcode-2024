use std::collections::HashMap;
use iter_tools::Itertools;
use regex::Regex;

pub fn day1(input: String) -> (u32, u32) {
    let lines: Vec<&str> = input.split("\n").collect();
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

    let mut occurrences: HashMap<u32, u32> = HashMap::new();
    for n in &halves.0 {
        if let Ok(pos) = halves.1.binary_search(n) {
            let mut occ = 0u32;
            let (l, r) = halves.1.split_at(pos);
            for i in l.iter().rev() {
                if i != n { break; }
                occ += 1;
            }
            for i in r.iter() {
                if i != n { break; }
                occ += 1;
            }
            occurrences.insert(*n, occ);
        }
    }

    let mut similarity = 0u32;
    occurrences.iter().for_each(|(k, v)| {
        similarity += k * v;
    });

    (diff, similarity)
}
