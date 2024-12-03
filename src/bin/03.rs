use std::collections::HashMap;

use regex::{Captures, Regex};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Some(mul_re.captures_iter(input).map(|c| mul(c)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    // dos and don'ts
    let statements: HashMap<_, _> = Regex::new(r"don't\(\)")
        .unwrap()
        .find_iter(input)
        .map(|mat| (mat.end(), false))
        .chain(
            Regex::new(r"do()\(\)")
                .unwrap()
                .find_iter(input)
                .map(|mat| (mat.end(), true)),
        )
        .collect();

    // fill arr with bool of curr state
    let indices: Vec<bool> = (0..input.len())
        .scan(true, |state, i| {
            if let Some(b) = statements.get(&i) {
                *state = *b;
            }
            Some(*state)
        })
        .collect();

    Some(
        re.captures_iter(input)
            .map(|c| {
                let mat = c.get(0).unwrap();
                if !indices[mat.start()] {
                    return 0;
                }
                mul(c)
            })
            .sum(),
    )
}

fn mul(c: Captures) -> u32 {
    c.get(1)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap_or_default()
        * c.get(2)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap_or_default()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
