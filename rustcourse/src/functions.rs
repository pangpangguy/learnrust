pub fn run() {
    greeting("Hello", "James");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum is : {}", get_sum);

    //Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Closure sum is : {}", add_nums(5, 5));

    //With closure we can use outside variables
    let n3 = 50;
    let add_nums2 = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum with outside variable: {}", add_nums2(5, 5));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

//Function that returns
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 //no ; !!
}
