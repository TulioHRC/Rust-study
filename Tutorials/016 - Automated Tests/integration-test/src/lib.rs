pub fn add_two(a: usize) -> usize {
  a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_with_zero() {
        let result = add_two(0);
        assert_eq!(result, 2);
    }

    #[test]
    fn add_two_with_ten() {
        let result = add_two(10);
        assert_eq!(result, 12);
    }
}