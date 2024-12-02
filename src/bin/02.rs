advent_of_code::solution!(2);

fn get_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(" ").map(|num| num.parse().unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = get_reports(input);
    let mut safe_reports: u32 = 0;
    for report in &reports {
        let mut max_diff = 0;
        if report.iter().is_sorted() || report.iter().rev().is_sorted() {
            for (idx, node) in report.iter().enumerate() {
                if (idx + 1) == report.len() {
                    break;
                };
                let diff = node - report[idx + 1];
                if diff == 0 {
                    max_diff = 0;
                    break;
                }
                max_diff = max_diff.max(diff.abs())
            }
        }

        if max_diff.abs() < 4 && max_diff != 0 {
            safe_reports += 1
        }
    }
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
