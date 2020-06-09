use std::collections::HashMap;

use mysql::prelude::*;
use mysql::*;
use r2d2::PooledConnection;
use r2d2_mysql::MysqlConnectionManager;

struct PostTag<'a> {
    post_uuid: &'a str,
    tag: &'a str,
}

pub fn create_post(
    tx: &mut Transaction,
    uuid: &str,
    title: &str,
    link_name: &str,
) -> crate::result::Result<()> {
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

pub fn create_post_tag(tx: &mut Transaction, uuid: &str, tags: &str) -> crate::result::Result<()> {
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
    if post_tags.is_empty() {
        post_tags.push(PostTag {
            post_uuid: &uuid,
            tag: "default",
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

pub fn get_post_list(
    tx: &mut Transaction,
    offset: &Option<u64>,
    limit: &Option<u64>,
    tags: &Option<String>,
) -> crate::result::Result<Vec<crate::types::Post>> {
    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(50);
    let uuid_list: Option<Vec<String>> = tags
        .as_ref()
        .map(|tags| {
            let sql = r"SELECT MAX(id) max_id, `post_uuid`
        FROM post_tag
        WHERE tag IN(?)
        GROUP BY `post_uuid`
        ORDER BY max_id DESC
        LIMIT ? OFFSET ?";
            tx.exec(sql, (tags, limit, offset))
        })
        .transpose()?
        .map(|value: Vec<(usize, String)>| value.into_iter().map(|item| item.1).collect());
    let posts: Vec<crate::types::Post> = match uuid_list {
        Some(uuid_list) => {
            let sql = r"SELECT * FROM `post` WHERE `uuid` in (?)";
            let para = uuid_list.join(",");
            tx.exec(sql, (para,))?
        }
        None => {
            let sql = r"SELECT * FROM `post` ORDER BY id DESC LIMIT ? OFFSET ?";
            tx.exec(sql, (limit, offset))?
        }
    };
    Ok(posts)
}

pub fn get_tags(tx: &mut Transaction) -> crate::result::Result<HashMap<String, usize>> {
    let sql = r"SELECT tag, COUNT(*) tag_count FROM post_tag GROUP BY tag";
    let tag_count: Vec<crate::types::TagCount> = tx.query(sql)?;
    let mut m = HashMap::new();
    for item in tag_count {
        m.insert(item.tag.clone(), item.tag_count);
    }
    Ok(m)
}

pub fn get_post_by_link_name(
    conn: &mut PooledConnection<MysqlConnectionManager>,
    link_name: &str,
) -> crate::result::Result<Option<crate::types::Post>> {
    let sql = r"SELECT * FROM `post` WHERE `link_name` = ?";
    Ok(conn.exec_first(sql, (link_name,))?)
}
