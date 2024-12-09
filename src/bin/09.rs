advent_of_code::solution!(9);

type DiskMap = Vec<Block>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    File(u64),
    Empty,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks = parse(input);

    let mut empty_index = blocks.iter().position(|b| matches!(b, Block::Empty));

    for i in (0..blocks.len()).rev() {
        match blocks[i] {
            Block::File(_) => {
                if let Some(empty_pos) = empty_index {
                    blocks.swap(i, empty_pos);
                    empty_index = blocks.iter().position(|b| matches!(b, Block::Empty));
                }
            }
            Block::Empty => continue,
        }
    }

    Some(
        blocks
            .into_iter()
            .filter(|b| *b != Block::Empty)
            .enumerate()
            .fold(0, |acc, (i, block)| match block {
                Block::File(id) => acc + (i as u64 * id),
                Block::Empty => unreachable!(),
            }),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks = parse(input);

    let mut empty_partitions = vec![];

    let mut current_start = None;
    for (i, block) in blocks.iter().enumerate() {
        match (block, current_start) {
            (Block::Empty, None) => current_start = Some(i),
            (Block::Empty, Some(_)) => (),
            (_, Some(start)) => {
                empty_partitions.push((start, i - start)); // (start_index, size)
                current_start = None;
            }
            _ => (),
        }
    }

    // if let Some(start) = current_start {
    //     empty_partitions.push((start, blocks.len() - start));
    // }

    let mut file_groups = vec![];
    let mut current_group = None;

    for (i, block) in blocks.iter().enumerate() {
        match (block, current_group) {
            (Block::File(id), None) => current_group = Some((*id, i, 1)),
            (Block::File(id), Some((group_id, start, size))) if *id == group_id => {
                current_group = Some((group_id, start, size + 1));
            }
            (_, Some((group_id, start, size))) => {
                file_groups.push((group_id, start, size)); // (id, start_index, size)
                current_group = None;

                if let Block::File(id) = block {
                    current_group = Some((*id, i, 1));
                }
            }
            _ => (),
        }
    }

    // if let Some((id, start, size)) = current_group {
    //     file_groups.push((id, start, size));
    // }

    for (_, start, size) in file_groups.into_iter().rev() {
        if let Some((empty_start, _)) = empty_partitions
            .iter()
            .find(|(_, space_size)| *space_size >= size)
        {
            for offset in 0..size {
                blocks.swap(empty_start + offset, start + offset);
            }

            let index = empty_partitions
                .iter()
                .position(|&(s, _)| s == *empty_start)
                .unwrap();
            if empty_partitions[index].1 == size {
                empty_partitions.remove(index);
            } else {
                empty_partitions[index] = (*empty_start + size, empty_partitions[index].1 - size);
            }
        }
    }

    Some(
        blocks
            .into_iter()
            .enumerate()
            // .filter(|(_, b)| *b != Block::Empty)
            .fold(0, |acc, (i, block)| match block {
                Block::File(id) => acc + (i as u64 * id),
                Block::Empty => acc,
            }),
    )
}

fn parse(input: &str) -> DiskMap {
    let mut id = 0;
    input
        .char_indices()
        .filter_map(|(i, c)| {
            let is_file = i % 2 == 0;
            let len = c
                .to_string()
                .parse::<u64>()
                .expect("only numbers can be in this input");

            let block = if is_file {
                // only increase id when file
                let file = Block::File(id);
                id += 1;
                file
            } else {
                Block::Empty
            };

            let blocks = (0..len).map(move |_| block).collect::<Vec<_>>();

            if len != 0 {
                Some(blocks)
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
