pub fn digit_value(digit: char) -> u64 {
    digit as u64 - '0' as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_value() {
        assert_eq!(0, digit_value('0'));
        assert_eq!(1, digit_value('1'));
        assert_eq!(2, digit_value('2'));
        assert_eq!(3, digit_value('3'));
        assert_eq!(4, digit_value('4'));
        assert_eq!(5, digit_value('5'));
        assert_eq!(6, digit_value('6'));
        assert_eq!(7, digit_value('7'));
        assert_eq!(8, digit_value('8'));
        assert_eq!(9, digit_value('9'));
    }
}
