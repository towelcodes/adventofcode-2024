use iter_tools::Itertools;
use regex::Regex;

// i can't get this to work
// i don't know why

pub fn day2(input: String) -> u32 {
    let lines: Vec<Vec<i32>> = input.split("\n").map(|line| {
        Regex::new(r"\b\d+\b").unwrap().find_iter(line).map(|entry| {
            entry.as_str().parse::<i32>().unwrap()
        }).collect_vec()
    }).collect_vec();

    let mut safe = 0u32;
    'o: for line in lines {
        let mut prev: Option<i32> = None;
        let mut prev_diff: Option<i32> = None;
        for entry in &line {
            if prev.is_none() { prev = Some(*entry); continue; }
            let diff = entry - prev.unwrap();
            if diff == 0 { println!("break: no change on line {:?}", line); continue 'o; }
            if let Some(prev_diff) = prev_diff {
                if diff.abs() > 3 {
                    println!("break: diff > 3 on line {:?}", line);
                    continue 'o;
                }
                if diff > 0 {
                    if prev_diff < 0 { println!("break: wrong direction (dec instead of inc) on line {:?}", line); continue 'o; }
                } else if prev_diff > 0 { println!("break: wrong direction (inc instead of dec) on line {:?}", line); continue 'o; }
            }
            prev = Some(*entry);
            prev_diff = Some(diff);
        }
        safe += 1;
    }

    safe
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn no_change() {
        let input = "1 3 4 5 5";
        assert_eq!(day2(input.to_string()), 0);
    }

    #[test]
    fn too_high() {
        let input = "3 4 8 9";
        assert_eq!(day2(input.to_string()), 0);
    }
    
    #[test]
    fn too_low() {
        let input = "9 8 4 3";
        assert_eq!(day2(input.to_string()), 0);
    }
    
    #[test]
    fn wrong_direction() {
        let input = "3 4 7 6";
        assert_eq!(day2(input.to_string()), 0);
    }
}