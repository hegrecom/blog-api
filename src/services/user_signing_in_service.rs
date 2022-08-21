use bcrypt::BcryptError;
use chrono::{Utc, Duration};
use diesel::MysqlConnection;
use uuid::Uuid;

use crate::daos::{UserDao, UserTokenDao};
use crate::dtos::{UserSigningInDto, UserTokenCreationDto};
use crate::exceptions::Exceptions;
use crate::models::UserToken;

pub struct UserSigningInService<'a> {
    connection: &'a MysqlConnection,
    user_dto: UserSigningInDto,
    password_salt: &'a str,
}

impl<'a> UserSigningInService<'a> {
    pub fn new(connection: &'a MysqlConnection, user_dto: UserSigningInDto, password_salt: &'a str) -> Self {
        UserSigningInService { connection, user_dto, password_salt }
    }

    pub fn run(&self) -> Result<UserToken, Exceptions> {
        let user_dao = UserDao::new(self.connection);

        if let Some(user) = user_dao.find_by_email(&self.user_dto.email)? {
            let encrypted_password: String = Self::encrypt_password(&self.user_dto.password, self.password_salt.as_bytes().try_into().unwrap())?;
            if user.authenticate(&encrypted_password) {
                let token = Uuid::new_v4().to_string();
                let expires_at = (Utc::now() + Duration::weeks(2)).naive_utc();
                let user_token_dto = UserTokenCreationDto { user_id: user.id, token, expires_at };
                let user_token_dao = UserTokenDao::new(self.connection);
                let user_token = user_token_dao.create(user_token_dto)?;

                Ok(user_token)
            } else {
                Err(Exceptions::Unauthorized { message: "email or password is incorrect" })
            }
        } else {
            Err(Exceptions::Unauthorized { message: "email or password is incorrect" })
        }
    }

    fn encrypt_password(password: &str, salt: &[u8; 16]) -> Result<String, BcryptError> {
        match bcrypt::hash_with_salt(password.to_owned(), 10, *salt) {
            Ok(result) => { 
                Ok(result.to_string())
            },
            Err(error) => { Err(error) },
        }
    }
}

