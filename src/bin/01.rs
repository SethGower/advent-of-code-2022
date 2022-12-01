pub fn part_one(input: &str) -> Option<u32> {
    // let elves: Vec<u32> = input
    //     .split("\n\n")
    //     .map(|x| x.split('\n').map(|y| y.parse::<u32>().unwrap()).sum())
    //     .collect();
    //
    // Some(*elves.iter().max().unwrap())
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut calories: Vec<u32> = Vec::new();
    for elf in elves {
        calories.push(
            elf.split('\n')
                // figure out better error handling. This was panicking on the last line
                .map(|x| x.parse::<u32>().unwrap_or_default())
                .sum(),
        )
    }
    Some(*calories.iter().max().unwrap())

    // None
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut calories: Vec<u32> = Vec::new();
    for elf in elves {
        calories.push(
            elf.split('\n')
                // figure out better error handling. This was panicking on the last line
                .map(|x| x.parse::<u32>().unwrap_or_default())
                .sum(),
        )
    }
    calories.sort();
    let sum = calories.pop()? + calories.pop()? + calories.pop()?;
    Some(sum)
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
