use rand::random;
use rayon_logs::prelude::*;
use rayon_logs::subgraph;

type Bins = Vec<(f64, Vec<f64>)>;

fn fusion(mut bins1: Bins, mut bins2: Bins) -> Bins {
    // subgraph permet de tagger les taches dans rayon_logs
    subgraph("fusion", bins1.len() + bins2.len(), || {
        // on va fusionner les deux bins a moitie vides
        // si elles existent
        let max1 = bins1
            .iter()
            .map(|&(s, _)| s)
            .enumerate()
            .max_by(|(_, sa), (_, sb)| sa.partial_cmp(&sb).unwrap()) // f64 n'implemente pas Ord
            .map(|(i, _)| i);

        let max2 = bins2
            .iter()
            .map(|&(s, _)| s)
            .enumerate()
            .max_by(|(_, sa), (_, sb)| sa.partial_cmp(&sb).unwrap())
            .map(|(i, _)| i);

        if let Some(i1) = max1 {
            if let Some(i2) = max2 {
                if bins1[i1].0 + bins2[i2].0 >= 1.0 {
                    let bin2 = bins2.swap_remove(i2);
                    bins1[i1].1.extend(bin2.1);
                    bins1[i1].0 += bin2.0 - 1.0;
                }
            }
        }
        bins1.extend(bins2);
        bins1
    })
}

fn bin_packing() {
    let tailles: Vec<f64> = (0..32).into_par_iter().map(|_| random()).collect();
    // on calcule les bins.
    // il s'agit d'un vecteur tel que chaque case represente une bin.
    // elle contient
    // - la place restante
    // - un vecteur contenant tout le contenu de la bin.
    let bins: Bins = tailles
        .par_iter()
        .cloned()
        .for_each(
            || -> Bins { Vec::new() },
            |mut bins, t| {
                let choix = bins
                    .iter_mut()
                    .filter(|(place_libre, _)| *place_libre >= t)
                    .next(); // on prend la premiere bin avec assez de place
                if let Some((place_libre, contenu)) = choix {
                    *place_libre -= t;
                    contenu.push(t);
                } else {
                    // si choix est None on rajoute une nouvelle bin.
                    bins.push((1.0 - t, vec![t]));
                }
                bins
            },
        )
        .reduce(Vec::new, |b1, b2| fusion(b1, b2));
    // assert!(bins
    //    .iter()
    //    .all(|(s, t)| (1.0 - t.iter().sum::<f64>() - *s).abs() <= 0.0001 && *s <= 1.0));
    // assert!(bins.iter().filter(|(s, _)| *s >= 0.5).count() <= 1);
}

fn main() {
    let pool = rayon_logs::ThreadPoolBuilder::new()
        .build()
        .expect("erreur de creation du pool");
    let (_, log) = pool.logging_install(bin_packing);
    log.save_svg("fold.svg")
        .expect("erreur de sauvegarde du svg");
}
