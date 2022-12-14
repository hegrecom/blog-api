use diesel::dsl::{exists, select};
use diesel::prelude::*;

use crate::dtos::UserCreationDto;
use crate::exceptions::Exceptions;
use crate::models::User;
use crate::schema::users::dsl::*;

pub struct UserDao<'a> {
    connection: &'a MysqlConnection,
}

impl<'a> UserDao<'a> {
    pub fn new(connection: &'a MysqlConnection) -> Self {
        UserDao { connection }
    }

    pub fn create(&self, user_dto: UserCreationDto) -> Result<User, Exceptions> {
        let user_found: bool = select(exists(users.filter(email.eq(&user_dto.email)))).get_result(self.connection)?;
        if user_found == true {
            return Err(Exceptions::BadRequest { message: "email is already taken".to_string() });
        }

        Ok(self.connection.transaction::<_, diesel::result::Error, _>(move || {
            diesel::insert_into(users)
                .values(&user_dto)
                .execute(self.connection)
                .and_then(|_| {
                    users.filter(email.eq(&user_dto.email)).first(self.connection)
                })
        })?)
    }

    pub fn find_by_email(&self, user_email: &str) -> Result<Option<User>, Exceptions> {
        Ok(users.filter(email.eq(user_email))
            .first::<User>(self.connection).optional()?)
    }
}

