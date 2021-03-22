// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavor {
    Soda,
    Raspberry,
    Boboa,
}
struct Drink {
    flavor: Flavor,
    ounce: f64,
}

fn print_drink(d: Drink) {
    match d.flavor {
        Flavor::Soda => println!("soda"),
        Flavor::Raspberry => println!("raspberry"),
        Flavor::Boboa => println!("boboa"),
    }
}
fn main() {
    let drink = Drink {
        flavor: Flavor::Soda,
        ounce: 6.0,
    };
    print_drink(drink);
}
