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
use futures_util::TryStreamExt;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};


async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    info!("Microservice received a request: {:?}", req);
    match (req.method(), req.uri().path()) {
        // GET request to / with instructions for POST request
        (&Method::GET, "/") => {
            Ok(Response::new(Body::from("You GET what you ask for: `curl localhost:3000/echo -XPOST -d 'hello world'`")))
        }
        // GET request to echo
        (&Method::GET, "/echo") => {
            Ok(Response::new(Body::from("Echo GET")))
        }
        // POST request to echo, returning the same
        (&Method::POST, "/echo") => {
            Ok(Response::new(req.into_body()))
        }
        // Convert to uppercase before sending back to client using a stream.
        (&Method::POST, "/echo/uppercase") => {
            let chunk_stream = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            Ok(Response::new(Body::wrap_stream(chunk_stream)))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            error!("NOT_FOUND: {:?}", not_found);            
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();

    // Binding the address and port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `echo` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(echo))
    });
    info!("Running microservice at {}", addr);

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    server.await?;

    Ok(())
}
