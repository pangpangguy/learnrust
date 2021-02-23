/*Provide the function to display a structure in the form:
        struct_name = { attributes }
  Use {:?} in println!
*/

/* enumerated type. either ONE of them*/

#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue,
}

#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    fave_color: Color,
}

impl Person {
    pub fn print(self) -> String {
        format!(
            "name = {}, age={} has {} children",
            self.name, self.age, self.children
        ) //Do not put a ; here!
    }
}
/* The last statement inside a {...} will be it's return!
Alternatively we can include a return keyword if we want to add
a ";" at the end. Ex:
... return format!(...); */

fn main() {
    let p = Person {
        name: "Matthew".to_string(),
        age: 35,
        children: 4,
        fave_color: Color::Green,
    };

    let c = Color::Red("hello".to_string());
    match c {
        Color::Red(s) => println!("It's {} from red!", s), //If we do Red(_) => we don't care what the string is
        Color::Blue => println!("It's blue!"),
        Color::Green => println!("It's green!"),
    }

    println!("Hey : {:?}", p);
}
