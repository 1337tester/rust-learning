#![allow(dead_code)]
#[macro_use] extern crate nickel;
extern crate base64;
extern crate log;
extern crate env_logger;
mod functions;
mod micros;
// use crate::micros::microser;

use log::{info, error};
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn get_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    info!("Microservice received a request: {:?}", _req);
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    env_logger::init();
    // Binding the address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(get_request))
    });
    info!("Running microservice at {}", addr);

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
