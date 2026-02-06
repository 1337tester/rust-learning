#![allow(dead_code)]
#[macro_use] extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen("127.0.0.1:6767").unwrap();
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn factorial(num:u64) -> u64 {
    if num <= 1 {
        return 1;
    }
    (2..=num).fold(1, |acc, x| acc * x)
    //let fact2 = (2..=num).product::<u64>();
}

fn palindrome(word:&str) -> bool {
    let reverse_word = word.chars().rev().collect::<String>();
    if word == reverse_word {
        true
    } else {
        false
    }
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
        // 21! = 51_090_942_171_709_440_000 > u64::MAX (18_446_744_073_709_551_615)
        // But actually overflows much earlier — u64 can hold up to ~20!
        // 143! is enormously larger → guaranteed overflow
        factorial(20);
        factorial(143);
        // Or any number > 20 will overflow
        // factorial(21);
    }
}
