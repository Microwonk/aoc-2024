use std::collections::HashSet;

use iter_tools::Itertools;

advent_of_code::solution!(6);

// TODO: optimize

pub fn part_one(input: &str) -> Option<u32> {
    let (mut player, walls) = parse(input);

    let x_minmax = walls
        .iter()
        .map(|pos| pos.0)
        .minmax()
        .into_option()
        .unwrap();

    let y_minmax = walls
        .iter()
        .map(|pos| pos.1)
        .minmax()
        .into_option()
        .unwrap();

    let mut direction = Direction::Up;

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::from([player]);

    while (x_minmax.0..=x_minmax.1).contains(&player.0)
        && (y_minmax.0..=y_minmax.1).contains(&player.1)
    {
        let dir = direction.tuple();
        let next_position = (player.0 + dir.0, player.1 + dir.1);
        if walls.iter().any(|pos| pos == &next_position) {
            direction = direction.turn_right();
        } else {
            player = next_position;
            visited_positions.insert(player);
        }
    }

    Some(visited_positions.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut player, walls) = parse(input);

    let original_guard_position = player;

    let x_minmax = walls
        .iter()
        .map(|pos| pos.0)
        .minmax()
        .into_option()
        .unwrap();

    let y_minmax = walls
        .iter()
        .map(|pos| pos.1)
        .minmax()
        .into_option()
        .unwrap();

    let mut direction = Direction::Up;

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::from([player]);

    loop {
        let dir = direction.tuple();
        let next_position = (player.0 + dir.0, player.1 + dir.1);

        if walls.iter().any(|pos| pos == &next_position) {
            direction = direction.turn_right();
        } else if (x_minmax.0..=x_minmax.1).contains(&next_position.0)
            && (y_minmax.0..=y_minmax.1).contains(&next_position.1)
        {
            player = next_position;
            visited_positions.insert(player);
        } else {
            break;
        }
    }

    visited_positions.remove(&original_guard_position);

    let results = visited_positions
        .into_iter()
        .filter(|new_wall| {
            let mut player = original_guard_position;
            let mut direction = Direction::Up;

            let mut visited_positions: HashSet<((i32, i32), Direction)> =
                HashSet::from([(player, direction)]);

            loop {
                let dir = direction.tuple();
                let next_position = (player.0 + dir.0, player.1 + dir.1);

                if walls.iter().any(|pos| pos == &next_position) || &next_position == new_wall {
                    direction = direction.turn_right();
                    continue;
                }

                if visited_positions.contains(&(next_position, direction)) {
                    // break true if next_position is also in set
                    break true;
                } else if (x_minmax.0..=x_minmax.1).contains(&next_position.0)
                    && (y_minmax.0..=y_minmax.1).contains(&next_position.1)
                {
                    player = next_position;
                    visited_positions.insert((player, direction));
                    continue;
                } else {
                    break false;
                }
            }
        })
        .count();

    Some(results as u32)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }

    fn tuple(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }
}

fn parse(input: &str) -> ((i32, i32), Vec<(i32, i32)>) {
    let player = input
        .lines()
        .enumerate()
        .find_map(|l| {
            if l.1.contains('^') {
                let p =
                    l.1.char_indices()
                        .find_map(|(i, c)| {
                            if c == '^' {
                                Some((i as i32, l.0 as i32))
                            } else {
                                None
                            }
                        })
                        .expect("Player should exist.");
                Some(p)
            } else {
                None
            }
        })
        .expect("There should still be a player.");

    let walls = input
        .lines()
        .enumerate()
        .flat_map(|l| {
            let w: Vec<(i32, i32)> =
                l.1.char_indices()
                    .filter_map(|(i, c)| {
                        if c == '#' {
                            Some((i as i32, l.0 as i32))
                        } else {
                            None
                        }
                    })
                    .collect();

            w
        })
        .collect();

    (player, walls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
