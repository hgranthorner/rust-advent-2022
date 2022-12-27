use std::collections::HashSet;

pub const SAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Snake {
    pub head: (usize, usize),
    pub tail: (usize, usize),
}

impl Snake {
    pub fn mv(&self, dir: &str, dist: usize) -> Self {
        let mut h = self.head;
        let mut t = self.tail;
        for _ in 0..dist {
            h = match dir {
                "U" => (h.0, h.1 + 1),
                "D" => (h.0, h.1 - 1),
                "L" => (h.0 - 1, h.1),
                "R" => (h.0 + 1, h.1),
                _ => unreachable!(),
            }
        }
        let new_head = match dir {
            "U" => (h.0, h.1 + dist),
            "D" => (h.0, h.1 - dist),
            "L" => (h.0 - dist, h.1),
            "R" => (h.0 + dist, h.1),
            _ => unreachable!(),
        };
        Snake {
            head: new_head,
            tail: todo!(), // (follow(h.0, new_head.0), follow(h.1, new_head.1)),
        }
    }
}

fn make_trail(old_snake: &Snake, new_snake: &Snake) -> Vec<(usize, usize)> {
    vec![(0, 0)]
}

fn total_distance(pos1: (usize, usize), pos2: (usize, usize)) -> (usize, usize) {
    let x_dist: i32 = pos2.0 as i32 - pos1.0 as i32;
    let y_dist: i32 = pos2.1 as i32 - pos1.1 as i32;
    (
        x_dist.unsigned_abs() as usize,
        y_dist.unsigned_abs() as usize,
    )
}

fn follow(old_tail: (usize, usize), new_head: (usize, usize)) -> (usize, usize) {
    // check if tail needs to move
    let dist = total_distance(old_tail, new_head);
    if dist.0 < 2 && dist.1 < 2 {
        return old_tail;
    }

    // check if tail needs to get behind the head
    //if
    todo!();
}

#[derive(Debug)]
pub enum Either {
    Left(usize),
    Right(usize),
}

fn solve_first(input: &str) -> usize {
    let mut tail_pos: HashSet<(usize, usize)> = HashSet::new();
    let mut s = Snake {
        head: (0, 0),
        tail: (0, 0),
    };
    for line in input.lines() {
        let mut splits = line.split(' ');
        let dir = splits.next().unwrap();
        let dist = splits.next().unwrap().parse().unwrap();
        let new_s = s.mv(dir, dist);
        let range = dbg!(match dir {
            "U" => (Either::Left(s.tail.0), (s.tail.1, new_s.tail.1)),
            "D" => (Either::Left(s.tail.0), (new_s.tail.1, s.tail.1)),
            "R" => (Either::Right(s.tail.1), (s.tail.0, new_s.tail.1)),
            "L" => (Either::Right(s.tail.1), (new_s.tail.1, s.tail.1)),
            _ => unreachable!(),
        });
        for n in range.1 .0..range.1 .1 {
            match range.0 {
                Either::Left(x) => tail_pos.insert((x, n)),
                Either::Right(y) => tail_pos.insert((n, y)),
            };
        }
        s = new_s;
        tail_pos.insert(s.tail);
    }

    tail_pos.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_dist() {
        assert_eq!((0, 0), total_distance((0, 0), (0, 0)));
        assert_eq!((0, 1), total_distance((0, 1), (0, 0)));
        assert_eq!((0, 0), total_distance((0, 1), (0, 1)));
        assert_eq!((2, 1), total_distance((0, 1), (2, 0)));
    }
    // #[test]
    // fn test_follow() {
    //     assert_eq!(0, follow(0, 0));
    //     assert_eq!(0, follow(0, 1));
    //     assert_eq!(1, follow(0, 2));
    //     assert_eq!(1, follow(1, 0));
    //     assert_eq!(1, follow(3, 0));
    // }

    // #[test]
    // fn test_solve_first() {
    //     assert_eq!(13, solve_first(SAMPLE));
    // }
}
