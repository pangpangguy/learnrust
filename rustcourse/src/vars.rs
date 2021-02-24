/* Variables:
1. Hold primitive data or references to data
2. Immutable by default (Can't reassign them)
3. Rust is a block-scoped language*/

pub fn run() {
    let name = "Ryan";
    let age = 23;

    //This will have an error
    //age = 38;
    println!("Default : My name is {} and I am {}", name, age);

    let mut age2 = 38;
    println!("mut : My name is {} and I am {}", name, age2);
    age2 = 99; //OK
    println!("mut : My name is {} and I am {}", name, age2);

    //Define constant. must explicitly define a type. Usually Uppercase
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multple vars
    let (my_name, my_age) = ("Ryan", 23);
    println!("I am {} and I am {}", my_name, my_age);
}
