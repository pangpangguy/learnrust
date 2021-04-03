use rayon::prelude::*;

fn additionne(a: &[u32], b: &[u32], retenue: u32) -> (Vec<u32>, u32) {
    assert_eq!(a.len(), b.len());
    if a.is_empty() {
        (Vec::new(), 0)
    } else if a.len() == 1 {
        let mut v = Vec::new();
        let r = a[0] + b[0] + retenue;
        v.push(r % 10);
        (v, r / 10) //if r < 10, r/10 = 0
    } else {
        //Divide recursively until length of 1
        let mid = a.len() / 2;
        let (a1, a2) = a.split_at(mid);
        let (b1, b2) = b.split_at(mid);
        //res = resultat, ret = retenue
        //g = gauche (partie gauche du vecteur), d = droite
        let (((res_g_sansret, ret_g_sansret), (res_g_avecret, ret_g_avecret)), (rest_d, ret_d)) = 
        rayon::join(|| rayon::join(|| additionne(a1, b1, 0), || additionne(a1,b1,0)), || additionne(a2,b2,0));

        //Pas de retenue de la moitié droite de calcul, alors on prend celui sans ret
        if ret_d == 0{
            let resultat:Vec<u32> = res_g_sansret.par_iter().chain(rest_d.par_iter()).cloned().collect();
            (resultat, ret_g_sansret)
        }
        //Sinon, prendre celui avec ret (pour faire +1 retenue)
        else{
            let resultat:Vec<u32> = res_g_avecret.par_iter().chain(rest_d.par_iter()).cloned().collect();
            (resultat, ret_g_avecret)
        }
    }
}

// ajoute 1 au nombre stocke dans s, renvoie vrai si tous les chiffres etaient 9
//cad il y a retenue apres incrementation
fn increment(s: &mut [u32]) -> bool {
    if let Some((limite, _)) = s.par_iter().enumerate().find_last(|&(indice, chiffre)| {
        *chiffre != 9 && (indice == (s.len() - 1) || s[indice + 1] == 9)
    }) {
        s[limite] += 1;
        s[(limite + 1)..].par_iter_mut().for_each(|v| *v = 0);
        false
    } else {
        s.par_iter_mut().for_each(|v| *v = 0);
        true
    }
}

// additionne a et b dans la tranche donnee
// Renvoyer True s'il y a une retenue
// r = resultat
fn addition2(a: &[u32], b: &[u32], r: &mut [u32]) -> bool {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), r.len());
    if a.is_empty() {
        false
    } else if a.len() == 1 {
        let s = a[0] + b[0];
        r[0] = s % 10;
        s >= 10 //true si vrai (retenue)
    } else {
        let milieu = a.len() / 2;
        let (a1, a2) = a.split_at(milieu);
        let (b1, b2) = b.split_at(milieu);
        let (r1, r2) = r.split_at_mut(milieu);
        let (retenue1, retenue2) = rayon::join(|| addition2(a1, b1, r1), || addition2(a2, b2, r2));
        if retenue2 {
            increment(r1) || retenue1   //S'il y a retenue de la partie droite, ajoute 1 à la partie gauche, la fonction renvoie True si : il y a retenue apres incrementation OU le calcul de la partie gauche.
        } else {
            retenue1
        }
    }
}
fn main() {
    let a = [1, 2, 3, 4];
    let b = [5, 6, 7, 8];
    println!("a: {:?}, b: {:?}", a, b);
    let (resultat, _) = additionne(&a, &b, 0);
    println!("somme : {:?}", resultat);

    let x = [0, 1, 5, 9, 9, 9, 9, 9];
    let mut y = x;
    increment(&mut y);
    println!("test increment {:?} -> {:?}", x, y);

    let x = [0, 1, 5, 3, 2, 3, 2, 7];
    let mut y = x;
    increment(&mut y);
    println!("test increment {:?} -> {:?}", x, y);

    let a = [1, 2, 3, 4];
    let b = [5, 6, 7, 8];
    let mut resultat = [0; 4];
    println!("a: {:?}, b: {:?}", a, b);
    addition2(&a, &b, &mut resultat);
    println!("somme : {:?}", resultat);

}
