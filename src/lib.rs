#![allow(dead_code, unused_variables)]
#![feature(anonymous_lifetime_in_impl_trait)]

pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;
pub mod six;
pub mod seven;
pub mod eight;
pub mod nine;
pub mod ten;

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
