DROP SCHEMA IF EXISTS linky CASCADE;
CREATE SCHEMA linky;

CREATE TABLE linky.link (
    shortlink varchar(150) not null,
    longlink varchar(150) not null,
    PRIMARY KEY (shortlink)
);