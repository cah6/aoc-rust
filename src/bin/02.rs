use std::cmp::Ordering;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let sum = lines
        .map(line_to_shapes)
        .map(|(opp, mine)| total_score(&opp, &mine))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let sum = lines
        .map(line_to_shapes_part_2)
        .map(|(opp, mine)| total_score(&opp, &mine))
        .sum();
    Some(sum)
}

#[derive(Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        match *self {
            Shape::Rock => match other {
                Shape::Rock => Ordering::Equal,
                Shape::Paper => Ordering::Less,
                Shape::Scissors => Ordering::Greater,
            },
            Shape::Paper => match other {
                Shape::Rock => Ordering::Greater,
                Shape::Paper => Ordering::Equal,
                Shape::Scissors => Ordering::Less,
            },
            Shape::Scissors => match other {
                Shape::Rock => Ordering::Less,
                Shape::Paper => Ordering::Greater,
                Shape::Scissors => Ordering::Equal,
            },
        }
    }
}

fn line_to_shapes(line: &str) -> (Shape, Shape) {
    let mut split = line.split(' ');
    let fst = split.next().unwrap();
    let snd = split.next().unwrap();
    let opp = letter_to_shape(fst);
    let mine = letter_to_shape(snd);
    (opp, mine)
}

fn line_to_shapes_part_2(line: &str) -> (Shape, Shape) {
    let mut split = line.split(' ');
    let fst = split.next().unwrap();
    let snd = split.next().unwrap();
    let opp = letter_to_shape(fst);
    let mine = match snd {
        // need to lose
        "X" => match opp {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        // need to tie
        "Y" => match opp {
            Shape::Rock => Shape::Rock,
            Shape::Paper => Shape::Paper,
            Shape::Scissors => Shape::Scissors,
        },
        // need to win
        "Z" => match opp {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        _ => panic!("Got unexpected second char: [{}]", snd),
    };
    (opp, mine)
}

fn letter_to_shape(letter: &str) -> Shape {
    match letter {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => {
            panic!("Got unknown letter: [{}]", letter)
        }
    }
}

fn total_score(opp: &Shape, mine: &Shape) -> u32 {
    combat_score(opp, mine) + shape_to_score(mine)
}

fn shape_to_score(shape: &Shape) -> u32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn combat_score(opp_shape: &Shape, my_shape: &Shape) -> u32 {
    match my_shape.cmp(opp_shape) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
