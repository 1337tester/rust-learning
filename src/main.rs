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
pub use crate::functions::decodecypher;
pub use crate::functions::server;
pub use crate::functions::rot13;

fn main() {
    // experiment();
    // server();
    decodecypher("M3I6r2IbMzq9")
}