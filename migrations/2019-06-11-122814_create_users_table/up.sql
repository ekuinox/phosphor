-- Your SQL goes here
create table users (
  id integer primary key autoincrement,
  username text unique not null,
  email text unique not null,
  encrypted_password text not null,
  created_at datetime default current_date
);