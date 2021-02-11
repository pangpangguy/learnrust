fn main() {
    //Question 1
    let vec = vec![1,2,3,4,5];
    println!("{:?}", vec);

    /*Question 2
     adding & => borrow: it tells the iterator that [1,2,3,4,5] is READ-ONLY (cannot 
     be modified), but vec still has has the ownership of the vector*/ 
    let mut somme = 0;
    for i in &vec {
        somme += *i;
    }
    println!("Second method somme : {}", somme);
    println!("vec still has ownership of vector: {:?}", vec);

    /*
    If we dont put a &, vec will lose ownership!
    let mut somme2 = 0;
    for i in vec{
        somme2 += i;
    }
    
    println!("Here vec no longer has the ownership, we would have an error!");
    println!("vec is : {:?}", vec);
    */

    //Question 3
    let  vec_zero1 = vec![0,0,0,0,0];
    let  mut vec_zero2 = vec![0;5];
    println!("vecZero1 : {:?}", vec_zero1); 
    println!("vecZero2 : {:?}", vec_zero2); 

    //Question 4
    for i in 1..6{
        vec_zero2[i-1] = i as i32;
    }
    assert_eq!(vec_zero2, vec);
    println!("vec_zero2 after change : {:?}", vec_zero2);

    //Question 5
    remise_a_zero(vec_zero2.as_mut_slice());
    println!("vec_zero2 after remise_a_zero : {:?}", vec_zero2);

}

//The function takes in a mutable slice/array. (vec_zero2 still has ownership!)
fn remise_a_zero(tranche: &mut [i32]){
    for x in tranche{
        *x = 0;
    }
}

