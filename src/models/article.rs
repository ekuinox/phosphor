use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::{Utc, NaiveDateTime};
use crate::schema::articles;

#[table_name = "articles"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[primary_key(id)]
pub struct Article {
    pub id: Option<i32>,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub accessible: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}
