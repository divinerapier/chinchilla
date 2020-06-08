use mysql::chrono::offset::TimeZone;
use mysql::prelude::FromRow;
use mysql::{from_row_opt, from_value, from_value_opt, FromRowError, Row};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub link_name: String,
    pub tags: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: usize,
    pub uuid: String,
    pub title: String,
    pub link_name: String,
    pub content: Option<String>,
    pub created_at: mysql::chrono::DateTime<mysql::chrono::offset::Local>,
    pub updated_at: mysql::chrono::DateTime<mysql::chrono::offset::Local>,
}

#[derive(Debug, Deserialize)]
pub struct GetPostListRequestOptions {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
    pub tag: Option<String>,
}

impl FromRow for Post {
    fn from_row_opt(row: Row) -> std::result::Result<Self, FromRowError>
    where
        Self: Sized,
    {
        if row.len() != 6 {
            return Err(FromRowError(row));
        }
        let id = from_value_opt::<usize>(row.get(0).unwrap()).unwrap();
        let uuid = from_value_opt::<String>(row.get(1).unwrap()).unwrap();
        let title = from_value_opt::<String>(row.get(2).unwrap()).unwrap();
        let link_name = from_value_opt::<String>(row.get(3).unwrap()).unwrap();
        let created_at: mysql::chrono::NaiveDateTime =
            from_value::<mysql::chrono::NaiveDateTime>(row.get(4).unwrap());
        let updated_at: mysql::chrono::NaiveDateTime =
            from_value::<mysql::chrono::NaiveDateTime>(row.get(5).unwrap());

        let created_at = mysql::chrono::Local.timestamp(
            created_at.timestamp(),
            (created_at.timestamp_nanos() / 1_000_000_000i64) as u32,
        );

        let updated_at = mysql::chrono::Local.timestamp(
            updated_at.timestamp(),
            (updated_at.timestamp_nanos() / 1_000_000_000i64) as u32,
        );

        Ok(Post {
            id,
            uuid,
            title,
            link_name,
            content: None,
            created_at,
            updated_at,
        })
    }
}

pub(crate) struct TagCount {
    pub tag: String,
    pub tag_count: usize,
}

impl FromRow for TagCount {
    fn from_row_opt(row: Row) -> std::result::Result<Self, FromRowError>
    where
        Self: Sized,
    {
        if row.len() != 2 {
            return Err(FromRowError(row));
        }
        let (tag, tag_count) = from_row_opt::<(String, usize)>(row)?;
        Ok(TagCount { tag, tag_count })
    }
}
