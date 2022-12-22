use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

pub const SAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction {
    pub from_index: usize,
    pub to_index: usize,
    pub amount: usize,
}

pub fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<Direction>) {
    let dir_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut piles: Vec<VecDeque<char>> = vec![];
    let mut dirs: Vec<Direction> = Vec::new();

    for line in input.lines() {
        if line.starts_with("move") {
            let matches = dir_regex.captures(line).unwrap();
            let amount: usize = matches[1].parse().unwrap();
            let from_index = matches[2].parse::<usize>().unwrap() - 1;
            let to_index = matches[3].parse::<usize>().unwrap() - 1;
            dirs.push(Direction {
                amount,
                from_index,
                to_index,
            });
        }
        for (i, chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let x: String = chunk.collect();
            if !x.contains('[') {
                continue;
            }

            while piles.len() <= i {
                piles.push(VecDeque::new());
            }

            piles
                .get_mut(i)
                .unwrap()
                .push_back(x.chars().nth(1).unwrap());
        }
    }

    (piles, dirs)
}

pub fn solve_first(input: &str) -> String {
    let mut result = String::new();
    let (mut piles, dirs) = parse_input(input);
    for dir in dirs {
        process_direction(dir, &mut piles);
    }

    println!("{:?}", piles);
    for mut pile in piles {
        result.push(pile.pop_front().unwrap());
    }

    result
}

