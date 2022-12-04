use std::collections::HashSet;
pub fn part_one(input: &str) -> Option<u32> {
    let overlaps = input
        .lines()
        .filter(|line| {
            let mut sets: Vec<HashSet<u32>> = vec![];
            for elf in line.split(',') {
                let mut elf = elf.split('-');
                let left = elf.next().unwrap().parse::<u32>().unwrap();
                let right = elf.next().unwrap().parse::<u32>().unwrap();
                let mut set: HashSet<u32> = HashSet::new();
                for j in left..=right {
                    set.insert(j);
                }
                sets.push(set);
            }
            if sets.get(0).unwrap().is_subset(sets.get(1).unwrap())
                || sets.get(1).unwrap().is_subset(sets.get(0).unwrap())
            {
                true
            } else {
                false
            }
        })
        .count();
    Some(overlaps as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let overlaps = input
        .lines()
        .filter(|line| {
            let mut sets: Vec<HashSet<u32>> = vec![];
            for elf in line.split(',') {
                let mut elf = elf.split('-');
                let left = elf.next().unwrap().parse::<u32>().unwrap();
                let right = elf.next().unwrap().parse::<u32>().unwrap();
                let mut set: HashSet<u32> = HashSet::new();
                for j in left..=right {
                    set.insert(j);
                }
                sets.push(set);
            }
            if sets
                .get(0)
                .unwrap()
                .intersection(sets.get(1).unwrap())
                .count()
                > 0
            {
                true
            } else {
                false
            }
        })
        .count();
    Some(overlaps as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
