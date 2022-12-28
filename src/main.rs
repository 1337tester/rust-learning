#![allow(dead_code)]
#[macro_use] extern crate nickel;
extern crate base64;
mod functions;

fn main() {
    // functions::experiment();
    println!("Decoded plaintext - {}", functions::decodecypher("M3I6r2IbMzq9"));
    // functions::server();
}