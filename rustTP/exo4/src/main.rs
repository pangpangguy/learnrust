struct Puissances{
    suivante: u32
}

impl Puissances{
    //Object initialisation
    fn new() -> Self{
        Puissances{
            suivante: 1
        }
    }
}

//Define the iterator trait
impl Iterator for Puissances{
    type Item = u32;
    //next of iterator could be None(Null!), hence we use Option
    fn next(&mut self) -> Option<Self::Item>{
        let current = self.suivante;
        self.suivante = self.suivante*2;
        Some(current)
    }
}

fn main() {
    let mut x = Puissances::new();
    println!("X starts at : {}", x.suivante);
    println!("X.next : {:?}", x.next());
    println!("X.next : {:?}", x.next());
    //Use .unwrap() to get the value of Some(value)
    println!("X.next : {:?}", x.next().unwrap());

    //loop automatically calls .unwrap()
    for i in x.take(10){
        println!("X.next : {}", i)
    }
    
    //Question 2
    //Create a vector of 10 powers of 2
    let vec : Vec<f64> = Puissances::new()
                .take(10)   //Call next() 10 times
                .map(|p| f64::from(p).sqrt())  //Convert to f64 then .sqrt()
                .collect();
    println!("Vec is {:?}", vec);
}
