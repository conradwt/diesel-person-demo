// usage:  cargo run --bin show_people

extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::models::*;

fn main() {
  use diesel_demo::schema::people::dsl::*;

  let connection = establish_connection();
  let results = people
    .limit(5)
    .load::<Person>(&connection)
    .expect("Error loading people");

  println!("Displaying {} people", results.len());
  println!("\n");

  for person in results {
    println!(
      "{}: {}, {}, {}, {}",
      person.id, person.email, person.first_name, person.last_name, person.username
    );
  }
}
