use iter_tools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse(input);

    let result = problems
        .iter()
        .filter_map(|(test, nums)| {
            if (0..nums.len() - 1)
                .map(|_| vec!['+', '*'])
                .multi_cartesian_product()
                .any(|operators| evaluate_expression1(nums, &operators) == *test)
            {
                Some(test)
            } else {
                None
            }
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let problems = parse(input);

    let result = problems
        .iter()
        .filter_map(|(test, nums)| {
            if (0..nums.len() - 1)
                .map(|_| vec!['+', '*', '|'])
                .multi_cartesian_product()
                .any(|operators| evaluate_expression2(nums, &operators) == *test)
            {
                Some(test)
            } else {
                None
            }
        })
        .sum();

    Some(result)
}

fn evaluate_expression1(numbers: &[u64], operators: &[char]) -> u64 {
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            _ => unreachable!(),
        }
    }
    result
}

fn evaluate_expression2(numbers: &[u64], operators: &[char]) -> u64 {
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            '|' => {
                result = format!("{}{}", result, numbers[i + 1])
                    .parse::<u64>()
                    .expect("should never fail to parse")
            }
            _ => unreachable!(),
        }
    }
    result
}

type Problem = (u64, Vec<u64>);

fn parse(input: &str) -> Vec<Problem> {
    input
        .lines()
        .map(|l| {
            let (test, nums) = l.split_once(':').expect("line must contain :");
            (
                test.parse::<u64>().expect("there must be a number here"),
                nums.split_whitespace()
                    .map(|n| n.parse::<u64>().expect("there must be a number here"))
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
