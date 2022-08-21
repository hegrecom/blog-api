use bcrypt::BcryptError;
use diesel::MysqlConnection;

use crate::daos::UserDao;
use crate::dtos::UserCreationDto;
use crate::exceptions::Exceptions;
use crate::models::User;

pub struct UserCreationService<'a> {
    connection: &'a MysqlConnection,
    user_dto: UserCreationDto,
    password_salt: &'a str,
}

impl<'a> UserCreationService<'a> {
    pub fn new(connection: &'a MysqlConnection, user_dto: UserCreationDto, password_salt: &'a str) -> Self {
        UserCreationService { connection, user_dto, password_salt, }
    }

    pub fn run(&self) -> Result<User, Exceptions> {
        let user_dto = UserCreationDto {
            email: self.user_dto.email.to_owned(),
            password: Self::encrypt_password(&self.user_dto.password, self.password_salt.as_bytes().try_into().unwrap())?,
        };

        UserDao::new(self.connection).create(user_dto)
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



