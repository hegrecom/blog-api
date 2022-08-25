use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostCreationDto {
    pub title: String,
    pub body: String,
}

