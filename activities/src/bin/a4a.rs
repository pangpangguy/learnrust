// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let n = "LOL";
    match n {
        "qsdqds" => println!("it's true"),
        "Alice" => println!("it's false"),
        _ => println!("not a string"),
    }
}
