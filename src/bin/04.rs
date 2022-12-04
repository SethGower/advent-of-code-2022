use regex::Regex;
#[derive(Debug)]
struct Elf {
    left: u32,
    right: u32,
}
impl Elf {
    fn subset(&self, other: &Self) -> bool {
        self.left >= other.left && self.right <= other.right
    }
    fn overlap(&self, other: &Self) -> bool {
        let rv: bool;
        if self.left < other.left {
            // the two are in ascending order
            rv = self.right >= other.left;
        } else {
            rv = self.left <= other.right;
        }
        rv
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)-(\d+),\s*(\d+)-(\d+)").ok()?;
    let overlaps = re
        .captures_iter(input)
        .filter(|cap| {
            let left = Elf {
                left: cap[1].parse::<u32>().unwrap(),
                right: cap[2].parse::<u32>().unwrap(),
            };
            let right = Elf {
                left: cap[3].parse::<u32>().unwrap(),
                right: cap[4].parse::<u32>().unwrap(),
            };
            right.subset(&left) || left.subset(&right)
        })
        .count();
    Some(overlaps as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)-(\d+),\s*(\d+)-(\d+)").ok()?;
    let overlaps = re
        .captures_iter(input)
        .filter(|cap| {
            let left = Elf {
                left: cap[1].parse::<u32>().unwrap(),
                right: cap[2].parse::<u32>().unwrap(),
            };
            let right = Elf {
                left: cap[3].parse::<u32>().unwrap(),
                right: cap[4].parse::<u32>().unwrap(),
            };
            left.overlap(&right)
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
