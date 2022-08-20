use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserSigningInDto {
    pub email: String,
    pub password: String,
}

