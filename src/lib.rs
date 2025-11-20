pub mod math_utils;
// pub use math_utils::*;  --> re-export
use crate::math_utils::*;

pub fn add_numbers(left: i32, right: i32) -> i32 {
    add(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_numbers(2, 2);
        assert_eq!(result, 4);
    }
}
