#[derive(Queryable)]
pub struct Person {
  pub id: i32,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub username: String,
}

use super::schema::people;

#[derive(Insertable)]
#[table_name = "people"]
pub struct NewPerson<'a> {
  pub email: &'a str,
  pub first_name: &'a str,
  pub last_name: &'a str,
  pub username: &'a str,
}
