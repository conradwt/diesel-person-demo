#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::{NewPerson, Person};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_person<'a>(
  conn: &PgConnection,
  email: &'a str,
  first_name: &'a str,
  last_name: &'a str,
  username: &'a str,
) -> Person {
  use schema::people;

  let new_person = NewPerson {
    email: email,
    first_name: first_name,
    last_name: last_name,
    username: username,
  };

  diesel::insert_into(people::table)
    .values(&new_person)
    .get_result(conn)
    .expect("Error saving new person")
}
