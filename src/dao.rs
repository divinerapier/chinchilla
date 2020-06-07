use crate::error::Error;
use crate::types::CreatePostRequest;
use mysql::prelude::*;
use mysql::*;

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

pub fn create_post_tag(tx: &mut Transaction, uuid: &str, tags: &str)  -> crate::error::Result<()>{
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

