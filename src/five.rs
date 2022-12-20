use regex::Regex;
use itertools::Itertools;

pub const SAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

pub struct Direction {
    pub from_index: usize,
    pub to_index: usize,
    pub amount: usize,
}

pub fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Direction>) {
    let dir_regex: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut piles: Vec<Vec<char>> = Vec::new();
    let mut dirs: Vec<Direction> = Vec::new();

    for line in input.lines() {
        if line.starts_with("move") {
            let matches = dir_regex.captures(line).unwrap();
            let amount: usize = matches[1].parse().unwrap();
            let from_index: usize = matches[2].parse().unwrap();
            let to_index: usize = matches[3].parse().unwrap();
            dirs.push(Direction {
                amount,
                from_index,
                to_index,
            });
        }
        let mut i = 0;
        for chunk in line.chars().chunks(4).into_iter() {
            i += 1;
            let x: String = chunk.collect();
            if x == "    " {
                continue;
            }
            // TODO: add each chunk to the correct pile
        }
    }

    (piles, dirs)
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn solves_first() {
        assert_eq!(2, solve_first(SAMPLE));
        assert_eq!(513, solve_first(INPUT));
    }

    #[test]
    fn solves_second() {
        assert_eq!(4, solve_second(SAMPLE));
        assert_eq!(878, solve_second(INPUT));
    }
    */
}
