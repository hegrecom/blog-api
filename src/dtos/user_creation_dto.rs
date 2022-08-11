use diesel::Insertable;
use crate::schema::users;

#[derive(Insertable)]
#[table_name="users"]
pub struct UserCreationDto {
    pub email: String,
    pub password: String,
}

