use std::collections::HashSet;
pub fn part_one(input: &str) -> Option<u32> {
    let sum: u32 = input
        .lines()
        .map(|x| -> u32 {
            let mut a: HashSet<char> = HashSet::new();
            let mut b: HashSet<char> = HashSet::new();

            let a_chars = &x[0..x.len() / 2];
            for char in a_chars.chars() {
                a.insert(char);
            }

            let b_chars = &x[x.len() / 2..];
            for char in b_chars.chars() {
                b.insert(char);
            }
            let mut int = a.intersection(&b);
            let ch = *int.next().unwrap();
            let rv = match ch as u32 {
                65..=90 => ch as u32 - 38,
                97..=122 => ch as u32 - 96,
                _ => 0,
            };
            rv
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
