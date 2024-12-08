advent_of_code::solution!(3);

use regex::Regex;

fn num_from_str(num: &str) -> u32 {
    num.parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut result = 0;
    for (_, [num_one, num_two]) in re.captures_iter(input).map(|c| c.extract()) {
        result += num_from_str(num_one) * num_from_str(num_two);
    }
    Some(result)
}

/// Candidate for worst code I've ever written
pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|(do\(\))|(don't\(\))").unwrap();
    let mut should_multiply = true;
    let mut result = 0;
    let subs: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    for s in subs {
        if s.starts_with("don") {
            should_multiply = false;
        } else if s.starts_with("do") {
            should_multiply = true;
        } else if should_multiply && s.starts_with("mul") {
            let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
            for (_, [num_one, num_two]) in re.captures_iter(s).map(|c| c.extract()) {
                result += num_from_str(num_one) * num_from_str(num_two);
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