fn process_direction(dir: Direction, piles: &mut Vec<VecDeque<char>>) {
    for _ in 0..dir.amount {
        let val = piles.get_mut(dir.from_index).unwrap().pop_front().unwrap();
        let target = piles.get_mut(dir.to_index).unwrap();
        target.push_front(val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (piles, dirs) = parse_input(SAMPLE);
        assert_eq!(2, piles.get(0).unwrap().len());
        assert_eq!(3, piles.get(1).unwrap().len());
        assert_eq!(1, piles.get(2).unwrap().len());
        assert_eq!(4, dirs.len());
    }

    #[test]
    fn test_process_direction() {
        let (mut piles, dirs) = parse_input(SAMPLE);
        process_direction(*dirs.first().unwrap(), &mut piles);
        let first_pile = piles.first().unwrap().into_iter().collect_vec();
        let second_pile = piles.get(1).unwrap().into_iter().collect_vec();
        assert_eq!(vec![&'D', &'N', &'Z'], first_pile);
        assert_eq!(vec![&'C', &'M'], second_pile);
    }

    #[test]
    fn solves_first() {
        assert_eq!("CMZ", solve_first(SAMPLE));
        assert_eq!("CMZ", solve_first(INPUT));
    }
    /*

    #[test]
    fn solves_second() {
        assert_eq!(4, solve_second(SAMPLE));
        assert_eq!(878, solve_second(INPUT));
    }
    */
}

const INPUT: &str = "            [G] [W]         [Q]    
[Z]         [Q] [M]     [J] [F]    
[V]         [V] [S] [F] [N] [R]    
[T]         [F] [C] [H] [F] [W] [P]
[B] [L]     [L] [J] [C] [V] [D] [V]
[J] [V] [F] [N] [T] [T] [C] [Z] [W]
[G] [R] [Q] [H] [Q] [W] [Z] [G] [B]
[R] [J] [S] [Z] [R] [S] [D] [L] [J]
 1   2   3   4   5   6   7   8   9 

move 6 from 5 to 7
move 2 from 9 to 1
move 4 from 8 to 6
move 1 from 8 to 1
move 2 from 9 to 1
move 1 from 6 to 1
move 13 from 7 to 8
move 1 from 2 to 8
move 9 from 1 to 5
move 1 from 3 to 8
move 3 from 6 to 7
move 4 from 4 to 1
move 11 from 5 to 6
move 6 from 6 to 9
move 3 from 4 to 2
move 7 from 8 to 6
move 1 from 7 to 5
move 1 from 4 to 3
move 7 from 1 to 5
move 2 from 2 to 7
move 4 from 9 to 6
move 1 from 3 to 6
move 1 from 1 to 9
move 1 from 3 to 6
move 1 from 5 to 8
move 4 from 6 to 7
move 3 from 8 to 7
move 7 from 5 to 7
move 1 from 3 to 1
move 1 from 2 to 6
move 14 from 6 to 5
move 2 from 5 to 2
move 3 from 9 to 2
move 6 from 2 to 9
move 7 from 8 to 6
move 7 from 7 to 3
move 2 from 8 to 7
move 6 from 3 to 7
move 17 from 7 to 1
move 1 from 3 to 1
move 1 from 2 to 5
move 4 from 5 to 6
move 17 from 6 to 9
move 7 from 9 to 4
move 1 from 2 to 7
move 2 from 5 to 4
move 3 from 7 to 8
move 7 from 5 to 2
move 6 from 2 to 8
move 8 from 9 to 6
move 1 from 2 to 3
move 8 from 4 to 9
move 7 from 6 to 9
move 18 from 1 to 7
move 1 from 1 to 8
move 2 from 6 to 9
move 1 from 3 to 9
move 1 from 4 to 6
move 1 from 8 to 3
move 1 from 3 to 1
move 10 from 7 to 2
move 9 from 8 to 4
move 1 from 6 to 4
move 2 from 7 to 8
move 5 from 4 to 9
move 17 from 9 to 5
move 2 from 7 to 6
move 5 from 9 to 7
move 5 from 4 to 2
move 8 from 2 to 4
move 8 from 4 to 3
move 2 from 6 to 5
move 2 from 8 to 5
move 3 from 9 to 3
move 4 from 7 to 3
move 6 from 9 to 6
move 4 from 6 to 9
move 5 from 9 to 3
move 8 from 5 to 2
move 1 from 1 to 9
move 1 from 6 to 3
move 1 from 9 to 4
move 5 from 7 to 4
move 19 from 3 to 1
move 4 from 2 to 8
move 13 from 5 to 1
move 1 from 6 to 3
move 3 from 3 to 6
move 2 from 8 to 9
move 4 from 2 to 9
move 2 from 2 to 6
move 1 from 1 to 6
move 5 from 1 to 9
move 10 from 9 to 3
move 15 from 1 to 6
move 21 from 6 to 2
move 20 from 2 to 1
move 2 from 8 to 9
move 28 from 1 to 2
move 6 from 4 to 6
move 2 from 1 to 5
move 3 from 3 to 4
move 2 from 5 to 4
move 1 from 4 to 3
move 3 from 4 to 5
move 2 from 5 to 4
move 1 from 1 to 8
move 25 from 2 to 9
move 1 from 4 to 6
move 1 from 3 to 8
move 4 from 3 to 6
move 1 from 4 to 9
move 2 from 6 to 3
move 1 from 5 to 9
move 5 from 2 to 8
move 7 from 9 to 6
move 2 from 9 to 4
move 3 from 2 to 1
move 3 from 3 to 4
move 1 from 3 to 5
move 16 from 6 to 3
move 7 from 8 to 3
move 5 from 4 to 3
move 1 from 1 to 3
move 1 from 2 to 6
move 1 from 5 to 6
move 21 from 3 to 5
move 2 from 1 to 2
move 1 from 6 to 7
move 10 from 9 to 8
move 1 from 6 to 5
move 5 from 8 to 7
move 12 from 5 to 3
move 20 from 3 to 6
move 4 from 7 to 9
move 1 from 7 to 3
move 1 from 2 to 5
move 1 from 3 to 8
move 2 from 8 to 4
move 4 from 8 to 7
move 3 from 6 to 1
move 1 from 1 to 5
move 2 from 9 to 2
move 2 from 1 to 5
move 2 from 5 to 6
move 3 from 7 to 1
move 2 from 1 to 4
move 4 from 6 to 8
move 3 from 4 to 7
move 3 from 2 to 5
move 2 from 7 to 9
move 9 from 9 to 8
move 1 from 4 to 1
move 7 from 5 to 7
move 1 from 7 to 8
move 1 from 3 to 1
move 4 from 7 to 5
move 2 from 1 to 9
move 1 from 1 to 2
move 5 from 5 to 4
move 1 from 2 to 6
move 5 from 7 to 9
move 5 from 4 to 7
move 11 from 9 to 6
move 14 from 8 to 9
move 23 from 6 to 5
move 6 from 9 to 5
move 1 from 6 to 2
move 10 from 5 to 3
move 1 from 4 to 9
move 1 from 2 to 1
move 2 from 7 to 3
move 10 from 5 to 7
move 8 from 5 to 2
move 5 from 3 to 5
move 7 from 5 to 8
move 1 from 2 to 7
move 9 from 7 to 9
move 3 from 2 to 3
move 2 from 6 to 2
move 2 from 3 to 6
move 4 from 7 to 5
move 1 from 1 to 5
move 4 from 3 to 1
move 2 from 5 to 2
move 1 from 3 to 2
move 2 from 6 to 8
move 7 from 5 to 3
move 9 from 2 to 4
move 2 from 1 to 2
move 2 from 5 to 3
move 1 from 4 to 9
move 1 from 6 to 9
move 1 from 4 to 2
move 2 from 1 to 7
move 3 from 2 to 6
move 4 from 8 to 7
move 2 from 8 to 3
move 2 from 3 to 7
move 1 from 6 to 5
move 2 from 8 to 2
move 5 from 4 to 1
move 8 from 9 to 8
move 1 from 5 to 7
move 10 from 9 to 2
move 8 from 8 to 2
move 1 from 1 to 6
move 12 from 3 to 9
move 7 from 7 to 4
move 13 from 2 to 4
move 7 from 2 to 7
move 1 from 6 to 7
move 3 from 9 to 8
move 2 from 6 to 3
move 1 from 3 to 2
move 1 from 3 to 9
move 3 from 1 to 5
move 1 from 1 to 6
move 4 from 7 to 6
move 5 from 7 to 1
move 1 from 2 to 1
move 6 from 9 to 4
move 5 from 9 to 7
move 3 from 8 to 3
move 22 from 4 to 9
move 24 from 9 to 8
move 1 from 9 to 2
move 2 from 4 to 3
move 10 from 8 to 3
move 1 from 2 to 1
move 1 from 3 to 8
move 1 from 6 to 3
move 1 from 1 to 4
move 4 from 3 to 4
move 4 from 6 to 1
move 2 from 4 to 5
move 4 from 7 to 2
move 7 from 4 to 6
move 4 from 6 to 1
move 2 from 6 to 3
move 1 from 6 to 2
move 5 from 5 to 2
move 12 from 3 to 5
move 3 from 7 to 8
move 6 from 2 to 3
move 11 from 1 to 9
move 1 from 1 to 7
move 1 from 7 to 5
move 2 from 3 to 9
move 2 from 9 to 7
move 4 from 2 to 5
move 2 from 7 to 1
move 17 from 8 to 1
move 1 from 3 to 2
move 16 from 1 to 3
move 8 from 3 to 4
move 2 from 8 to 3
move 2 from 1 to 5
move 1 from 2 to 6
move 12 from 5 to 8
move 1 from 6 to 3
move 9 from 3 to 9
move 8 from 4 to 6
move 2 from 1 to 6
move 6 from 8 to 4
move 3 from 4 to 6
move 1 from 1 to 9
move 11 from 6 to 8
move 3 from 4 to 3
move 17 from 9 to 5
move 2 from 6 to 7
move 1 from 9 to 1
move 2 from 8 to 6
move 1 from 7 to 5
move 1 from 8 to 9
move 1 from 1 to 7
move 3 from 9 to 6
move 2 from 7 to 8
move 1 from 9 to 6
move 15 from 5 to 2
move 9 from 3 to 9
move 11 from 8 to 3
move 6 from 9 to 8
move 4 from 6 to 7
move 3 from 3 to 7
move 5 from 5 to 6
move 7 from 7 to 5
move 3 from 6 to 1
move 2 from 1 to 4
move 1 from 9 to 2
move 2 from 9 to 3
move 2 from 6 to 3
move 1 from 1 to 8
move 6 from 5 to 9
move 8 from 2 to 5
move 10 from 8 to 5
move 1 from 2 to 9
move 21 from 5 to 9
move 2 from 8 to 4
move 5 from 9 to 1
move 2 from 5 to 2
move 15 from 9 to 2
move 1 from 5 to 9
move 9 from 9 to 3
move 1 from 1 to 6
move 3 from 4 to 1
move 20 from 3 to 5
move 20 from 5 to 4
move 7 from 4 to 3
move 1 from 1 to 7
move 11 from 4 to 5
move 4 from 3 to 2
move 11 from 5 to 4
move 2 from 6 to 7
move 4 from 3 to 9
move 2 from 2 to 8
move 2 from 9 to 4
move 6 from 4 to 6
move 2 from 7 to 9
move 1 from 7 to 6
move 1 from 4 to 9
move 4 from 4 to 6
move 2 from 8 to 6
move 1 from 4 to 3
move 1 from 4 to 6
move 1 from 3 to 1
move 3 from 4 to 3
move 9 from 2 to 8
move 2 from 3 to 7
move 5 from 6 to 2
move 2 from 7 to 5
move 1 from 5 to 2
move 1 from 9 to 3
move 1 from 5 to 1
move 13 from 2 to 5
move 4 from 9 to 5
move 1 from 3 to 4
move 9 from 2 to 3
move 7 from 3 to 2
move 11 from 5 to 6
move 5 from 8 to 7
move 1 from 3 to 1
move 2 from 8 to 5
move 2 from 8 to 1
move 1 from 4 to 1
move 6 from 2 to 7
move 3 from 5 to 3
move 1 from 2 to 5
move 7 from 7 to 9
move 3 from 3 to 5
move 1 from 2 to 5
move 2 from 3 to 2
move 6 from 1 to 7
move 10 from 7 to 3
move 1 from 2 to 3
move 6 from 9 to 8
move 1 from 2 to 4
move 2 from 6 to 1
move 5 from 1 to 9
move 8 from 5 to 8
move 2 from 1 to 6
move 6 from 3 to 4
move 1 from 5 to 3
move 4 from 9 to 6
move 1 from 1 to 4
move 2 from 9 to 2
move 5 from 6 to 1
move 11 from 6 to 7
move 1 from 2 to 8
move 6 from 7 to 5
move 10 from 8 to 4
move 2 from 3 to 9
move 3 from 3 to 5
move 4 from 7 to 9
move 2 from 1 to 3
move 10 from 5 to 8
move 6 from 6 to 1
move 2 from 6 to 8
move 2 from 9 to 5
move 4 from 9 to 6
move 7 from 4 to 8
move 5 from 6 to 1
move 4 from 8 to 2
move 2 from 5 to 6
move 5 from 4 to 5
move 1 from 7 to 5
move 2 from 3 to 6
move 1 from 3 to 8
move 4 from 6 to 1
move 4 from 2 to 3
move 5 from 5 to 1
move 2 from 3 to 2
move 2 from 3 to 2
move 20 from 8 to 2
move 5 from 4 to 8
move 1 from 4 to 3
move 8 from 2 to 1
move 1 from 5 to 6
move 5 from 2 to 3
move 1 from 6 to 5
move 5 from 3 to 2
move 1 from 3 to 7
move 6 from 8 to 5
move 13 from 2 to 9
move 7 from 9 to 8
move 1 from 7 to 8
move 5 from 8 to 3
move 2 from 2 to 5
move 2 from 8 to 4
move 27 from 1 to 5
move 1 from 2 to 3
move 5 from 3 to 1
move 22 from 5 to 7
move 1 from 8 to 5
move 1 from 3 to 2
move 7 from 1 to 3
move 2 from 3 to 7
move 2 from 2 to 4
move 5 from 9 to 1
move 5 from 3 to 9
move 3 from 1 to 5
move 3 from 1 to 6
move 3 from 6 to 3
move 4 from 4 to 2
move 8 from 5 to 3
move 8 from 7 to 4
move 14 from 7 to 4
move 1 from 1 to 7
move 6 from 9 to 6
move 7 from 5 to 3
move 14 from 3 to 6
move 2 from 2 to 1
move 4 from 3 to 7
move 6 from 7 to 6
move 1 from 7 to 6
move 1 from 5 to 1
move 2 from 1 to 5
move 3 from 5 to 7
move 8 from 6 to 5
move 5 from 5 to 1
move 1 from 7 to 3
move 1 from 3 to 8
move 22 from 4 to 7
move 7 from 6 to 3
move 4 from 3 to 2
move 3 from 1 to 3
move 17 from 7 to 6
move 1 from 8 to 1
move 2 from 2 to 4
move 3 from 7 to 2
move 2 from 2 to 9
move 1 from 1 to 8
move 2 from 3 to 1
move 6 from 6 to 8
move 2 from 9 to 2
move 4 from 5 to 1
move 5 from 8 to 9
move 1 from 7 to 3
move 4 from 3 to 4
move 1 from 7 to 4
move 4 from 9 to 7
move 5 from 7 to 9
move 1 from 7 to 3
move 2 from 2 to 8
move 5 from 4 to 2
move 21 from 6 to 8
move 2 from 3 to 8
move 23 from 8 to 6
move 1 from 2 to 6
move 2 from 9 to 8
move 22 from 6 to 7
move 2 from 9 to 3
move 2 from 3 to 7
move 2 from 1 to 6
move 1 from 2 to 5
move 3 from 1 to 3
move 6 from 7 to 4
move 5 from 8 to 5
move 1 from 3 to 8
move 1 from 9 to 3
move 6 from 4 to 8
move 1 from 5 to 3
move 6 from 2 to 8
move 15 from 7 to 5
move 1 from 7 to 1
move 14 from 5 to 8
move 1 from 4 to 9
move 5 from 1 to 7
move 3 from 6 to 2
move 4 from 5 to 6
move 1 from 4 to 8
move 4 from 3 to 1
move 2 from 9 to 2
move 7 from 7 to 1
move 7 from 2 to 7
move 9 from 8 to 6
move 7 from 7 to 1
move 12 from 6 to 8
move 25 from 8 to 6
move 3 from 8 to 1
move 28 from 6 to 2
move 15 from 2 to 3
move 1 from 5 to 4
move 3 from 2 to 7
move 6 from 2 to 9";
