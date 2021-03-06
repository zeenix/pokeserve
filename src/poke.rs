use serde::Deserialize;
use reqwest::Client;
use crate::error::Error;

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

    pub async fn fetch_pokemon(&self, name: &str) -> Result<String, Error> {
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
            .error_for_status()?
            .json::<Species>()
            .await?;

        for entry in species.flavor_text_entries.into_iter() {
            if entry.language.name == "en" {
                return Ok(entry.flavor_text)
            }
        }

        Err(Error::MissingPokemon)
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

        rt.block_on(poke.fetch_pokemon("charizard")).unwrap();
    }
}
