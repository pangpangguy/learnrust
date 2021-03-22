// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer{
    name: String,
    age: u32,
}
impl Customer{
    fn new(name: String, age: u32)->Self{
        Self{name, age}
    }
}

fn can_purchase(c:&Customer) -> Result<(), String>{
    if (c.age > 21){
        return Err("Under age.".to_owned());        
    }
    Ok(())  // () is the same as nothing.
}

fn main() {
    let customer = Customer::new("Bob".to_owned(), 25);
    let customer2 = Customer::new("Alice".to_owned(), 10);
    println!("{:?} : {:?}", customer.name, can_purchase(&customer));
    println!("{:?} : {:?}", customer2.name, can_purchase(&customer2));
}
