extern crate rayon;
use rayon::prelude::*;

#[derive(Debug)]
enum Token {
    Plus,
    Fois,
    Entier(u32),
}

use Token::{Entier, Fois, Plus};

fn evaluation_simple(expression: &[Token]) -> u32 {
    unimplemented!()
}

fn main() {
    let expression = vec![
        Entier(1),
        Plus,
        Entier(3),
        Fois,
        Entier(5),
        Fois,
        Entier(2),
        Plus,
        Entier(1),
    ];
    println!("{:?} = {}", expression, evaluation_simple(&expression));
}
