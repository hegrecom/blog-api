use diesel::Insertable;
use crate::schema::posts;

use super::PostCreationDto;

#[derive(Insertable)]
#[table_name="posts"]
pub struct PostDto {
    user_id: i32,
    title: String,
    body: String,
}

impl PostDto {
    pub fn new(user_id: i32, post_creation_dto: PostCreationDto) -> Self {
        PostDto { 
            user_id, 
            title: post_creation_dto.title,
            body: post_creation_dto.body,
        }
    }
}

