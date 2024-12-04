use std::{collections::HashMap, ops::Add};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let positions = pos(input);
    let mas = ['M', 'A', 'S'];

    Some(
        positions
            .iter()
            .filter(|(_, value)| **value == 'X')
            .map(|(position, _)| {
                Dir::DIRS_1
                    .iter()
                    .map(|dirs| {
                        dirs.iter()
                            .map(|pos| positions.get(&(position + pos)))
                            .enumerate()
                            .all(|(index, value)| mas.get(index) == value)
                    })
                    .filter(|b| *b)
                    .count() as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let positions = pos(input);
    let ms = ['M', 'S'];

    Some(
        positions
            .iter()
            .filter(|(_, value)| **value == 'A')
            .filter(|(position, _)| {
                Dir::DIRS_2
                    .iter()
                    .map(|dirs| {
                        dirs.iter()
                            .map(|pos| positions.get(&(*position + pos)))
                            .enumerate()
                            .all(|(index, value)| ms.get(index) == value)
                    })
                    .filter(|b| *b)
                    .count()
                    == 2
            })
            .count() as u32,
    )
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Dir {
    x: i32,
    y: i32,
}

impl Add<&Dir> for &Dir {
    type Output = Dir;

    fn add(self, other: &Dir) -> Self::Output {
        Dir {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Dir {
    const DIRS_1: [[Self; 3]; 8] = [
        [Self::new(0, -1), Self::new(0, -2), Self::new(0, -3)],
        [Self::new(0, 1), Self::new(0, 2), Self::new(0, 3)],
        [Self::new(-1, 0), Self::new(-2, 0), Self::new(-3, 0)],
        [Self::new(1, 0), Self::new(2, 0), Self::new(3, 0)],
        [Self::new(-1, -1), Self::new(-2, -2), Self::new(-3, -3)],
        [Self::new(1, -1), Self::new(2, -2), Self::new(3, -3)],
        [Self::new(-1, 1), Self::new(-2, 2), Self::new(-3, 3)],
        [Self::new(1, 1), Self::new(2, 2), Self::new(3, 3)],
    ];

    const DIRS_2: [[Self; 2]; 4] = [
        [Self::new(-1, -1), Self::new(1, 1)],
        [Self::new(-1, 1), Self::new(1, -1)],
        [Self::new(1, 1), Self::new(-1, -1)],
        [Self::new(1, -1), Self::new(-1, 1)],
    ];

    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn pos(input: &str) -> HashMap<Dir, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (Dir::new(x as i32, y as i32), value))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
