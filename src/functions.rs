use nickel::Nickel;

fn say_hello() -> &'static str {
    "Hello dear world (inside function)!"
}

pub fn server() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            say_hello()
        }
    });

    server.listen("127.0.0.1:6767").unwrap();
}

pub fn experiment() {
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

pub fn rot13(data: String) -> String {
    // Create new string.
    let mut result = String::with_capacity(data.len());
    // Loop over bytes.
    for c in data.bytes() {
        // Adjust the byte.
        let mut adjusted = c;
        if c >= b'a' && c <= b'z' {
            if c > b'm' {
                adjusted -= 13;
            } else {
                adjusted += 13;
            }
        } else if c >= b'A' && c <= b'Z' {
            if c > b'M' {
                adjusted -= 13;
            } else {
                adjusted += 13;
            }
        }
        // Push to new string.
        result.push(adjusted as char);
    }
    result
}

pub fn decodecypher(cypher:&str) -> () {
    // first rot13
    let unrot13: String = rot13(cypher.to_string());
    // println!("Unrot13 - {}", unrot13);

    //base64 decode
    let bytes = base64::decode(&unrot13).unwrap();
    let string_utf8_lossy = String::from_utf8_lossy(&bytes);
    // println!("decode base64: {}", string_utf8_lossy);

    
    let last_unrot13: String = rot13(string_utf8_lossy.to_string());
    println!("Decoded plaintext - {}", last_unrot13);
}