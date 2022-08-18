use diesel::Insertable;
use serde::Deserialize;
use crate::schema::users;

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct UserCreationDto {
    pub email: String,
    pub password: String,
}

