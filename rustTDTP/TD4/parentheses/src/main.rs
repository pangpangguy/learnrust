extern crate rayon;
use rayon::prelude::*;
use std::collections::LinkedList;
fn calcul_pi(chaine: &str) -> Vec<i32> {
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
            l1.append(&mut l2);
            //    println!("{:?}", l1);
            l1
        })
        .into_iter()
        .fold(Vec::new(), |mut v1, v2| {
            let dernier = v1.last().cloned().unwrap_or(0);
            //           println!("v1 :{:?}", v1);
            //          println!("v2 : {:?}", v2);
            //pi_k = pi_{k-1} + valeur_courante(+1/-1)
            let pi_k = v2.par_iter().map(|x| *x + dernier);
            v1.par_extend(pi_k);
            v1
        })
}

//On a Sn = chaine de n parentheses, et pi_k = valeur pour la chaine (1..k+1)
//Donc Sn correctement parenthesée => pi_n-1 = 0 && pi_i < 0 for all i = (1..n-2) (toujours plus de '(' que ')' )
fn correctement_parenthesee(chaine: &str) -> bool {
    let pi = calcul_pi(chaine);
    pi.last().unwrap() == &0 && pi.par_iter().all(|x| *x >= 0)
}

/// Renvoie l'indice de la parenthese ouvrante correspondante.
/// Pre-suppose la chaine bien parenthesee.
fn mise_en_correspondance(chaine: &str, indice_fermante: usize, pi: &[i32]) -> usize {
    let pi_k = pi[indice_fermante] + 1;
    chaine
        .as_bytes()
        .par_iter()
        .zip(pi.par_iter())
        .take(indice_fermante)
        .enumerate() //enumerate gives (index, value). here value = (v1,v2)
        .find_last(|(_, (charac, pi))| **charac == '(' as u8 && **pi == pi_k)
        .map(|(index, _)| index)
        .unwrap()
}

/// Renvoie l'indice de chaque parenthese ouvrante et pour
/// chaque fermante l'indice de la parenthese ouvrante correspondante.
fn apparier(chaine: &str) -> Vec<usize> {
    let pi = calcul_pi(chaine);
    chaine
        .as_bytes()
        .par_iter()
        .enumerate()
        .map(|(index, charac)| {
            if *charac == b'(' {
                index
            } else {
                mise_en_correspondance(chaine, index, &pi)
            }
        })
        .fold(Vec::new, |mut vector, output| {
            vector.push(output);
            vector
        })
        .reduce(Vec::new, |mut v1, mut v2| {
            v1.append(&mut v2);
            v1
        })
}

/// renvoie le nombre de parentheses ouvrantes non fermees
/// et le nombre de parentheses fermantes non ouvertes.

fn reduire_sequentielle(caracteres: &[u8]) -> (u32, u32) {
    let mut unopened_close = 0;
    let mut current_opens = 0;
    for c in caracteres {
        if *c == b'(' {
            current_opens += 1;
        } else if current_opens == 0 {
            //closing bracket, and there are no open brackets => unopened close
            unopened_close += 1;
        } else {
            //Close an opened bracket
            current_opens -= 1;
        }
    }
    (current_opens, unopened_close)
}
fn reduire_parallel_dvp(caracteres: &[u8], limite: usize) -> (u32, u32) {
    if caracteres.len() >= limite {
        reduire_sequentielle(caracteres)
    } else {
        let mid = caracteres.len() / 2;
        let (c1, c2) = caracteres.split_at(mid);
        let ((o1, f1), (o2, f2)) = rayon::join(
            || reduire_parallel_dvp(c1, limite),
            || reduire_parallel_dvp(c2, limite),
        );
        if o1 > f2 {
            (o1 + o2 - f2, f1)
        } else {
            (o2, f1 + f2 - o1)
        }
    }
}

fn main() {
    let s = "(())((())())";
    println!("la chaine est {}", s);
    let pi = calcul_pi(s);
    println!("pi est {:?}", pi);
    println!("parenthesage ok ? {}", correctement_parenthesee(s));
    for (i, c) in s.chars().enumerate() {
        if c == ')' {
            println!(
                "l'ouvrante de {} est {}",
                i,
                mise_en_correspondance(s, i, &pi)
            );
        }
    }
    println!("apparier: {:?}", apparier(s));
    println!("reduire: {:?}", reduire_parallel_dvp(s.as_bytes(), 1));
    let s2 = ")(())(";
    println!(
        "reduire sur {}: {:?}",
        s2,
        reduire_parallel_dvp(s2.as_bytes(), 1)
    );
}
