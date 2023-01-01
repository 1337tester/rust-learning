extern crate base64;

fn rot13(data: String) -> String {
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

fn decodecypher(cypher:&str) -> String {
    // first rot13
    let unrot13: String = rot13(cypher.to_string());
    // println!("Unrot13 - {}", unrot13);

    //base64 decode
    let bytes = base64::decode(&unrot13).unwrap();
    let string_utf8_lossy = String::from_utf8_lossy(&bytes);
    // println!("decode base64: {}", string_utf8_lossy);

    
    let last_unrot13: String = rot13(string_utf8_lossy.to_string());
    // println!("Decoded plaintext - {}", last_unrot13);
    last_unrot13
}

fn main() {
    println!("Decoded plaintext - {}", decodecypher("M3I6r2IbMzq9"));
}