/* Strings in Rust: 2 Types of strings:
1. Primitive str = Immutable fixed-length string somewhere in memory
2. String = Growable, heap-allocated data structure (dynamic)*/
pub fn run() {
    let hello = "Hello"; //primitive
    let hello2 = String::from("Hello"); //dynamic

    println!("Primitive: {} , Dynamic : {}", hello, hello2);

    //Get length
    println!("Length: {} and {}", hello.len(), hello2.len());

    let mut hello3 = String::from("Hello, ");
    //Add a character
    hello3.push('W');

    //Add a string
    hello3.push_str("orld!");
    println!("{}", hello3);

    //Capacity in bytes (nb of bytes it can store)
    println!("Capacity: {}", hello3.capacity());

    //Check if empty
    println!("String is empty: {}", hello3.is_empty());

    //Contains something?
    println!("String contains 'World' ? :{}", hello3.contains("World"));

    //Replace
    println!("Replace: {}", hello3.replace("World", "There"));

    //Loop through String by whitespace
    for word in hello3.split_whitespace() {
        println!("Word = {}", word);
    }

    //Create a string with a certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion
    assert_eq!(2, s.len());
    //assert_eq!(3, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s);
}
