//Reference pointers - Point to a resource in memory

pub fn run() {
    //Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values : {:?}", (arr1, arr2));

    /*Non primitives:
    If we assign another variable to a piece of data, the first variable
    will no longer hold that value. We will need to use a reference (&) to
    point to that data*/
    let vec1 = vec![1, 2, 3];
    // let vec2 = vec1; //vec1 no longer owns the array
    let vec3 = &vec1; //OK
    println!("Values: {:?}", (&vec1, vec3));
}
