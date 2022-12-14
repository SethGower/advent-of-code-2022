pub fn part_one(input: &str) -> Option<u32> {
    let elves: Vec<u32> = input
        .split("\n\n")
        .map(|x| -> u32 {
            x.lines()
                .map(|y| {
                    if let Ok(num) = y.parse::<u32>() {
                        num
                    } else {
                        0
                    }
                })
                .sum()
        })
        .collect::<Vec<u32>>();

    Some(*elves.iter().max().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|x| -> u32 {
            x.lines()
                .map(|y| {
                    if let Ok(num) = y.parse::<u32>() {
                        num
                    } else {
                        0
                    }
                })
                .sum()
        })
        .collect::<Vec<u32>>();

    elves.sort_by(|a, b| b.cmp(a));
    Some(*&elves[0..3].iter().sum::<u32>())
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
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
