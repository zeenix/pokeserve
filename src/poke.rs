use serde::Deserialize;
use reqwest::{Client, Result};

pub struct Poke {
    client: Client,
}

#[derive(Deserialize)]
struct Species {
    flavor_text_entries: Vec<FlavorTextEntry>,
}

#[derive(Deserialize)]
struct FlavorTextEntry {
    flavor_text: String,
    language: Language,
}

#[derive(Deserialize)]
struct Language {
    name: String,
}

impl Poke {
    pub fn new() -> Self {
        Poke {
            client: Client::new(),
        }
    }

    pub async fn fetch_pokemon(&self, name: &str) -> Result<String> {
        // FIXME: This is the best API endpoint I could find that could fetch me the description.
        // Unfortunately, it fetches description of all flavors of the species and not all of them
        // are great descriptions. Let me know I missed some API end point that'd be more
        // appropriate here.
        let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}/?language=en", name);
        let species = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Species>()
            .await?;

        for entry in species.flavor_text_entries.into_iter() {
            if entry.language.name == "en" {
                return Ok(entry.flavor_text)
            }
        }

        // FIXME: Need an error type
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn fetch_pokemon() {
        let rt = Runtime::new().unwrap();
        let poke = Poke::new();

        assert!(rt.block_on(poke.fetch_pokemon("charizard")).is_ok());
    }
}
