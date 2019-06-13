use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::{Utc, NaiveDateTime};
use yyid::yyid_string;
use crate::schema::access_tokens;
use crate::models::user::User;

#[table_name = "access_tokens"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[primary_key(token)]
pub struct AccessToken {
    pub token: Option<String>,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

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

    pub fn auth(token: &String, connection: &SqliteConnection) -> Option<AccessToken> {
        match access_tokens::table.find(&token).first(connection) {
            Ok(access_token) => Some(access_token),
            Err(_) => None
        }
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
