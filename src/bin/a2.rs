// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add(num1: i32, num2: i32) -> i32 {
  num1 + num2
}

fn display_result(result: i32) {
  println!("The result of the operation is {:?}", result);
}

fn main() {
  let result = add(5, 10);
  display_result(result);
}
