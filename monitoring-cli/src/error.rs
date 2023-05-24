#[derive(Debug)]
pub enum CliError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}
