/* Arrays  = FIXED list where elements are the SAME data types*/
pub fn run() {
    //[type, size]
    //Data types in the array must be exact (correct type and correct number of elements!)
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    //Get single value
    println!("Single value: {}", numbers[0]);

    let mut numbers2: [i32; 5] = [1, 2, 3, 4, 5];
    //Reassign value
    numbers2[2] = 200;
    println!("{:?}", numbers[2]);

    //Get array length
    println!("Length: {}", numbers.len());

    //Arrays are stack allocated
    //Each element occupies 4 byte
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
