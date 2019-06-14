use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::{Utc, NaiveDateTime};
use crate::schema::articles;
use crate::define_enum;

define_enum! {
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum Accessible {
        Draft = 0,
        Private = 1,
        Protected = 2,
        Public = 3,
    }
}

#[table_name = "articles"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[primary_key(id)]
pub struct Article {
    pub id: Option<i32>,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub accessible: Accessible,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

