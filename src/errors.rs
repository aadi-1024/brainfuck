#[derive(Debug)]
pub struct EOF {
}

impl std::fmt::Display for EOF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("reached EOF\n")
    }
}

impl std::error::Error for EOF {}