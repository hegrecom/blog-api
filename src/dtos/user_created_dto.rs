use chrono::NaiveDateTime;
use serde::Serialize;

use crate::models::User;

#[derive(Serialize)]
pub struct UserCreatedDto {
    pub id: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<User> for UserCreatedDto {
    fn from(user: User) -> Self {
        UserCreatedDto { 
            id: user.id, 
            email: user.email, 
            created_at: user.created_at, 
            updated_at: user.updated_at 
        }
    }
}

