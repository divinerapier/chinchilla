use std::collections::HashMap;

use crate::error::Error;
use crate::types::CreatePostRequest;
use mysql::prelude::*;
use mysql::*;
use std::ops::Index;

struct PostTag<'a> {
    post_uuid: &'a str,
    tag: &'a str,
}

pub fn create_post(
    tx: &mut Transaction,
    uuid: &str,
    title: &str,
    link_name: &str,
) -> crate::error::Result<()> {
    let sql = r"INSERT INTO post (uuid, title, link_name)
    VALUES (:uuid, :title, :link_name)";
    let _result: Vec<String> = tx.exec(
        sql,
        params! {
        "uuid" => &uuid,
        "title" => &title,
        "link_name" => &link_name,
        },
    )?;
    Ok(())
}

pub fn create_post_tag(tx: &mut Transaction, uuid: &str, tags: &str) -> crate::error::Result<()> {
    let sql = r"INSERT INTO post_tag (post_uuid, tag)
       VALUES (:post_uuid, :tag)";
    let mut post_tags = vec![];
    let mut tags_map: std::collections::HashSet<String> = std::collections::HashSet::new();

    for tag in tags.split(',') {
        let tag: &str = tag.trim_start().trim_end();
        if tag.len() == 0 {
            continue;
        }
        if tags_map.contains(tag) {
            continue;
        }
        tags_map.insert(tag.to_string());
        post_tags.push(PostTag {
            post_uuid: &uuid,
            tag: tag,
        });
    }
    tx.exec_batch(
        sql,
        post_tags.iter().map(|p| {
            params! {
                "post_uuid" => p.post_uuid,
                "tag" => p.tag,
            }
        }),
    )?;
    Ok(())
}

struct TagCount {
    tag: String,
    tag_count: usize,
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

pub fn get_tags(tx: &mut Transaction) -> crate::error::Result<HashMap<String, usize>> {
    let sql = r"SELECT tag, COUNT(*) tag_count FROM post_tag GROUP BY tag";
    let tag_count: Vec<TagCount> = tx.query(sql)?;
    let mut m = HashMap::new();
    for item in tag_count {
        m.insert(item.tag.clone(), item.tag_count);
    }
    Ok(m)
}
