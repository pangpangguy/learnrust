/*Data types:
Primitive Types:
1.Integers: u8, i8, u16, i16, .... , u128, i128
2. Floats: f32, f64
3. Boolean: bool
4. Character: char (1 character!)
5. Tuples
6. Arrays
*/

pub fn run() {
    /*Rust is a statically typed language: it must know the type of all variables
    at compile time. However, the compiler can usually infer what type we want to
    use based on the value and how we use it.(Default values)
    Example: */
    let x = 1; //Default is i32
    let y = 2.5; //Default is f64

    //Adding explicit type
    let z: i64 = 1234566878;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //boolean
    let is_active = true; //Convention in Rust is underscore instead of camelCase.

    //get boolean from expression
    let is_greater = 10 > 5;
    println!("{:?}", (x, y, z, is_active, is_greater));

    //Char => use single quotes
    let a1 = 'a';
    let face = '\u{1F600}'; //smiley face emoji
    println!("{:?}", (a1, face));
}
