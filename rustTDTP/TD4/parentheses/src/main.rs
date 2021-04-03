extern crate rayon;
use rayon::prelude::*;
use std::collections::LinkedList;
fn calcul_pi(chaine: &str) -> LinkedList<Vec<i32>> {
    //Calculate opening bracket2
    chaine
        .par_chars()
        .map(|x| if x == '(' { 1 } else { -1 })
        .fold(Vec::new, |mut v, x| {
            let new_total = v.last().cloned().unwrap_or(0);
            v.push(new_total + x);
            v
        })
        .map(|v| {
            let mut l = LinkedList::new();
            l.push_back(v);
            l
        })
        .reduce(LinkedList::new, |mut l1, mut l2| {
            l1.pushback(&mut l2);
            l1
        })
}

fn correctement_parenthesee(chaine: &str) -> bool {
    unimplemented!()
}

/// Renvoie l'indice de la parenthese ouvrante correspondante.
/// Pre-suppose la chaine bien parenthesee.
fn mise_en_correspondance(chaine: &str, indice_fermante: usize, pi: &[i32]) -> usize {
    unimplemented!()
}

/// Renvoie l'indice de chaque parenthese ouvrante et pour
/// chaque fermante l'indice de la parenthese ouvrante correspondante.
fn apparier(chaine: &str) -> Vec<usize> {
    unimplemented!()
}

/// renvoie le nombre de parentheses ouvrantes non fermees
/// et le nombre de parentheses fermantes non ouvertes.
fn reduire(caracteres: &[u8], limite: usize) -> (u32, u32) {
    unimplemented!()
}

fn main() {
    let s = "(())((())())";
    println!("la chaine est {}", s);
    let pi = calcul_pi(s);
    println!("pi est {:?}", pi);
    println!("parenthesage ok ? {}", correctement_parenthesee(s));
    /*  for (i, c) in s.chars().enumerate() {
        if c == ')' {
            println!(
                "l'ouvrante de {} est {}",
                //            i,
                //              mise_en_correspondance(s, i, &pi)
            );
        }
    }*/
    println!("apparier: {:?}", apparier(s));
    println!("reduire: {:?}", reduire(s.as_bytes(), 1));
    let s2 = ")(())(";
    println!("reduire sur {}: {:?}", s2, reduire(s2.as_bytes(), 1));
}
