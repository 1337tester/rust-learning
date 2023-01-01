fn experiment() {
    // iterating strings
    for character in "Здравствуйте".chars() {
        println!("{character}");
    }

    // string slicing
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    // iterator example
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // iterator example
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // another iterator example
    let a = vec![1, 2, 3];
    let a_iter = a.iter();
    for val in a_iter {
    println!("The value is {}", val);
}
}

fn main() {
    experiment();
}