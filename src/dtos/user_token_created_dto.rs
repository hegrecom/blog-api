use chrono::NaiveDateTime;
use serde::Serialize;

use crate::models::UserToken;

#[derive(Serialize)]
pub struct UserTokenCreatedDto {
    pub token: String,
    pub expires_at: NaiveDateTime,
}

impl From<UserToken> for UserTokenCreatedDto {
    fn from(user_token: UserToken) -> Self {
        UserTokenCreatedDto { token: user_token.token, expires_at: user_token.expires_at }
    }
}

