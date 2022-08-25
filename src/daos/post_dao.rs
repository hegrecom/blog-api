use diesel::prelude::*;

use crate::dtos::PostDto;
use crate::exceptions::Exceptions;
use crate::models::Post;
use crate::schema::posts::dsl::*;

pub struct PostDao<'a> {
    connection: &'a MysqlConnection
}

impl<'a> PostDao<'a> {
    pub fn new(connection: &'a MysqlConnection) -> Self {
        PostDao { connection }
    }

    pub fn create(&self, post_dto: PostDto) -> Result<Post, Exceptions> {
        Ok(diesel::insert_into(posts)
            .values(&post_dto)
            .execute(self.connection)
            .and_then(|_| {
                posts.order(id.desc()).first(self.connection)
            })?)
    }
}

