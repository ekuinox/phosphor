use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::{Utc, NaiveDateTime};
use argon2::{self, Config};
use crate::schema::users;

fn convert_salt_from_naive_utc(date_time: &chrono::NaiveDateTime) -> String {
    date_time.format("%s").to_string()
}

fn encrypt_password(password: &String, date_time: &chrono::NaiveDateTime) -> String {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), convert_salt_from_naive_utc(&date_time).as_bytes(), &config).unwrap()
}

fn is_correct_password(password: &String, encrypted_password: &String) -> bool {
    argon2::verify_encoded(&encrypted_password, password.as_bytes()).unwrap()
}

#[test]
fn password_matching_test() {
    let current_time = Utc::now().naive_utc();
    let password = "password_matching_test".to_string();
    let encrypted_password = encrypt_password(&password, &current_time);
    assert!(is_correct_password(&password, &encrypted_password));
}

#[table_name = "users"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
#[primary_key(id)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    pub encrypted_password: String,
    pub created_at: Option<NaiveDateTime>
}

impl User {
    pub fn new(username: String, email: String, password: String) -> User {
        let current_time = Utc::now().naive_utc();
        User { id: None, username: username, email: email, encrypted_password: encrypt_password(&password, &current_time), created_at: Some(current_time) }
    }

    pub fn create(user: User, connection: &SqliteConnection) -> Option<User> {
        if diesel::insert_into(users::table).values(&user).execute(connection).is_ok() {
            return Some(users::table.order(users::id.desc()).first(connection).unwrap())
        }
        return None;
    }

    pub fn auth(username: String, password: String, connection: &SqliteConnection) -> Option<User> {
        match users::table.filter(users::username.eq(username)).first::<User>(connection) {
            Ok(user) => {
                match is_correct_password(&password, &user.encrypted_password) {
                    true => Some(user),
                    false => None
                }
            },
            Err(_) => None
        }
    }

    pub fn get(id: i32, connection: &SqliteConnection) -> Option<User> {
        match users::table.find(id).get_result(connection) {
            Ok(user) => Some(user),
            Err(_) => None
        }
    }

    pub fn read(connection: &SqliteConnection) -> Vec<User> {
        users::table.order(users::id).load::<User>(connection).unwrap()
    }

    pub fn update(id: i32, user: User, connection: &SqliteConnection) -> bool {
        diesel::update(users::table.find(id)).set(&user).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &SqliteConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(connection).is_ok()
    }
}

// 全ユーザに公開する
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUserData {
    pub id: i32,
    pub username: String
}

pub trait ToPublic {
    fn to_public(&self) -> PublicUserData;
}

impl ToPublic for User {
    fn to_public(&self) -> PublicUserData {
        PublicUserData {
            id: self.id.unwrap(),
            username: self.username.clone()
        }
    }
}

// ユーザ自身に公開する
#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateUserData {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime
}

pub trait ToPrivate {
    fn to_private(&self) -> PrivateUserData;
}

impl ToPrivate for User {
    fn to_private(&self) -> PrivateUserData {
        PrivateUserData {
            id: self.id.unwrap(),
            username: self.username.clone(),
            email: self.email.clone(),
            created_at: self.created_at.unwrap()
        }
    }
}
