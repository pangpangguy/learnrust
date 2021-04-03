use rayon::prelude::*;

/**Exercice 1 Echauffement */
const N: usize = 5;
//Question 1 : Calculer la somme d'un vecteur
fn somme_vecteur(v: &[u32]) -> u32 {
    v.par_iter().sum()
}

//Question 2: Creer vecteur contenant f(i), i de 0->n
fn creer_vecteur(n: u32) -> Vec<u32> {
    (0..n).into_par_iter().map(|x| x + 1).collect()
}

//Question 3: verifier somme de chaque ligne est le meme
fn verifier_somme(matrice: &[[i32; N]; N]) -> bool {
    //Calculer somme de 1ere ligne
    let somme: i32 = matrice[0].par_iter().sum();
    for i in 1..N {
        let somme_ligne: i32 = matrice[i].par_iter().sum();
        if somme_ligne != somme {
            return false;
        }
    }
    true
}

fn verifier_somme2(matrice: &[[i32; N]; N]) -> bool {
    let somme: i32 = matrice[0].par_iter().sum();
    matrice[1..]
        .par_iter()
        .all(|x| x.par_iter().sum::<i32>() == somme)
}

//Exercice 2
//Question 2
fn produit_parallel_matrice_vecteur(matrice: &[[u32; N]; N], vecteur: &[u32; N]) -> [u32; N] {
    let mut resultat = [0; N];

    //un iterator sur les produits calculées
    let produit = matrice.par_iter().map(|x| {
        x.par_iter()
            .zip(vecteur.par_iter())
            .map(|(a, b)| a * b)
            .sum()
    });

    resultat
        .par_iter_mut()
        .zip(produit)
        .for_each(|(a, b)| *a = b);

    resultat
}

fn main() {
    println!("{:?}", somme_vecteur(&[3, 3, 3, 3, 3, 3]));
    println!("{:?}", creer_vecteur(5));
    let matrix = [
        [0, 1, 2, 3, 1],
        [2, 1, 3, 0, 1],
        [0, 4, 1, 1, 1],
        [6, 0, 0, 0, 0],
        [0, 0, 0, 6, 1],
    ];
    println!("{:?}", verifier_somme(&matrix));
    println!("{:?}", verifier_somme2(&matrix));

    let matrix2: [[u32; N]; N] = [
        [0, 1, 2, 3, 1],
        [2, 1, 3, 0, 1],
        [0, 4, 1, 1, 1],
        [6, 0, 0, 0, 0],
        [0, 0, 0, 6, 1],
    ];
    println!(
        "{:?}",
        produit_parallel_matrice_vecteur(&matrix2, &[1, 1, 1, 1, 1])
    );
}
