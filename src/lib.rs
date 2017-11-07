#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

use diesel::{JoinDsl,LoadDsl};

infer_schema!("dotenv:DATABASE_URL");

/// A room that can be booked
#[derive(Debug, Identifiable, Queryable)]
pub struct Room {
    pub id: i32,
    pub name: String
}

/// A person who can book a room
#[derive(Debug, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String
}

#[derive(Debug, Identifiable, Queryable)]
pub struct Booking {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
}

type DieselResult<T> = Result<T, diesel::result::Error>;

impl Booking {
    pub fn with_rooms(c: &diesel::PgConnection) -> DieselResult<Vec<(Booking, Room)>> {
        use self::bookings::dsl::*;
        bookings.inner_join(rooms::table).load(c)
    }

    pub fn with_users(c: &diesel::PgConnection) -> DieselResult<Vec<(Booking, User)>> {
        use self::bookings::dsl::*;
        bookings.inner_join(users::table).load(c)
    }

    pub fn full(c: &diesel::PgConnection) -> DieselResult<Vec<(Booking, Room, User)>> {
        use self::bookings::dsl::*;

        bookings.inner_join(rooms::table)
                .inner_join(users::table)
                .load(c)
    }
}
