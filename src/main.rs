use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

type ResponseError = Box<dyn std::error::Error + Send + Sync>;
type ServerResponse = Result<Response<Body>, ResponseError>;

async fn respond(request: Request<Body>) -> ServerResponse {
    let mut response = Response::new(Body::empty());
    match (request.method(), request.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("homepage");
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response)
}

#[tokio::main]
async fn main() {
    let port_string = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => String::from("8080"),
    };
    let port = match port_string.parse::<u16>() {
        Ok(i) => i,
        Err(_) => 8080,
    };
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let service = hyper::service::make_service_fn(|_| {
        async { Ok::<_, ResponseError>(hyper::service::service_fn(respond)) }
    });
    let server = Server::bind(&address).serve(service);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
