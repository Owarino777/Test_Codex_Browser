#[derive(Default)]
pub struct Renderer;

impl Renderer {
    /// Render content to the screen. Currently prints a preview.
    pub fn render(&self, content: &str) {
        // Placeholder for WebGPU-based pipeline.
        println!("Rendered content: {:.60}...", content.replace('\n', " "));
    }
}
