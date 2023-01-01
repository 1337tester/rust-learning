use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use log::info;
use log::error;

async fn get_call(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    info!("Microservice received a request: {:?}", _req);
    Ok(Response::new(Body::from("Hello World!")))
}

pub async fn microser(){
    env_logger::init();
    // We'll bind to 127.0.0.1:3000
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(get_call)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    info!("Running microservice at {}", addr);
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }    
}