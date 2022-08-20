use chrono::NaiveDateTime;
use diesel::Insertable;

use crate::schema::user_tokens;

#[derive(Insertable)]
#[table_name="user_tokens"]
pub struct UserTokenCreationDto {
    pub user_id: i32,
    pub token: String,
    pub expires_at: NaiveDateTime,
}

