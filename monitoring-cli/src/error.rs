#[derive(Debug)]
pub enum CliError {
    Serde(serde_json::Error),
    Io(std::io::Error),
}
