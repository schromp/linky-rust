<<<<<<< HEAD
--psql -U actix -p 5430 -f sql/database.sql actix

DROP SCHEMA IF EXISTS linky CASCADE;
CREATE SCHEMA linky;

=======
>>>>>>> c2520bfeb6ad4cb539ae2c48245a3d74d0b8f071
CREATE TABLE linky.link (
    shortlink varchar(150) not null,
    longlink varchar(150) not null,
    PRIMARY KEY (shortlink)
);