drop table if exists;

create table link (
    id serial primary key,
    shortlink varchar(150) not null,
    longlink varchar(150) not null, 
);

insert into link (shortlink, longlink) values ('ownsite', 'koziollek.com'), ('yt', 'youtube.com');