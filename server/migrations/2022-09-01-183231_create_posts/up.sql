-- Your SQL goes here
CREATE TABLE links (
    id serial primary key,
    shortlink varchar(150) not null,
    longlink varchar(150) not null
)