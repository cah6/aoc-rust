use std::collections::{BinaryHeap, HashMap};

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');

    let (_last, sums) = lines.fold((0, HashMap::new()), |(curr, mut sums), item| {
        if item.is_empty() {
            (curr + 1, sums)
        } else {
            let i = item.parse::<u32>().unwrap();
            *sums.entry(curr).or_insert(0) += i;
            (curr, sums)
        }
    });

    let mut sorted = sums.values().copied().collect::<BinaryHeap<_>>();

    sorted.pop()
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split('\n');

    let (_last, sums) = lines.fold((0, HashMap::new()), |(curr, mut sums), item| {
        if item.is_empty() {
            (curr + 1, sums)
        } else {
            let i = item.parse::<u32>().unwrap();
            *sums.entry(curr).or_insert(0) += i;
            (curr, sums)
        }
    });

    let mut sorted = sums.values().copied().collect::<BinaryHeap<_>>();

    let e1 = sorted.pop().unwrap_or_default();
    let e2 = sorted.pop().unwrap_or_default();
    let e3 = sorted.pop().unwrap_or_default();

    Some(e1 + e2 + e3)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24_000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
