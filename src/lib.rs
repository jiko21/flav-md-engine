mod util;

pub fn test() -> i32 {
    1 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let result = test();
        assert_eq!(result, 2);
    }
}
