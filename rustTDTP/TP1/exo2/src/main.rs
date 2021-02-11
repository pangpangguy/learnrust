fn main() {

    //Question 1 
    // In a.collect(), a is an iterator.  
    //r and s are both vectors
    let r: Vec<_> = (1..21).collect();
    println!("r is {:?}",r);
    println!("r is {:?}",r);

    //Question 2
    let s: Vec<_> = (1..21).rev().collect();
    println!("s is {:?}",s);

    //Question 3 & 4
    //note : a.zip(b) is an iterator of values (ai, bi) 
    let mut t =  Vec::new();
    for (x,y) in r.iter().zip(s.iter()){
        t.push(x+y);
    }
    println!("t is {:?}", t);

    //Question 5: using map
    let t2: Vec<_> = r.iter().zip(s.iter()).collect();
    let t3: Vec<_> = r.iter().zip(s.iter()).map(|(x,y)| x+y).collect();
    println!("t2 is {:?}", t2);
    println!("t2 is {:?}", t3);

    //Question 6: test
    println!("All is 21 : {}", t3.iter().all(|x| *x == 21));

    //Question 7: sum of r
    let mut somme = 0;
    for x in &r{
        somme += *x;
    }
    println!("Sum of r: {}", somme);
    let somme2: i32 = r.iter().sum();
    println!("Sum of r: {:?}", somme2);

    //Question 8
    let u: Vec<_> = (1..11).map(|x| x*x).collect();
    println!("u is {:?}", u);
}
