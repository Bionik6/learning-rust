// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal
fn display_first_name(name: &str) {
    println!("Hello {:?}", name);
  }

  fn display_last_name(name: &str) {
    println!("Hello {:?}", name);
  }

  fn display_full_name(first_name: &str, last_name: &str) {
    println!("Hello {:?} {:?}", first_name, last_name);
  }

fn main() {
  display_first_name("Ibrahima");
  display_last_name("Ciss");
  display_full_name("Ibrahima", "Ciss");
}
