pub struct Example {
    pub input: String,
    pub output: String,
}

impl Example {
    pub fn into_string(&self) -> String {
        format!("Input: {} -> Output: {}", self.input, self.output)
    }
}
