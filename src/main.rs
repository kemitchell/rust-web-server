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

const DEFAULT_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    let port;
    let port_env_var = env::var("PORT");
    match port_env_var {
        Ok(port_string) => {
            match port_string.parse::<u16>() {
                Ok(i) => port = i,
                Err(_) => {
                    eprintln!("Error: Could not parse PORT environment variable.");
                    std::process::exit(1);
                }
            };
        }
        Err(_) => port = DEFAULT_PORT,
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
