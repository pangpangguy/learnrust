/* Strings in Rust: 2 Types of strings:
1. Primitive str = Immutable fixed-length string somewhere in memory. Called string slice. (can be on stack/heap/.)
2. String = Growable, heap-allocated data structure (dynamic)*/
pub fn run() {
    let hello = "Hello"; //primitive aka &str
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

    /*String slice does not "own" a memory slot, it only borrows/points
    to it. It can be on either stack or heap.*/
    let str1: &str = "Hey";
    /*String are always on the heap! So must be careful when using them (&!)*/
    let str2: String = String::from("There!");

    /*Conversion between &str and String*/
    //&str -> String
    let string_from_str = str1.to_string();
    let string_from_str2 = "Some hardcoded string".to_string();

    //String -> &str
    let str_from_string: &str = &str2; //Points to str2
    println!("str_from_string: {}", str_from_string);

    //Adding 2 strings together
    let combine_string_literals = ["Hello ", "There!"].concat();
    let combine_with_format_macro = format!("{} {}", "With", "Macro");

    //Adding 2 Strings
    let a = String::from("String 1");
    let b = String::from("String 2");
    let combined = a + &b; //We need to add & to each subsequence string, except the first one.

    println!("Combine with concat(): {}", combine_string_literals);
    println!("Combine with format!: {}", combine_with_format_macro);
    //    println!("Combine with format!: {}", combined);

    //substring
    let c = String::from("Helloooo");
    let d: &str = "Testzzz";
    let substring1: &str = &c[0..2];
    let substring2: &str = &d[0..2];

    println!("substring1 : {}, substring2 : {}", c, substring2);
}
