-- Your SQL goes here
create table burger (
    id serial primary key,
    name varchar(255) unique not null,
    image_url varchar(255),
    description text
);