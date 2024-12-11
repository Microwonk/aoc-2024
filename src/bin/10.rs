use std::collections::HashMap;

use pathfinding::prelude::strongly_connected_components_from;

advent_of_code::solution!(10);

type TrailMap = HashMap<(i32, i32), u32>;

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    Some(
        map.iter()
            .filter(|(_, height)| height == &&0)
            .map(|(pos, _)| search_trail(&map, pos))
            .sum(),
    )
}

// TODO
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn search_trail(map: &TrailMap, position: &(i32, i32)) -> u32 {
    let components = strongly_connected_components_from(position, |pos| {
        DIRECTIONS
            .iter()
            .zip(std::iter::repeat(*pos))
            .map(|(dir, location)| ((dir.0 + location.0, dir.1 + location.1), location))
            .filter(|(new_location, location)| {
                map.get(new_location).is_some_and(|h| {
                    let current_height = map.get(location).unwrap();

                    *h == current_height + 1
                })
            })
            .map(|(new, _)| new)
    });

    components
        .into_iter()
        .flatten()
        .filter(|pos| map.get(pos).unwrap() == &9)
        .count() as u32
}

pub fn parse(input: &str) -> TrailMap {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.char_indices()
                .map(|(j, c)| ((i as i32, j as i32), c.to_digit(10).unwrap()))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
