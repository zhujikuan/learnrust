//! 这是一个逻辑门模拟器程序，用于演示编写单元测试和集成测试
//! unit tests and integration tests
//!

// logic_gates/src/lib.rs

/// 实现一个逻辑门and， 将两位作为输入，并返回一个位作为输出
/// bit as output
pub fn and(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 1) => 1,
        _ => 0,
    }
}

/// 实现一个逻辑门xor， 将两位作为输入，并返回一个位作为输出
pub fn xor(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 0) => 1,
        (0, 1) => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::{and, xor};
    #[test]
    fn test_and() {
        assert_eq!(1, and(1, 1));
        assert_eq!(0, and(0, 1));
        assert_eq!(0, and(1, 0));
        assert_eq!(0, and(0, 0));
    }
    #[test]
    fn test_xor() {
        assert_eq!(1, xor(1, 0));
        assert_eq!(0, xor(0, 0));
        assert_eq!(0, xor(1, 1));
        assert_eq!(1, xor(0, 1));
    }
}
