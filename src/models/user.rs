use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

