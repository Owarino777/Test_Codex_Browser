pub struct NetworkStack {
    client: reqwest::Client,
}

impl NetworkStack {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Fetch a URL and return the response body as text.
    pub async fn fetch(&self, url: &str) -> Result<String, reqwest::Error> {
        let response = self.client.get(url).send().await?;
        response.text().await
    }
}

impl Default for NetworkStack {
    fn default() -> Self {
        Self::new()
    }
}
