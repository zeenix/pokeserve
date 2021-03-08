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

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use tokio::runtime::Runtime;

    #[test]
    fn e2e() {
        let rt = Runtime::new().unwrap();

        let (tx, rx) = oneshot::channel::<()>();

        // Spawn the server
        rt.spawn(run(rx));

        rt.block_on(run_client());

        // Time to shutdown the server
        let _ = tx.send(());
    }

    async fn run_client() {
        let client = Client::new();

        let expected_pokemon: [ShakepeareanPokemon; 2] = [
            ShakepeareanPokemon {
                name: "charizard".to_string(),
                description: "Spits fire yond is hot enow to melt boulders. Known to cause forest \
                              fires unintentionally.".to_string(),
            },
            ShakepeareanPokemon {
                name: "butterfree".to_string(),
                description: "In hurlyburly,  't flaps its wings at high speed to release highly \
                              toxic dust into the air.".to_string(),
            },
        ];

        for expected in &expected_pokemon {
            let url = format!("http://localhost:3000/pokemon/{}", expected.name);
            let outcome = client
                .get(&url)
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap()
                .json::<ShakepeareanPokemon>()
                .await
                .unwrap();
            assert_eq!(outcome, *expected);
        }
    }
}
