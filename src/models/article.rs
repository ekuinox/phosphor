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
#[derive(Debug, AsChangeset, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[primary_key(id)]
pub struct Article {
    pub id: Option<i32>,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub permalink: String,
    pub accessible: Accessible,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermalinksListData {
    pub permalink: String,
    pub title: String,
    pub updated_at: NaiveDateTime
}

impl Article {
    pub fn new(
        id: Option<i32>,
        user_id: i32,
        title: String,
        body: String,
        permalink: String,
        accessible: Accessible,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>
        ) -> Article {
        Article {
            id: id,
            user_id: user_id,
            title: title,
            body: body,
            permalink: permalink,
            accessible: accessible,
            created_at: created_at,
            updated_at: updated_at
        }
    }

    // 現在時刻でArticleを生成する
    pub fn new_with_now(user_id: i32, title: String, body: String, permalink: String, accessible: Accessible) -> Article {
        let now = Utc::now().naive_utc();
        Self::new(None, user_id, title, body, permalink, accessible, Some(now), Some(now))
    }

    // selfをテーブルに挿入する
    pub fn insert(&self, connection: &SqliteConnection) -> Option<Article> {
        match diesel::insert_into(articles::table).values(self).execute(connection) {
            Ok(_) => Some(self.clone()),
            Err(_) => None
        }
    }

    // 記事permalinkより記事を取得する
    pub fn get_by_permalink(permalink: String, connection: &SqliteConnection) -> Option<Article> {
        match articles::table.filter(articles::permalink.eq(permalink)).first::<Article>(connection) {
            Ok(article) => Some(article),
            Err(_) => None
        }
    }

    // user_idを元に記事を取得する
    pub fn get_by_user_id(user_id: i32, connection: &SqliteConnection) -> Option<Vec<Article>> {
        match articles::table.filter(articles::user_id.eq(user_id)).get_results::<Article>(connection) {
            Ok(users_articles) => Some(users_articles),
            Err(_) => None
        }
    }

    // user_idを元に記事のpermalinkリストを取得する
    pub fn get_list_by_user_id(user_id: i32, count: i64, connection: &SqliteConnection) -> Option<Vec<PermalinksListData>> {
        match articles::table.filter(articles::user_id.eq(user_id)).order(articles::updated_at).limit(count).get_results::<Article>(connection) {
            Ok(users_articles) => {
                Some(users_articles.iter().map(|article| article.to_permalink_list_data()).collect::<Vec<_>>())
            },
            Err(_) => None
        }

    }
}

pub trait ToPermalinkListData {
    fn to_permalink_list_data(&self) -> PermalinksListData;
}

impl ToPermalinkListData for Article {
    fn to_permalink_list_data(&self) -> PermalinksListData {
        PermalinksListData {
            permalink: self.permalink.clone(),
            title: self.title.clone(),
            updated_at: self.updated_at.unwrap()
        }
    }
}
