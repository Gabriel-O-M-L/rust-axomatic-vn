create table if not exists "publisher"
(
    publisher_id    uuid primary key default gen_random_uuid(),
    name            text not null,
    logo            text not null,
    country         text not null,
    founded_date    date not null,
    website         text,

);