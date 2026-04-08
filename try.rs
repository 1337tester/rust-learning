use learning_rust::{factorial, palindrome};
use chrono::Local;

fn main() {
    let date = Local::now().format("%B %e, %Y");
    println!("Hello from script! Today is {}.", date);

    let v = vec![1,2,3,4];
    summing::sum_vec(v.clone());

    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("Alternative calc of sum is: {sum}");

    let num = 6;
    println!("Factorial of {num} is: {}", factorial(num));

    let word1 = "palindrome";
    println!("Is word '{word1}' a palindrome? -> {}", palindrome(word1));
    let word2 = "abba";
    println!("Is word '{word2}' a palindrome? -> {}", palindrome(word2));

    let word3 = "anomochronomic";
    println!("Word {word3} contains {} letters", count_letters(word3));
}

fn count_letters(word:&str) -> usize {
    word.len()
}

mod summing {
    pub fn sum_vec(v:Vec<i32>) -> () {
        println!("Sum = {}", v.iter().sum::<i32>());
    }
}
