use std::collections::HashMap;

use iter_tools::Itertools;

advent_of_code::solution!(5);

type Rules = HashMap<u32, Vec<u32>>;
type Updates = Vec<Update>;
type Update = Vec<u32>;

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);
    Some(
        updates
            .iter()
            .filter(|update| apply_rules(update, &rules))
            .fold(0, |acc, update| {
                acc + update.get(update.len() / 2).unwrap_or(&0)
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, mut updates) = parse(input);

    let res = updates
        .iter_mut()
        .filter(|update| !apply_rules(update, &rules))
        .map(|update| {
            update.sort_by(|a, b| rule_sorter(a, b, &rules));
            update.get(update.len() / 2).unwrap_or(&0)
        })
        .sum();
    Some(res)
}

fn apply_rules(update: &Update, rules: &Rules) -> bool {
    update.is_sorted_by(|a, b| rule_sorter(a, b, rules).is_lt())
}

fn rule_sorter(a: &u32, b: &u32, rules: &Rules) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    if let Some(higher_priority) = rules.get(a) {
        if higher_priority.contains(b) {
            return Less;
        }
    }
    if let Some(higher_priority) = rules.get(b) {
        if higher_priority.contains(a) {
            return Greater;
        }
    }
    Equal
}

fn parse(input: &str) -> (Rules, Updates) {
    let (rule_lines, update_lines): (Vec<_>, Vec<_>) =
        input.lines().partition(|line| line.contains('|'));

    (
        rule_lines
            .into_iter()
            .map(|line| {
                let (a, b) = line.split_once('|').unwrap();
                (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
            })
            .into_group_map(),
        update_lines
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| line.split(',').map(|s| s.parse::<u32>().unwrap()).collect())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
