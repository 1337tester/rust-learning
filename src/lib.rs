pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn factorial(num:u64) -> u64 {
    if num <= 1 {
        return 1;
    }
    (2..=num).fold(1, |acc, x| acc * x)
}

pub fn palindrome(word:&str) -> bool {
    let reverse_word = word.chars().rev().collect::<String>();
    word == reverse_word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_palindrome() {
        assert_eq!(palindrome("iai"), true); 
        assert_eq!(palindrome("alfongofraerf"), false);
        assert_eq!(palindrome("alfonofla"), true);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(7), 5040);
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_factorial_overflow() {
        factorial(21);
    }
}
