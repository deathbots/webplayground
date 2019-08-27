
extern crate hyper;
extern crate futures;

// based on https://github.com/hyperium/hyper/blob/0.12.x/examples/echo.rs
use hyper::rt::Future;
use futures::future;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};


type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;


fn index(_req: Request<Body>) -> Body {
    Body::from(include_str!("../ui/dist/index.html").to_string())
}

fn bundle(_req: Request<Body>) -> Body {
    Body::from(include_str!("../ui/dist/bundle.js").to_string())
}

fn router(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = index(req);
        }

        (&Method::GET, "/bundle.js") => {
            *response.body_mut() = bundle(req);
        }

        // The 404 Not Found route...
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

fn main() {

    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    // A `Service` is needed for every connection, so this
    // creates one
    let new_svc = || {
        service_fn(router)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    hyper::rt::run(server);

}
