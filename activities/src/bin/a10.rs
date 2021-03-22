// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn message(b: bool) {
    match b {
        true => println!("its big"),
        false => println!("its false"),
    }
}

fn main() {
    let x = 500;
    let is_big = x > 100;
    message(is_big);
}
