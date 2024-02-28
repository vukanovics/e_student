use crate::{
    database::Connection,
    error::Error,
    schema::{comments, discussions},
    user::UserId,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

pub type DiscussionId = u32;

#[derive(Queryable, Serialize, Debug)]
pub struct Discussion {
    id: DiscussionId,
    markdown: Option<String>,
}

impl Discussion {
    pub fn get(connection: &mut Connection, id: DiscussionId) -> Result<Discussion, Error> {
        discussions::table
            .filter(discussions::id.eq(id))
            .limit(1)
            .first(connection)
            .map_err(Error::from)
    }

    pub fn create(connection: &mut Connection) -> Result<DiscussionId, Error> {
        diesel::insert_into(discussions::table)
            .default_values()
            .execute(connection)
            .map_err(Error::from)
            .map(|_| ())?;

        Ok(
            diesel::select(crate::database::last_insert_id()).first::<i32>(connection)?
                as DiscussionId,
        )
    }
}

pub type CommentId = u32;

#[derive(Queryable, Serialize, Debug)]
pub struct Comment {
    id: CommentId,
    discussion: DiscussionId,
    author: UserId,
    markdown: String,
    created: NaiveDateTime,
}

pub struct Comments(pub Vec<Comment>);

impl Comments {
    pub fn get(connection: &mut Connection, discussion: DiscussionId) -> Result<Self, Error> {
        comments::table
            .filter(comments::discussion.eq(discussion))
            .load(connection)
            .map_err(Error::from)
            .map(|c| Comments { 0: c })
    }
}

#[derive(Serialize, Debug)]
pub struct DiscussionWithComments {
    discussion: Discussion,
    comments: Vec<Comment>,
}

impl DiscussionWithComments {
    pub fn get(connection: &mut Connection, discussion: DiscussionId) -> Result<Self, Error> {
        Ok(DiscussionWithComments {
            discussion: Discussion::get(connection, discussion)?,
            comments: Comments::get(connection, discussion)?.0,
        })
    }
}
