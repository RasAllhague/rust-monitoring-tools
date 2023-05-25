use chrono::{NaiveDateTime, NaiveDate};
use serenity::model::{
    prelude::{
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
        ChannelId,
    },
    user::User,
};

#[derive(Debug)]
pub enum ParserError {
    InvalidType(String),
    InvalidAmount(String, usize),
    NotFound(String),
}

pub struct NamedOptionParser;

impl NamedOptionParser {
    pub fn parse_string(options: &[CommandDataOption], name: &str) -> Result<Option<String>, ParserError> {
        let options: Vec<&CommandDataOption> =
            options.into_iter().filter(|x| x.name == name).collect();

        if options.len() == 0 {
            return Ok(None)
        }

        if options.len() > 1 {
            return Err(ParserError::InvalidAmount(name.to_string(), options.len()));
        }

        if let Some(value) = options[0].resolved.as_ref() {
            if let CommandDataOptionValue::String(data) = value {
                return Ok(Some(data.clone()));
            }
        }

        Ok(None)
    }
}

pub struct PositionalOptionParser;

impl PositionalOptionParser {
    pub fn parse_user(
        &self,
        options: &[CommandDataOption],
        index: usize,
    ) -> Result<User, ParserError> {
        if let Some(option) = options.get(index) {
            if let Some(value) = option.resolved.as_ref() {
                if let CommandDataOptionValue::User(data, _) = value {
                    return Ok(data.clone());
                }
            }
        }

        return Err(ParserError::NotFound(format!("index: {index}")));
    }

    pub fn parse_date(options: &[CommandDataOption]) -> Result<NaiveDateTime, ParserError> {
        let date_parts: Result<Vec<i64>, ParserError> = (0..3)
            .map(|x| PositionalOptionParser::parser_integer(options, x))
            .collect();

        let date_parts = date_parts.expect("User input expected.");

        if let Some(date) = NaiveDate::from_ymd_opt(
            date_parts[2] as i32,
            date_parts[1] as u32,
            date_parts[0] as u32,
        ) {
            if let Some(date) = date.and_hms_opt(0, 0, 0) {
                return Ok(date);
            }
        }

        Err(ParserError::InvalidType(String::from("Not a valid date.")))
    }

    pub fn parse_channel_id(
        options: &[CommandDataOption],
        index: usize,
    ) -> Result<ChannelId, ParserError> {
        if let Some(option) = options.get(index) {
            if let Some(value) = option.resolved.as_ref() {
                if let CommandDataOptionValue::Channel(data) = value {
                    return Ok(data.id);
                }
            }
        }

        Err(ParserError::NotFound(format!("index:{index}!")))
    }

    pub fn parser_integer(options: &[CommandDataOption], index: usize) -> Result<i64, ParserError> {
        if let Some(option) = options.get(index) {
            if let Some(value) = option.resolved.as_ref() {
                if let CommandDataOptionValue::Integer(data) = value {
                    return Ok(*data);
                }
            }
        }

        Err(ParserError::NotFound(format!("index: {index}")))
    }

    pub fn parse_string(
        options: &[CommandDataOption],
        index: usize,
    ) -> Result<String, ParserError> {
        if let Some(option) = options.get(index) {
            if let Some(value) = option.resolved.as_ref() {
                if let CommandDataOptionValue::String(data) = value {
                    return Ok(data.clone());
                }
            }
        }

        Err(ParserError::NotFound(format!("index: {index}")))
    }
}
