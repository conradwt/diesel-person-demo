// usage:  cargo run --bin write_person

extern crate diesel;
extern crate diesel_demo;

use self::diesel_demo::*;
use std::io::stdin;

fn main() {
  let connection = establish_connection();

  println!("Email: ");
  let mut email = String::new();
  stdin().read_line(&mut email).unwrap();
  let email = &email[..(email.len() - 1)]; // Drop the newline character

  println!("\n");

  println!("First Name: ");
  let mut first_name = String::new();
  stdin().read_line(&mut first_name).unwrap();
  let first_name = &first_name[..(first_name.len() - 1)]; // Drop the newline character

  println!("\n");

  println!("Last Name: ");
  let mut last_name = String::new();
  stdin().read_line(&mut last_name).unwrap();
  let last_name = &last_name[..(last_name.len() - 1)]; // Drop the newline character

  println!("\n");

  println!("Username: ");
  let mut username = String::new();
  stdin().read_line(&mut username).unwrap();
  let username = &username[..(username.len() - 1)]; // Drop the newline character

  let person = create_person(&connection, &email, &first_name, &last_name, &username);
  println!("\nSaved with id {}", person.id);
}

// #[cfg(not(windows))]
// const EOF: &'static str = "CTRL+D";

// #[cfg(windows)]
// const EOF: &'static str = "CTRL+Z";
