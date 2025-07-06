# Test Codex Browser

This repository experiments with a minimal prototype for a new browser engine written in Rust. The current version fetches remote content asynchronously and renders a short preview on the console. Core components include:

- **QuantumEngine**: High-level orchestrator of renderer, security manager and network stack.
- **Renderer**: Prints a preview of fetched content.
- **SecurityManager**: Basic URL validation.
- **NetworkStack**: Async HTTP client built with `reqwest`.

The project is managed with Cargo. To run the unit tests:

```bash
cargo test
```

This is still an early experiment without a user interface, but it demonstrates asynchronous networking and a simple security check.
Example usage:

```rust
#[tokio::main]
async fn main() {
    let browser = quantum_browser::Browser::new();
    browser.open("https://example.com").await.unwrap();
}
```


