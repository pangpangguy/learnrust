use itertools::kmerge;
use std::mem::swap;

pub fn remplir_blocs(tranche: &mut [u32], taille: usize) {
    for (valeur, bloc) in tranche.chunks_mut(taille).enumerate() {
        for x in bloc.iter_mut() {
            *x = valeur as u32;
        }
    }
}
pub fn fusion(s: &[u32], d: &mut [u32], taille_blocs: usize) {
    for (x, dest) in kmerge(s.chunks(taille_blocs)).zip(d.iter_mut()) {
        *dest = *x;
    }
}

pub fn tri_fusion(tranche: &mut [u32]) {
    let taille = tranche.len();
    let mut taille_blocs = 1;
    let mut tampon: Vec<u32> = Vec::with_capacity(taille);
    unsafe {
        tampon.set_len(taille);
    }
    let mut destination = tampon.as_mut_slice();
    let mut source = tranche;
    let mut donnees_dans_tranche = true;
    println!("{:?}", destination);
    while taille_blocs < taille {
        for (b, d) in source
            .chunks(2 * taille_blocs)
            .zip(destination.chunks_mut(2 * taille_blocs))
        {
            println!("Source : {:?}", source);
            println!("Before b {:?}", b);
            println!("Before d {:?}", d);
            fusion(b, d, taille_blocs);
            println!("After b {:?}", b);
            println!("After d {:?}", d);
        }
        swap(&mut source, &mut destination);
        taille_blocs *= 2;
        donnees_dans_tranche = !donnees_dans_tranche;
    }
    println!("Source {:?}", source);
    println!("Dest {:?}", destination);
    if !donnees_dans_tranche {
        destination.copy_from_slice(source);
    }

    println!("Source : {:?}", source);
}
fn main() {
    println!("Hello, world!");
    let test = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut source = &test[..3];
    let mut dest = &test[3..7];
    println!("{:?}", source);
    println!("{:?}", dest);
    swap(&mut source, &mut dest);
    println!("{:?}", source);
    println!("{:?}", dest);

    let mut vector: Vec<u32> = Vec::with_capacity(10);
    unsafe {
        vector.set_len(10);
    }
    println!("{:?}", vector.as_mut_slice());
    println!("{:?}", vector.len());

    let mut vecctor = [0; 10];
    println!("{:?}", vecctor);

    let mut test_fusion = [1, 4, 2, 9, 7, 3, 11, 8];
    println!("{:?}", test_fusion);
    tri_fusion(&mut test_fusion);
    println!("{:?}", test_fusion);
}
