This is a demonstration repository for a [Diesel](sgrif/diesel) issue.

## Setup

### Versions

- **Rust:** `1.23.0-nightly (8b22e70b2 2017-10-31)`
- **Diesel:** `0.16.0`
- **Database:** `postgres`
- **Operating System:** any

### Feature Flags

- **diesel:**
- **diesel_codegen:** `[ "postgres" ]`

## Problem Description


### What are you trying to accomplish?

I have a [reduced test case](https://github.com/trombonehero/diesel-join-example) that involves three types of DB-backed objects: `Room`, `User` and `Booking`. A `Booking` object represents a `User` having reserved a `Room` (e.g., for a meeting):

```sql
create table bookings (
	id serial primary key not null,
	room_id integer not null,
	user_id integer not null,

	foreign key (room_id) references rooms(id),
	foreign key (user_id) references users(id)
);
```

When I run `diesel print-schema` I see:

```
[...]
joinable!(bookings -> rooms (room_id));
joinable!(bookings -> users (user_id));
```

I'm also able to use a single `inner_join` between `Booking` and either `Room` or `User`:

```rust
/* ... */

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
}
```

Next, I want to be able to retrieve a `Vec<(Booking, Room, User)>` from the database by joining both `bookings->rooms` and `bookings->users` in the same query.


### What is the expected output?

When I try to compile:

```rust
impl Booking {
    /* ... */
    pub fn full(c: &diesel::PgConnection) -> DieselResult<Vec<(Booking, Room, User)>> {
        use self::bookings::dsl::*;

        bookings.inner_join(rooms::table)
                .inner_join(users::table)
                .load(c)
    }
}
```

I would expect Diesel to generate a query like:

```sql
SELECT * from bookings
    INNER JOIN rooms ON rooms.id = room_id
    INNER JOIN users ON users.id = user_id
```

(i.e., joining `bookings->rooms` and `bookings->users`)

### What is the actual output?

```
error[E0277]: the trait bound `__diesel_infer_schema::infer_rooms::rooms::table:
 diesel::JoinTo<__diesel_infer_schema::infer_users::users::table>` is not satisf
ied
  --> src/lib.rs:47:18
   |
47 |                 .inner_join(users::table)
   |                  ^^^^^^^^^^ the trait `diesel::JoinTo<__diesel_infer_schema
::infer_users::users::table>` is not implemented for `__diesel_infer_schema::inf
er_rooms::rooms::table`
```

So, Diesel seems to be trying to join `bookings->rooms` and `rooms->users`. I would've expected this if I were trying to compile:

```rust
bookings.inner_join(
    rooms::table.inner_join(
        users::table
    )
)
```

but in my use case, I'm trying to join both `rooms` and `users` with `bookings` individually.


### Steps to reproduce

1. Create an empty database, set `DATABASE_URL`
1. Clone https://github.com/trombonehero/diesel-join-example
1. Execute `cargo build`


## Checklist

- [X] I have already looked over the [issue tracker](https://github.com/diesel-rs/diesel/issues) for similar issues.
