-- Add migration script here
create table if not exists sheets_auth(
    uid text primary key not null,
    refresh_token text not null,
    token text,
    expires_at datetime
);

create table if not exists sheets_id(
    uid text primary key not null,
    spreadsheet_id text not null
);
