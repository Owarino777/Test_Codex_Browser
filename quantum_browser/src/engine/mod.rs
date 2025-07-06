pub mod network;
pub mod renderer;
pub mod security;

pub struct QuantumEngine {
    pub renderer: renderer::Renderer,
    pub security_manager: security::SecurityManager,
    pub network_stack: network::NetworkStack,
}

impl QuantumEngine {
    pub fn new() -> Self {
        Self {
            renderer: renderer::Renderer::default(),
            security_manager: security::SecurityManager::default(),
            network_stack: network::NetworkStack::default(),
        }
    }

    /// Fetch content from a URL and render it.
    pub async fn browse(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.security_manager.is_allowed(url) {
            return Err("Blocked URL".into());
        }
        let content = self.network_stack.fetch(url).await?;
        self.renderer.render(&content);
        Ok(())
    }
}
