pub mod engine;

pub struct Browser {
    engine: engine::QuantumEngine,
}

impl Browser {
    pub fn new() -> Self {
        Self {
            engine: engine::QuantumEngine::new(),
        }
    }

    pub async fn open(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.engine.browse(url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn browser_fetches_content() {
        let mut server = Server::new_async().await;
        let mock = server.mock("GET", "/").with_body("Hello world").create();

        let browser = Browser::new();
        let url = format!("{}/", server.url());
        browser.open(&url).await.unwrap();

        mock.assert();
    }
}
