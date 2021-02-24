pub fn run() {
    //Print to console
    println!("Hello from the print.rs file!!");

    /*In rust we must use a placeholder to print anything
    (string, number,..)*/
    println!("Number:{}", 1);
    println!("{} is from {}", "Ryan", "Tee");

    //Positional arguments
    println!(
        "{0} is originall from {1} and {0} likes to play basketball",
        "Ryan", "Malaysia"
    );

    //Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Ryan",
        activity = "basketball"
    );

    //Placeholde traits
    println!("Binary: {:b} | Hex : {:x} | Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello", true)); //=> a tuple but we can put in multiple values

    //Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
