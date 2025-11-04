// une_autre_fonction.rs
fn une_autre_fonction(x :i32) {
    println!("C'est une autre {} fonction !", {x});
    afficher_mesure_avec_unite(x, 'm');
    println!("{}", get5());
}
fn afficher_mesure_avec_unite(valeur: i32, unite: char) {
    println!("La mesure est : {}{}", valeur, unite);
}
fn get5() -> i32 {
    5
}
