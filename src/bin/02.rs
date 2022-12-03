use std::cmp::Ord;
use std::cmp::Ordering;
use std::ops::Neg;
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
struct Game_Part1 {
    left: Option<Choice>,
    right: Option<Choice>,
    left_score: u32,
    right_score: u32,
}
impl Game_Part1 {
    fn new(left: &str, right: &str) -> Option<Game_Part1> {
        let mut rv: Game_Part1 = Game_Part1 {
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
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Player {
    choice: Option<Choice>,
    outcome: Option<Outcome>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Game_Part2 {
    left: Option<Player>,
    right: Option<Player>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}
impl Neg for Outcome {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Outcome::Loss => Outcome::Win,
            Outcome::Draw => Outcome::Draw,
            Outcome::Win => Outcome::Loss,
        }
    }
}
impl Game_Part2 {
    fn new(left: &str, right: &str) -> Option<Game_Part2> {
        let mut rv: Game_Part2 = Game_Part2 {
            left: None,
            right: None,
        };
        rv.right = Some(Player {
            choice: None,
            outcome: match right {
                "X" => Some(Outcome::Loss),
                "Y" => Some(Outcome::Draw),
                "Z" => Some(Outcome::Win),
                _ => None,
            },
        });
        rv.left = match left {
            "A" => Some(Player {
                choice: Some(Choice::Rock),
                outcome: Some(-rv.right?.outcome?),
            }),
            "B" => Some(Player {
                choice: Some(Choice::Paper),
                outcome: Some(-rv.right?.outcome?),
            }),
            "C" => Some(Player {
                choice: Some(Choice::Scissors),
                outcome: Some(-rv.right?.outcome?),
            }),
            _ => None,
        };
        if let Some(mut player) = rv.right {
            player.choice = Game_Part2::decide_choice(-player.outcome?, rv.left?.choice?);
            rv.right = Some(player);
        }
        Some(rv)
    }
    fn decide_choice(outcome: Outcome, other_player: Choice) -> Option<Choice> {
        if outcome == Outcome::Draw {
            return Some(other_player);
        } else {
            match other_player {
                Choice::Rock => {
                    if outcome == Outcome::Win {
                        Some(Choice::Scissors)
                    } else {
                        Some(Choice::Paper)
                    }
                }
                Choice::Paper => {
                    if outcome == Outcome::Win {
                        Some(Choice::Rock)
                    } else {
                        Some(Choice::Scissors)
                    }
                }
                Choice::Scissors => {
                    if outcome == Outcome::Win {
                        Some(Choice::Paper)
                    } else {
                        Some(Choice::Rock)
                    }
                }
            }
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        let mut line = line.split(' ');
        let game = Game_Part1::new(line.next()?, line.next()?)?;
        sum += game.right_score;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum: u32 = 0;

    for line in lines {
        let mut line = line.split(' ');
        let game = Game_Part2::new(line.next()?, line.next()?)?;
        if let Some(right) = game.right {
            sum += right.outcome? as u32;
            sum += right.choice? as u32;
        }
    }
    Some(sum)
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
