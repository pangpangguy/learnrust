//Conditionals

pub fn run() {
    let age: u8 = 18;
    let check_id = false;
    let knows_person_of_age = true;

    //if else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: You're under age!");
    } else {
        println!("Bartender: I'll need to see your Id!");
    }

    //Shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of Age = {}", is_of_age);
}
