use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::{Utc, NaiveDateTime};
use yyid::yyid_string;
use crate::schema::access_tokens;
use crate::models::user::{User, Authenticate};

#[table_name = "access_tokens"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[primary_key(token)]
pub struct AccessToken {
    pub token: Option<String>,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

// 24時間更新がない場合死ぬ
const VALID_HOURS: i64 = 24;

impl AccessToken {
    pub fn new(user_id: i32) -> AccessToken {
        let now = Utc::now().naive_utc();
        AccessToken { token: Some(yyid_string()), user_id: user_id, created_at: Some(now), updated_at: Some(now) }
    }

    pub fn create(&self, connection: &SqliteConnection) -> Option<AccessToken> {
        match diesel::insert_into(access_tokens::table).values(self).execute(connection) {
            Ok(_) => Some(self.clone()),
            Err(_) => None
        }
    }

    pub fn from_string(token: &String, connection: &SqliteConnection) -> Option<AccessToken> {
        match access_tokens::table
            .filter(access_tokens::token.eq(&token)) // トークンが一致している
            .filter(access_tokens::updated_at.gt(Utc::now().naive_utc() - chrono::Duration::hours(VALID_HOURS))) // 有効時間内（指定した時間より新しい）
            .first::<AccessToken>(connection) {
            Ok(access_token) => Some(access_token),
            Err(_) => None
        }
    }

    // updated_atを更新する
    pub fn touch(&self, connection: &SqliteConnection) -> AccessToken {
        let access_token = AccessToken { updated_at: Some(Utc::now().naive_utc()), .. self.clone() };
        diesel::update(access_tokens::table.find(&access_token.token)).set(&access_token).execute(connection).unwrap();
        return access_token;
    }
}

pub trait ToUser {
    fn to_user(&self, connection: &SqliteConnection) -> Option<User>;
}

impl ToUser for AccessToken {
    fn to_user(&self, connection: &SqliteConnection) -> Option<User> {
        User::get(self.user_id, &connection)
    }
}
