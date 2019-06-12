-- Your SQL goes here
create table access_tokens (
    token text primary key,
    user_id integer not null,
    created_at datetime default current_date,
    updated_at datetime default current_date,
    foreign key(user_id) references users(id)
);
