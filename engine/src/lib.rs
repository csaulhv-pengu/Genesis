/// Genesis Engine
///
/// Core application lifecycle.
pub struct App {
    running: bool,
}

impl App {
    /// Create a new application instance
    pub fn new() -> Self {
        Self { running: true }
    }

    /// Run the main application loop
    pub fn run(&mut self) {
        while self.running {
            // Engine update step (placeholder)
            // Rendering step (placeholder)

            // For now, exit immediately
            self.exit();
        }
    }

    /// Stop the application
    pub fn exit(&mut self) {
        self.running = false;
    }
}