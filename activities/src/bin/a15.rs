// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets{
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let name1 = String::from("Alice");
    let name2 = String::from("Josh");
    let tickets = vec![Tickets::Standard(55.0), Tickets::Vip(90.0,name1), Tickets::Backstage(150.0,name2)];

    for t in tickets{
        match t {
            Tickets::Standard(price) => println!("Standard price is {:?}", price),
            Tickets::Vip(price, holder)=> println!("VIP price {:?} of {:?}", price, holder),
            Tickets::Backstage(price, holder)=> println!("Backstage price {:?} of {:?}", price, holder),
        }
    }
}
