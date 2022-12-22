use std::collections::VecDeque;

const SAMPLES: [&str; 5] = [
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
    "bvwbjplbgvbhsrlpgdmjqwftvncz",
    "nppdvjthqldpwncqszvftbrmjlhg",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
];

pub fn solve_first(input: &str) -> usize {
    let mut most_recent: VecDeque<char> = VecDeque::with_capacity(3);
    let mut cs = input.chars().enumerate();
    most_recent.push_back(cs.next().unwrap().1);
    most_recent.push_back(cs.next().unwrap().1);
    most_recent.push_back(cs.next().unwrap().1);
    for (i, c) in cs {
        most_recent.push_back(c);
        for n in 0..most_recent.len() {
            
        }
        if !most_recent.contains(&c) {
            return i + 1;
        }
        most_recent.pop_front();        
    }

    panic!("Failed to find marker!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_first() {
        let res = solve_first(SAMPLES[0]);
        assert_eq!(7, res);
    }
}
