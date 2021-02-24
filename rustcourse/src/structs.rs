//Normal struct
struct Color {
    red: u8, //0 -> 255
    green: u8,
    blue: u8,
}

//tuple struct
struct Color2(u8, u8, u8);

//struct with function
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    //Function that gets full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
        //write the text to String. print! == format! + output to stdout
    }

    //Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
        //RQ: &str is a static string, we need to conver it to heap memory
        //using ...to_string()
    }

    //Show name as tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = Color2(255, 0, 0);
    println!("Color tuple struct: {} {} {}", c2.0, c2.1, c2.2);
    //Change a property
    c.red = 200;
    c2.0 = 200;

    let mut p = Person::new("Ryan", "Tee"); //Calling the new function of struct Person
    println!("Person : {} {}", p.first_name, p.last_name);
    println!("{}", p.full_name()); //method/fn => must have ()

    //Set last name
    p.set_last_name("Dough");
    println!("{}", p.full_name());

    //display tuple
    println!("{:?}", p.to_tuple());
}
