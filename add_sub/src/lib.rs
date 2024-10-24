pub fn add(arg1: &i32, arg2: &i32) -> i32 {
    arg1 + arg2
}

pub fn subtract(arg1: &i32, arg2: &i32) -> i32 {
    arg1 - arg2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(&1, &2), 3);
        assert_eq!(add(&5, &3), 8);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(&5, &3), 2);
        assert_eq!(subtract(&10, &7), 3);
    }
}
