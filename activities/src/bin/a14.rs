// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

#[derive(Debug)]
enum Color{
    Red,
    Blue,
    Green,
}

impl Color{
    fn print(&self){
        match self{
            Color::Red => print!("Red"),
            Color::Blue => print!("Blue"),
            Color::Green => print!("Green"),
        }
    }
}
#[derive(Debug)]
struct Person{
    age: u32,
    name: String,
    fav_color: Color,
}

impl Person{
    fn new(age: u32, name: String, fav_color: Color)->Self{
        Self{
            age, name, fav_color,
        }
    }

    fn print(&self){
        println!("Name:{}, Age:{}", self.name, self.age);
        print!("Favourite Colour: ");
        self.fav_color.print();
    }
}
fn main() {
    let people = vec![
        Person::new(5, String::from("Alice"), Color::Red),
        Person::new(20, String::from("Giant"), Color::Blue),
        Person::new(15, String::from("Damien"), Color::Green),
    ];

    for p in &people{
        if p.age <= 10 {
            p.print();
        }
    }    
}
