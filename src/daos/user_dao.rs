use std::error::Error;

use diesel::prelude::*;

use crate::dtos::UserCreationDto;
use crate::models::User;
use crate::schema::users::dsl::*;

pub struct UserDao<'a> {
    connection: &'a MysqlConnection,
}

impl<'a> UserDao<'a> {
    pub fn new(connection: &'a MysqlConnection) -> Self {
        UserDao { connection }
    }

    pub fn create(&self, user_dto: UserCreationDto) -> Result<User, Box<dyn Error + Send + Sync>> {
        Ok(self.connection.transaction::<_, diesel::result::Error, _>(move || {
            diesel::insert_into(users)
                .values(&user_dto)
                .execute(self.connection)
                .and_then(|_| {
                    users.filter(email.eq(&user_dto.email)).first(self.connection)
                })
        })?)
    }
}

