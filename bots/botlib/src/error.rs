use monitoring_core::client::ClientError;

use crate::parser::ParserError;


#[derive(Debug)]
pub enum CommandError {
    Client(ClientError),
    Parser(ParserError),
    Serenity(serenity::Error),
    IO(std::io::Error),
}

impl From<ClientError> for CommandError {
    fn from(value: ClientError) -> Self {
        CommandError::Client(value)
    }
}

impl From<ParserError> for CommandError {
    fn from(value: ParserError) -> Self {
        CommandError::Parser(value)
    }
}

impl From<serenity::Error> for CommandError {
    fn from(value: serenity::Error) -> Self {
        CommandError::Serenity(value)
    }
}

impl From<std::io::Error> for CommandError {
    fn from(value: std::io::Error) -> Self {
        CommandError::IO(value)
    }
}