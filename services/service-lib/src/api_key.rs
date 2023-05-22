use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

pub struct ApiKey<'r>(&'r str);

pub struct ApiKeyVault(String);

#[derive(Debug)]
pub enum ApiKeyError {
    MissingServerKey,
    MissingRequestKey,
    Invalid,
}

impl ApiKeyVault {
    pub fn new(key: &str) -> Self {
        ApiKeyVault(String::from(key))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let api_key = req.rocket().state::<ApiKeyVault>();

        if let Some(valid_key) = api_key {
            /// Returns true if `key` is a valid API key string.
            fn is_valid(key: &str, valid_key: &str) -> bool {
                key == valid_key
            }

            return match req.headers().get_one("x-api-key") {
                None => Outcome::Failure((Status::BadRequest, ApiKeyError::MissingRequestKey)),
                Some(key) if is_valid(key, &valid_key.0) => Outcome::Success(ApiKey(key)),
                Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            };
        }

        Outcome::Failure((Status::InternalServerError, ApiKeyError::MissingServerKey))
    }
}
