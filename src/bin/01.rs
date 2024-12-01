use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = inp(input);

    left.sort_unstable();
    right.sort_unstable();

    let res = left
        .into_iter()
        .zip(right)
        .map(|(n1, n2)| n1.abs_diff(n2))
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = inp(input);

    // Slow Lookup
    // let res = left
    //     .into_iter()
    //     .map(|n| n * right.iter().cloned().filter(|n2| *n2 == n).count() as u32)
    //     .sum();

    // Instead use Hashmap for fast lookup:
    let right_counts: HashMap<u32, u32> = right.into_iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });

    let res = left
        .into_iter()
        .map(|n| n * right_counts.get(&n).copied().unwrap_or(0))
        .sum();

    Some(res)
}

fn inp(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .split('\n')
        .map(|s| {
            s.split_once("   ").map_or((0, 0), |(c1, c2)| {
                (
                    c1.parse().unwrap_or_default(),
                    c2.parse().unwrap_or_default(),
                )
            })
        })
        .unzip()
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
