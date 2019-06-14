-- Your SQL goes here
create table articles (
    id integer primary key autoincrement,
    user_id integer not null,
    title text not null,
    body text not null,
    permalink text unique not null,
    accessible integer not null,
    created_at datetime default current_date,
    updated_at datetime default current_date,
    foreign key(user_id) references users(id)
);
