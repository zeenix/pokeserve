use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use serde::Serialize;
use serde_json::to_string;

#[derive(Debug, Serialize)]
struct ShakepeareanPokemon {
    name: String,
    description: String,
}

mod poke;
mod shakespeare;
mod error;

use crate::error::Error;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    let response = Response::builder();
    let uri = req.uri();
    let path = uri.path();

    let parts: Vec<&str> = path.split("/").into_iter().collect();
    if parts.len() != 3 || parts[0] != "" || parts[1] != "pokemon" || parts[2] == "" {
        return Ok(response
            .status(StatusCode::BAD_REQUEST)
            .body("Invalid path".into()).unwrap())
    }

    // FIXME: Very inefficient to create the clients for each request.
    let desc = poke::Poke::new().fetch_pokemon(parts[2]).await?;
    let desc = desc.replace("\n", " ");
    let translated = shakespeare::Shakepeare::new().translate(&desc).await?;
    let pokemon_desc = ShakepeareanPokemon {
        name: parts[2].to_string(),
        description: translated,
    };
    let json = to_string(&pokemon_desc)?;

    Ok(response
       .status(StatusCode::OK)
       .body(json.into())?)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Error>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
