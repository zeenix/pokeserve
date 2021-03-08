use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use tokio::sync::oneshot;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
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

async fn run(shutdown_rx: oneshot::Receiver<()>) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Error>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);
    let graceful = server.with_graceful_shutdown(async {
        shutdown_rx.await.ok();
    });

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    // FIXME: Ignoring tx here, hook it up to UNIX signale (SIGINT and SIGTERM) handlers.
    let (_, rx) = oneshot::channel::<()>();

    run(rx).await
}
