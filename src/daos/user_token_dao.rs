use std::error::Error;

use diesel::prelude::*;

use crate::dtos::UserTokenCreationDto;
use crate::models::UserToken;
use crate::schema::user_tokens::dsl::*;

pub struct UserTokenDao<'a> {
    connection: &'a MysqlConnection,
}

impl<'a> UserTokenDao<'a> {
    pub fn new(connection: &'a MysqlConnection) -> Self {
        UserTokenDao { connection }
    }

    pub fn create(&self, user_token_dto: UserTokenCreationDto) -> Result<UserToken, Box<dyn Error + Send + Sync>> {
        Ok(diesel::insert_into(user_tokens)
            .values(&user_token_dto)
            .execute(self.connection)
            .and_then(|_| {
                user_tokens.filter(user_id.eq(&user_token_dto.user_id)).order(id.desc()).first(self.connection)
            })?)
    }
}

