fn main() {
    let a = [1, 2, 3];

    // the sum of all of the elements of the array
    let sum = a.iter().fold(0, |acc, x| acc + x);
    println!("sum is: {sum}");

    let num = 6;
    println!("factorial of {num} is {}", (2..=num).fold(1, |acc, x| acc * x));
    println!("Factorial (different logic) of {num} is {}", (2..=num).product::<u64>());
}