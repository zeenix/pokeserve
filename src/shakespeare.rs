use serde::Deserialize;
use reqwest::{Client, Result};

#[derive(Deserialize)]
struct Translation {
   contents: TranslationContent,
}

#[derive(Deserialize)]
struct TranslationContent {
   translated: String,
}

pub struct Shakepeare {
    client: Client,
}

impl Shakepeare {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn translate(&self, text: &str) -> Result<String> {
        let response = self
            .client
            .get("http://api.funtranslations.com/translate/shakespeare.json")
            .query(&[("text", text)])
            .send()
            .await?
            .error_for_status()?;

        let translated = response.json::<Translation>().await?;

        Ok(translated.contents.translated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn shakespeare() {
        let rt = Runtime::new().unwrap();
        let shakes = Shakepeare::new();

        let text = "You gave Mr. Tim a hearty meal, but unfortunately what he ate made him die.";
        let translation = "Thee did giveth mr. Tim a hearty meal,  but unfortunately what he did \
                           doth englut did maketh him kicketh the bucket.";

        let translated = rt.block_on(shakes.translate(text)).unwrap();
        assert_eq!(translated, translation);
    }
}
