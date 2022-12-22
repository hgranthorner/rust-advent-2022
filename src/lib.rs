#![allow(dead_code, unused_variables)]

pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;
pub mod six;
pub mod seven;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
