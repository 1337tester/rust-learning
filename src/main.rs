#![allow(dead_code)]
#[macro_use] extern crate nickel;
extern crate base64;
extern crate log;
extern crate env_logger;
mod functions;
mod micros;
use crate::micros::microser;

fn main() {
    microser();
}
