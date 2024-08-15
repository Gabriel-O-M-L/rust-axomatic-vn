create table if not exists "Author"
(
    author_id     uuid primary key default gen_random_uuid(),
    name          text not null,
    birthdate     date not null,
    nationality   text not null,
    biography     text not null,
    image         text not null
);