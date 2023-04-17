-- Add migration script here
create table if not exists log(
    id integer primary key autoincrement,
    uid text not null,
    timestamp datetime not null,
    type text not null,
    count integer default 0 not null,
    name text,
    time integer,
    comment text
);