create table users (
	id serial primary key not null,
	name varchar not null
);

create table rooms (
	id serial primary key not null,
	name varchar not null
);

create table bookings (
	id serial primary key not null,
	room_id integer not null,
	user_id integer not null,

	foreign key (room_id) references rooms(id),
	foreign key (user_id) references users(id)
);
