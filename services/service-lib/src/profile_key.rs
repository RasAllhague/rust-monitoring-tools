use rocket::{
    error,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use rocket_db_pools::Connection;

use crate::{database::MonitoringDb, models::device_profiles::DeviceProfile};

pub struct ProfileKey<'r>(&'r str);

#[derive(Debug)]
pub enum ProfileKeyError {
    MissingKey,
    InvalidKey,
    DbAccessError,
    IdNotPresent,
    InvalidId,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ProfileKey<'r> {
    type Error = ProfileKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let r = req.guard::<Connection<MonitoringDb>>().await;
        let mut conn = match r {
            rocket::outcome::Outcome::Success(s) => s,
            _ => {
                error!("Failed to get database!");
                return Outcome::Failure((
                    Status::InternalServerError,
                    ProfileKeyError::DbAccessError,
                ));
            }
        };

        let id = match req.param::<u32>(1) {
            Some(res) => match res {
                Ok(i) => i,
                Err(why) => {
                    error!("Failed to get id: {why}!");
                    return Outcome::Failure((Status::BadRequest, ProfileKeyError::IdNotPresent));
                }
            },
            None => {
                error!("Failed to get id!");
                return Outcome::Failure((Status::BadRequest, ProfileKeyError::IdNotPresent));
            }
        };

        if let Ok(profile) = DeviceProfile::get(&mut *conn, id as i32).await {
            if let Some(profile) = profile {
                /// Returns true if `key` is a valid API key string.
                fn is_valid(key: &str, valid_key: &str) -> bool {
                    key == valid_key
                }

                return match req.headers().get_one("x-profile-key") {
                    None => Outcome::Failure((Status::BadRequest, ProfileKeyError::InvalidKey)),
                    Some(key) if is_valid(key, &profile.profile_key) => {
                        Outcome::Success(ProfileKey(key))
                    }
                    Some(_) => Outcome::Failure((Status::BadRequest, ProfileKeyError::InvalidKey)),
                };
            }

            return Outcome::Failure((Status::BadRequest, ProfileKeyError::InvalidId));
        }

        Outcome::Failure((Status::BadRequest, ProfileKeyError::IdNotPresent))
    }
}
