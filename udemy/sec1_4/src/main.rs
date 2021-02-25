//Generics (T) can be any type.
#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T), //Whatever T turns out to be
    Error(E),
}

/**A struct, returns an option when there is something, but there could be nothing
 * as well!
 */
pub enum Option<T> {
    Some(T),
    None,
}

/**Returns either a thing (type i32) or a string to represent error */
fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        Res::Error("Cannot divide by 0!".to_string())
    } else {
        Res::Thing(a / b)
    }
    /*If we dont not specify the else block here, we must explicitly
    specify the return statement. Without else statement, Rust considers that
    the else block will return (), so it expects the same in the "if" blocka as
    well. (we must then specify "return", see below) */
}

/**Using Result from the standard library */
fn divide_std(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Result::Err("Cannot divide by 0!".to_string());
    } else {
        return Result::Ok(a / b);
    }
}

fn main() {
    println!("Hello, world!");

    let a = divide(10, 5);
    let b = divide(10, 0);
    let c = divide_std(10, 5);
    println!("a = {:?}, b = {:?}", a, b);
    println!("a = {:?}", a);
    match a {
        Res::Thing(v) => println!("Match val = {}", v), //v is of type i32 which implements display
        _ => {} //_ represents every other case. We do nothing in this case
    }

    /*If we only care about 1 case, we can use "if let"
     * ====> if let (pattern) = a {..code..}
     * Meaning: if we can treat a as a pattern thing with a certain value (v for example),
     * then we can use v inside the code block
     */
    if let Res::Thing(v) = a {
        println!("if let val = {}", v);
    }
}
