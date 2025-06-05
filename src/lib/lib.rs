pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(2, 3), 5);
        assert_eq!(sum(-1, 1), 0);
        assert_eq!(sum(0, 0), 0);
        assert_eq!(sum(-5, -5), -10);
    }
}
