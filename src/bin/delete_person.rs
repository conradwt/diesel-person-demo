// usage: cargo run --bin delete_person person-id

extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use std::env::args;

fn main() {
  use diesel_demo::schema::people::dsl::*;

  let target = args().nth(1).expect("Expected a target to match against");
  let primary_key: i32 = target.parse().unwrap();

  let connection = establish_connection();
  let num_deleted = diesel::delete(people.filter(id.eq(primary_key)))
    .execute(&connection)
    .expect("Error deleting person");

  println!("Deleted {} person", num_deleted);
}
