fn add(left: isize, right: isize) -> isize{
    return left + right;
} 

#[cfg(test)]
mod tests {
    use super::*; // to use outer scope functions

    #[test]
    fn it_works() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_error() {
        panic!("Error test.")
    }

    #[test]
    fn test_add_zero() {
        let result = add(0, 2);
        assert_eq!(result, 1, "OMG");
    }
}
fn main() {
    println!("Hello, world!");
}
