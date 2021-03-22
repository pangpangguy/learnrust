pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

/*An iterator must have:
1. a type for the item
2. a function next()*/
impl Iterator for Stepper {
    type Item = i32;
    //&mut self => pointer to a mutable version of self
    //Reminder: Option is either Some or None
    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let next = self.curr;
        self.curr += self.step;
        return Some(next);
    }
}

fn main() {
    /*Using loop*/
    let mut n = 0;
    loop {
        n += 1; //N ote: n++ doesnt work on Rust
        if n > 10 {
            break;
        }
        println!("Hello, {}!", n);
    }

    /*Using for loop*/
    for i in 0..10 {
        //0..10 range object of 0 to 10
        println!("For loop {}", i);
    }

    /*Using our constructed Stepper iterator*/
    let mut stp = Stepper {
        curr: 0,
        step: 1,
        max: 10,
    };

    loop {
        match stp.next() {
            Some(v) => println!("Stepper: {}", v),
            None => break, //End of loop
        }
    }

    /*Using for loop*/
    let stpper = Stepper {
        curr: 0,
        step: 3,
        max: 30,
    };
    for x in stpper {
        println!("{}", x);
    }

    /*Using 'while let' loop*/
    let mut while_stepper = Stepper {
        curr: 50,
        step: 10,
        max: 100,
    };

    while let Some(v) = while_stepper.next() {
        println!("{}", v);
    }
}
