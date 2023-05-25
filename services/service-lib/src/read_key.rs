use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

pub struct ReadKey<'r>(&'r str);

pub struct ReadKeyVault(String);

#[derive(Debug)]
pub enum ReadKeyError {
    MissingServerKey,
    MissingRequestKey,
    Invalid,
}

impl ReadKeyVault {
    pub fn new(key: &str) -> Self {
        ReadKeyVault(String::from(key))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ReadKey<'r> {
    type Error = ReadKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let api_key = req.rocket().state::<ReadKeyVault>();

        if let Some(valid_key) = api_key {
            /// Returns true if `key` is a valid read key string.
            fn is_valid(key: &str, valid_key: &str) -> bool {
                key == valid_key
            }

            return match req.headers().get_one("x-read-key") {
                None => Outcome::Failure((Status::BadRequest, ReadKeyError::MissingRequestKey)),
                Some(key) if is_valid(key, &valid_key.0) => Outcome::Success(ReadKey(key)),
                Some(_) => Outcome::Failure((Status::BadRequest, ReadKeyError::Invalid)),
            };
        }

        Outcome::Failure((Status::InternalServerError, ReadKeyError::MissingServerKey))
    }
}
