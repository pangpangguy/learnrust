// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut furnitures = HashMap::new();

    furnitures.insert("Chairs", 5);
    furnitures.insert("Beds", 3);
    furnitures.insert("Tables", 2);
    furnitures.insert("Couches", 0);

    let mut total_stock = 0;
    for (key, value) in furnitures.iter(){
        let stock_count = if value == &0 { String::from("out of stock")}
        else{
            format!("{:?}", value)
        };
        total_stock += value;
        println!("key:{:?}, value:{:?}", key, stock_count);
    }

    println!("{:?}", total_stock);


}
