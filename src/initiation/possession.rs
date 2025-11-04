fn possession() {
    {
        let s = String::from("hello"); // s est en vigueur à partir de ce point
        // on fait des choses avec s ici
        println!("{}", s);
    } // cette portée est désormais terminée, et s n'est plus en vigueur maintenant

    /** Interdit en rust
        let s1 = String::from("hello");
        let s2 = s1;

        println!("{}, world!", s1);
    **/
    // Bonne pratique :
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s1 = donne_possession();     // donne_possession déplace sa valeur de
    // retour dans s1

    let s2 = String::from("hello");  // s2 rentre dans la portée

    let s3 = prend_et_rend(s2);      // s2 est déplacée dans
    // prend_et_rend, qui elle aussi
    // déplace sa valeur de retour dans s3.

    println!("s3 = {}", s3);

}

// Ici, s3 sort de la portée et est éliminée. s2 a été déplacée donc il ne se
// passe rien. s1 sort aussi de la portée et est éliminée.

fn donne_possession() -> String { // donne_possession va déplacer sa
    // valeur de retour dans la
    // fonction qui l'appelle.

    let texte = String::from("yours");   // texte rentre dans la portée.

    texte                                // texte est retournée et
    // est déplacée vers le code qui
    // l'appelle.
}

// Cette fonction va prendre une String et en retourne aussi une.
fn prend_et_rend(texte: String) -> String { // texte rentre dans la portée.

    texte  // texte est retournée et déplacée vers le code qui l'appelle.
}