pub struct ZeroShotPrompt {
    prompt: String,
}

impl ZeroShotPrompt {
    pub fn new(prompt: String) -> Self {
        Self { prompt }
    }

    pub fn prompt(&self) -> String {
        self.prompt.clone()
    }
}
