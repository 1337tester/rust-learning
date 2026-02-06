extern crate base64;

use base64::engine::general_purpose;
use base64::Engine as _;

fn rot13(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for byte in input.bytes() {
        let adjusted = match byte {
            b'a'..=b'z' => {
                if byte > b'm' {
                    byte - 13
                } else {
                    byte + 13
                }
            }
            b'A'..=b'Z' => {
                if byte > b'M' {
                    byte - 13
                } else {
                    byte + 13
                }
            }
            _ => byte,
        };
        result.push(adjusted as char);
    }

    result
}

fn decode_cypher(cypher: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Step 1: Apply ROT13 first
    let after_rot13 = rot13(cypher);

    // Step 2: Decode base64
    let bytes = general_purpose::STANDARD.decode(&after_rot13)?;

    // Step 3: Convert bytes → string (lossy if invalid UTF-8)
    let decoded_utf8 = String::from_utf8_lossy(&bytes);

    // Step 4: Apply ROT13 again
    let final_text = rot13(&decoded_utf8);

    Ok(final_text)
}

fn main() {
    let cypher = "M3I6r2IbMzq9";

    match decode_cypher(cypher) {
        Ok(plaintext) => {
            println!("Decoded plaintext: {}", plaintext);
        }
        Err(e) => {
            eprintln!("Decoding failed: {}", e);
        }
    }
}
