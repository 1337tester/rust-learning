#!/usr/bin/env -S cargo +nightly -Zscript

//! This is a Rust script
fn main() {
    println!("Hello from script! Today is {}.", "February 2, 2026");

    let v = vec![1,2,3,4];
    summing::sum_vec(v.clone());

    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("Alternative calc of sum is: {sum}");

    let num = 6;
    factorial(num);

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
