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

fn intersection(mut sets: Vec<HashSet<char>>) -> HashSet<char> {
    if sets.is_empty() {
        return HashSet::new();
    }

    if sets.len() == 1 {
        return sets.pop().unwrap();
    }

    let mut result = sets.pop().unwrap();
    result.retain(|item| sets.iter().all(|set| set.contains(item)));
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut lines_iter = lines.iter();
    let mut priorities = vec![];
    for i in 0..lines.len() / 3 {
        let mut sets: Vec<HashSet<char>> = vec![];
        for _ in 0..3 {
            let mut set = HashSet::new();
            let line = lines_iter.next()?;
            for char in line.chars() {
                set.insert(char);
            }
            sets.push(set);
        }
        let int = intersection(sets);
        priorities.push(int);
    }
    let rv = priorities.iter().map(|h| -> u32 {
        let ch = *h.iter().next().unwrap();
        let rv = match ch as u32 {
            65..=90 => ch as u32 - 38,
            97..=122 => ch as u32 - 96,
            _ => 0,
        };
        rv

    }).sum();
    Some(rv)
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
        assert_eq!(part_two(&input), Some(70));
    }
}
