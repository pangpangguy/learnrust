fn main() {
    let mut n = 0;

    loop {
        n += 1; //N ote: n++ doesnt work on Rust
        if n > 10 {
            break;
        }
        println!("Hello, {}!", n);
    }

    for i in 0..10 {
        println!("For loop {}", i);
    }

    println!("All done!");
}
