#![warn(clippy::all)]
use itertools::kmerge;
use std::iter::repeat_with;
use std::mem::swap;

/// Renvoie un vecteur de u32 aleatoire (valeurs comprises entre 0 et 99) de la taille donnee.
pub fn vecteur_aleatoire(taille: usize) -> Vec<u32> {
    repeat_with(|| rand::random::<u32>() % 100)
        .take(taille)
        .collect()
}

/// Remplit la tranche par 'taille' zeros puis 'taille' uns, ...
/// # Example:
/// ```
/// use tri_fusion::remplir_blocs;
/// let mut tableau = [0; 5];
/// remplir_blocs(&mut tableau, 1);
/// assert_eq!(tableau, [0, 1, 2, 3, 4]);
/// remplir_blocs(&mut tableau, 2);
/// assert_eq!(tableau, [0, 0, 1, 1, 2]);
/// remplir_blocs(&mut tableau, 3);
/// assert_eq!(tableau, [0, 0, 0, 1, 1]);
/// ```
pub fn remplir_blocs(tranche: &mut [u32], taille: usize) {
    let mut nb = 0;
    for chunk in tranche.chunks_mut(taille) {
        for elem in chunk.iter_mut() {
            *elem = nb;
        }
        nb = nb + 1;
    }
}

/// Fusionne les tableaux de taille `taille_blocs` contenus dans s dans d.
/// Pre-condition: tous les sous-tableaux de taille `taille_blocs` sont tries.
/// # Example:
/// ```
/// use tri_fusion::fusion;
/// let source = [0,4,2,3];
/// let mut destination = [0,0,0,0];
/// fusion(&source, &mut destination, 2);
/// assert_eq!(destination, [0,2,3,4]);
/// let source = [0,4,5,2,3];
/// let mut destination = [0,0,0,0,0];
/// fusion(&source, &mut destination, 3);
/// assert_eq!(destination, [0,2,3,4,5]);
/// let source = [3];
/// let mut destination = [0];
/// fusion(&source, &mut destination, 1);
/// assert_eq!(destination, [3]);
/// ```
pub fn fusion(s: &[u32], d: &mut [u32], taille_blocs: usize) {
    use itertools::zip;
    for (elem, dest) in kmerge(s.chunks(taille_blocs)).zip(d.iter_mut()) {
        *dest = *elem;
    }
}

/// Tri fusion iteratif
/// # Example:
/// ```
/// use tri_fusion::{vecteur_aleatoire, tri_fusion};
/// let mut v = vecteur_aleatoire(1000);
/// let mut w = v.clone();
/// v.sort();
/// tri_fusion(&mut w);
/// assert_eq!(v, w);
/// ```
pub fn tri_fusion(tranche: &mut [u32]) {
    let n = tranche.len();
    let mut taille_blocs = 1;
    let mut source = tranche; //now source becomes the original slice
    let mut destination: Vec<u32> = Vec::with_capacity(n);
    unsafe {
        destination.set_len(n);
    }
    let mut dest = destination.as_mut_slice();
    let mut original_in_tranche: bool = true; //if false, it means dest is the tranche
    while taille_blocs < n {
        for (b, d) in source
            .chunks(2 * taille_blocs)
            .zip(dest.chunks_mut(2 * taille_blocs))
        {
            fusion(b, d, taille_blocs);
        }
        taille_blocs *= 2;
        swap(&mut source, &mut dest);
        original_in_tranche = !original_in_tranche;
    }

    if !original_in_tranche {
        //dest is the original tranche, and the sorted array is in source
        dest.copy_from_slice(source);
    }
}
