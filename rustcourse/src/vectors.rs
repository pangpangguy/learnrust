/*Vectors = resizable arrays*/
pub fn run() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    //Get single value
    println!("Single value: {}", numbers[0]);

    let mut numbers2: Vec<i32> = vec![1, 2, 3, 4, 5];
    //Reassign value
    numbers2[2] = 200;
    println!("{:?}", numbers[2]);

    //Get array length
    println!("Length: {}", numbers.len());

    //Arrays are stack allocated
    //Each element occupies 4 byte
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //Add to vector (must be mutable!)
    numbers2.push(6);
    println!("Vector after push:{:?}", numbers2);

    //Remove last value with pop()
    numbers2.pop();
    println!("Vector after pop:{:?}", numbers2);

    //Loop through vector values
    for x in numbers2.iter() {
        println!("Number : {}", x);
    }

    //Loop and mutate values
    for x in numbers2.iter_mut() {
        *x *= 2;
    }
    println!("Numbers after mut : {:?}", numbers2);
}
