advent_of_code::solution!(1);

fn left_right(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in input.lines() {
        let parts = line.split_once("   ").unwrap();
        left.push(parts.0.parse().unwrap());
        right.push(parts.1.parse().unwrap());
    }
    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = left_right(input);
    left.sort();
    right.sort();
    let mut distances = 0;
    for (index, element) in left.iter().enumerate() {
        distances += element.abs_diff(right[index]);
    }
    Some(distances)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = left_right(input);
    let mut total = 0;
    for num in left {
        let similarity_score =
            usize::try_from(num).unwrap() * right.iter().filter(|&n| *n == num).count();
        total += similarity_score;
    }
    Some(total.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
