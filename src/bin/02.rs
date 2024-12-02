advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        inp(input)
            .into_iter()
            .filter(is_safe)
            // safe reports
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        inp(input)
            .into_iter()
            .filter(|line| {
                if is_safe(line) {
                    return true;
                }

                (0..line.len()).any(|n| {
                    let mut modified = line.clone();
                    modified.remove(n); // Remove the n-th level.
                    is_safe(&modified)
                })
            })
            // safe reports
            .count() as u32,
    )
}

#[allow(clippy::ptr_arg)]
fn is_safe(line: &Vec<i32>) -> bool {
    let decrease = line[0] > line[1];
    line.windows(2).all(|pair| {
        let diff = pair[1] - pair[0];
        if decrease {
            (-3..=-1).contains(&diff)
        } else {
            (1..=3).contains(&diff)
        }
    })
}

fn inp(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|s| {
            s.split(" ")
                .map(|c| c.parse::<i32>().unwrap_or_default())
                .collect()
        })
        .collect()
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
        assert_eq!(result, Some(4));
    }
}
