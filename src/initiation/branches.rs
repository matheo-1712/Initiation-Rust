fn branches() {
    let condition = true;
    let nombre = if condition { 5 } else { 6 };

    if nombre < 5 {
        println!("La condition est vérifiée");
    } else {
        println!("La condition n'est pas vérifiée");
    }
}