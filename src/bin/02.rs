enum Choice {
    Rock,
    Paper,
    Scissors,
}
enum Player {
    Left,
    Right,
}
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
        Some(rv)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    // let mut games : Vec<Game> = input.lines().map(|x| {
    //     let x = x.split(' ');
    //     let mut game : Game = Game::new(x.next().unwrap(), x.next().unwrap()).unwrap();

    // })
    None
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
