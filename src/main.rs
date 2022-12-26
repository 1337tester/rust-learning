// #[macro_use] extern crate nickel;

// use nickel::Nickel;

// fn say_hello() -> &'static str {
//     "Hello dear world (inside function)!"
// }

// fn server() {
//     let mut server = Nickel::new();

//     server.utilize(router! {
//         get "**" => |_req, _res| {
//             say_hello()
//         }
//     });

//     server.listen("127.0.0.1:6767").unwrap();
// }
#[macro_use] extern crate nickel;
extern crate base64;
mod functions;
pub use crate::functions::experiment;
pub use crate::functions::server;
pub use crate::functions::rot13;

fn main() {
    // experiment();
    // server();

    // first rot13
    let cypher: &str = "M3I6r2IbMzq9";
    let unrot13: String = rot13(cypher.to_string());
    println!("Unrot13 - {}", unrot13);

    //base64 decode
    let bytes = base64::decode(&unrot13).unwrap();
    let string_utf8_lossy = String::from_utf8_lossy(&bytes);
    println!("decode base64: {}", string_utf8_lossy);

    
    let last_unrot13: String = rot13(string_utf8_lossy.to_string());
    println!("Final plaintext - {}", last_unrot13);


    // let mut buffer = Vec::<u8>::new();
    // base64::decode_config_buf("Z3V6e2VoZmd9", base64::STANDARD, &mut buffer).unwrap();
    // println!("{:?}", buffer);
}