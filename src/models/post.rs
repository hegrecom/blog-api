use chrono::NaiveDateTime;
use diesel::Queryable;
use diesel_derive_enum::DbEnum;

#[derive(Queryable)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    staus: PostStatus,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(DbEnum, Debug)]
pub enum PostStatus {
    Draft,
    Published,
}

