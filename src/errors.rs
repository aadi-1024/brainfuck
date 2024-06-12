#[derive(Debug)]
pub struct EOF {}

#[derive(Debug)]
pub struct RuntimeErr {
    pub message: String,
}

impl std::fmt::Display for EOF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("reached EOF\n")
    }
}

impl std::fmt::Display for RuntimeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message.as_str())
    }
}

impl std::error::Error for EOF {}
impl std::error::Error for RuntimeErr {}