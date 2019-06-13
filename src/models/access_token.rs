use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::NaiveDateTime;
use yyid::yyid_string;
use crate::schema::access_tokens;

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
        AccessToken { token: Some(yyid_string()), user_id: user_id, created_at: None, updated_at: None }
    }

    pub fn create(&self, connection: &SqliteConnection) -> Option<AccessToken> {
        let result = diesel::insert_into(access_tokens::table).values(self).execute(connection);
        if result.is_err() {
            return None;
        }
        return Some(self.clone())
    }

    pub fn auth(token: String, connection: &SqliteConnection) -> Option<i32> {
        let result = access_tokens::table.find(token).first(connection);
        if result.is_ok() {
            let access_token: AccessToken = result.unwrap();
            return Some(access_token.user_id)
        }
        return None;
    }
}