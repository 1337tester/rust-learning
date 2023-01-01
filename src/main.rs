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
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};


// async fn get_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     info!("Microservice received a request: {:?}", _req);
//     Ok(Response::new("You GET what you ask for".into()))
// }

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    info!("Microservice received a request: {:?}", req);
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Ok(Response::new(Body::from("You GET what you ask for: `curl localhost:3000/echo -XPOST -d 'hello world'`")))
        }
        (&Method::POST, "/echo") => {
            Ok(Response::new(req.into_body()))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
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
        Ok::<_, Infallible>(service_fn(echo))
    });
    info!("Running microservice at {}", addr);

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
