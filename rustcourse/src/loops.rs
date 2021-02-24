//Loops

pub fn run() {
    let mut count = 0;

    //Infinite loop
    loop {
        count += 1;
        println!("Number = {}", count);

        if count == 20 {
            break;
        }
    }

    //While loop
    //Example Fizzbuzz
    let mut count2 = 0;
    while count2 <= 100 {
        if count2 % 15 == 0 {
            println!("FizzBuzz!");
        } else if count2 % 3 == 0 {
            println!("Fizz!");
        } else if count2 % 4 == 0 {
            println!("Buzz!");
        } else {
            println!("Ignored: {}", count2);
        }
        count2 += 1;
    }

    //For Range Loop
    for x in 0..100 {
        println!("number is {}", x);
    }
}
