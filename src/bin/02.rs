use std::cmp::Ord;
use std::cmp::Ordering;
#[derive(Eq, PartialEq, Copy, Clone, Debug, Ord)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if *self == *other {
            Some(Ordering::Equal)
        } else if *self == Choice::Rock && *other == Choice::Scissors {
            Some(Ordering::Greater)
        } else if *self == Choice::Scissors && *other == Choice::Paper {
            Some(Ordering::Greater)
        } else if *self == Choice::Rock && *other == Choice::Paper {
            Some(Ordering::Less)
        } else if *other == Choice::Rock && *self == Choice::Scissors {
            Some(Ordering::Less)
        } else if *other == Choice::Scissors && *self == Choice::Paper {
            Some(Ordering::Less)
        } else if *other == Choice::Rock && *self == Choice::Paper {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}


#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Game {
    left: Option<Choice>,
    right: Option<Choice>,
    left_score: u32,
    right_score: u32,
}
impl Game {
    fn new(left: &str, right: &str) -> Option<Game> {
        let mut rv: Game = Game {
            left: None,
            right: None,
            left_score: 0,
            right_score: 0,
        };
        rv.left = match left {
            "A" => Some(Choice::Rock),
            "B" => Some(Choice::Paper),
            "C" => Some(Choice::Scissors),
            _ => None,
        };
        rv.right = match right {
            "X" => Some(Choice::Rock),
            "Y" => Some(Choice::Paper),
            "Z" => Some(Choice::Scissors),
            _ => None,
        };
        if let (Some(left), Some(right)) = (&rv.left, &rv.right) {
            rv.left_score = *left as u32;
            rv.right_score = *right as u32;
            if *left == *right {
                rv.left_score += 3;
                rv.right_score += 3;
            } else if *left > *right {
                rv.left_score += 6;
            } else if *left < *right {
                rv.right_score += 6;
            }
        }
        Some(rv)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        let mut line = line.split(' ');
        let game = Game::new(line.next()?, line.next()?)?;
        sum += game.right_score;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
