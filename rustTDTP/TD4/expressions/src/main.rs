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
    expression
        .par_split(|x| match *x {
            Token::Plus => true,
            _ => false,
        })
        .map(|x| {
            x.iter()
                .map(|x| match *x {
                    Token::Entier(i) => i,
                    _ => 1,
                })
                .fold(1, |acc, x| acc * x)
        })
        .sum()
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
