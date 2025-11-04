/**
Les règles de référencement
Récapitulons ce que nous avons vu à propos des références :

À un instant donné, vous pouvez avoir soit une référence mutable, soit un nombre quelconque de références immuables.
Les références doivent toujours être en vigueur.
Ensuite, nous aborderons un autre type de référence : les slices.
**/
fn references() {
    let s1 = String::from("helloooooooooooooooooooooooo");

    let long = calculer_taille(&s1);

    println!("La taille de '{}' est {}.", s1, long);
}

fn calculer_taille(s: &String) -> usize {
    s.len()
}