use std::{iter::successors, ops::Range};

use iter_tools::Itertools;

advent_of_code::solution!(8);

type ParseResult = (Range<i32>, Range<i32>, Vec<((i32, i32), char)>);

pub fn part_one(input: &str) -> Option<u32> {
    let (y_bound, x_bound, results) = parse(input);

    let res = results
        .chunk_by(|a, b| a.1 == b.1)
        .flat_map(|chunk| {
            chunk.iter().combinations(2).flat_map(|sats| {
                let sat1 = sats[0].0;
                let sat2 = sats[1].0;
                let diff = (sat1.0 - sat2.0, sat1.1 - sat2.1);
                [
                    (sat1.0 + diff.0, sat1.1 + diff.1),
                    (sat2.0 - diff.0, sat2.1 - diff.1),
                ]
            })
        })
        .filter(|pos| x_bound.contains(&pos.0) && y_bound.contains(&pos.1))
        .unique()
        .count() as u32;

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (y_bound, x_bound, results) = parse(input);

    let res = results
        .chunk_by(|a, b| a.1 == b.1)
        .flat_map(|chunk| {
            chunk
                .iter()
                .combinations(2)
                .flat_map(|sats| {
                    let sat1 = sats[0].0;
                    let sat2 = sats[1].0;
                    let diff = (sat1.0 - sat2.0, sat1.1 - sat2.1);

                    let first: Vec<_> = successors(Some(sat1), |pos| {
                        (x_bound.contains(&pos.0) && y_bound.contains(&pos.1))
                            .then_some((pos.0 + diff.0, pos.1 + diff.1))
                    })
                    .collect();

                    let second: Vec<_> = successors(Some(sat2), |pos| {
                        (x_bound.contains(&pos.0) && y_bound.contains(&pos.1))
                            .then_some((pos.0 - diff.0, pos.1 - diff.1))
                    })
                    .collect();

                    [first, second]
                })
                .flatten()
        })
        .filter(|pos| x_bound.contains(&pos.0) && y_bound.contains(&pos.1))
        .unique()
        .count() as u32;

    Some(res)
}

fn parse(input: &str) -> ParseResult {
    let results = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .filter_map(|(j, c)| {
                    if c.is_alphanumeric() {
                        Some(((i as i32, j as i32), c))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .collect();
    (
        0i32..input.lines().count() as i32,
        0i32..input.lines().next().unwrap().len() as i32,
        results,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
