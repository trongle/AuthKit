-- Add migration script here
create table users (
	id int unsigned auto_increment primary key,
	username varchar(12) not null unique,
	email varchar(255) not null unique,
	password text not null,
	created_at timestamp default current_timestamp,
	updated_at timestamp on update current_timestamp
);
