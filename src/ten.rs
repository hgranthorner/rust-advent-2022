pub const SAMPLE: &str = "noop
addx 3
addx -5";

pub enum Instruction {
    NoOp,
    Add(i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if value == "noop" {
            Self::NoOp
        } else {
            let val = value.split(' ').last().unwrap().parse::<i32>().unwrap();
            Self::Add(val)
        }
    }
}

pub fn should_record(clock_count: usize) -> bool {
    clock_count == 20 || (clock_count as i32 - 20) % 40 == 0
}

pub fn solve_first(input: &str) -> i32 {
    let mut clock_count = 1;
    let mut x_val: i32 = 1;
    let mut record = Vec::<i32>::new();
    for instr in input.lines().into_iter().map(Instruction::from) {
        if should_record(clock_count) {
            let v = x_val * clock_count as i32;
            record.push(v);
        }
        match instr {
            Instruction::NoOp => {
                clock_count += 1;
            }
            Instruction::Add(val) => {
                clock_count += 1;
                if should_record(clock_count) {
                    let v = x_val * clock_count as i32;
                    record.push(v);
                }
                clock_count += 1;
                x_val += val;
            }
        }
    }

    record.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_first() {
        assert_eq!(13140, solve_first(SAMPLE_2));
        assert_eq!(14540, solve_first(INPUT));
    }
}

const SAMPLE_2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

const INPUT: &str = "addx 2
addx 3
addx 3
addx -2
addx 4
noop
addx 1
addx 4
addx 1
noop
addx 4
addx 1
noop
addx 2
addx 5
addx -28
addx 30
noop
addx 5
addx 1
noop
addx -38
noop
noop
noop
noop
addx 5
addx 5
addx 3
addx 2
addx -2
addx 2
noop
noop
addx -2
addx 12
noop
addx 2
addx 3
noop
addx 2
addx -31
addx 32
addx 7
noop
addx -2
addx -37
addx 1
addx 5
addx 1
noop
addx 31
addx -25
addx -10
addx 13
noop
noop
addx 18
addx -11
addx 3
noop
noop
addx 1
addx 4
addx -32
addx 15
addx 24
addx -2
noop
addx -37
noop
noop
noop
addx 5
addx 5
addx 21
addx -20
noop
addx 6
addx 19
addx -5
addx -8
addx -22
addx 26
addx -22
addx 23
addx 2
noop
noop
noop
addx 8
addx -10
addx -27
addx 33
addx -27
noop
addx 34
addx -33
addx 2
addx 19
addx -12
addx 11
addx -20
addx 12
addx 18
addx -11
addx -14
addx 15
addx 2
noop
addx 3
addx 2
noop
noop
noop
addx -33
noop
addx 1
addx 2
noop
addx 3
addx 4
noop
addx 1
addx 2
noop
noop
addx 7
addx 1
noop
addx 4
addx -17
addx 18
addx 5
addx -1
addx 5
addx 1
noop
noop
noop
noop";
