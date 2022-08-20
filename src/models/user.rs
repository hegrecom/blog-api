use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn authenticate(&self, password: &str) -> bool {
        self.password == password
    }
}

