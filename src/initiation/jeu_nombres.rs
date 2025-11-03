use std::cmp::Ordering;
use std::io;
use rand::Rng;

// Jeu du nombre plus petit ou plus grand
fn numbers() {

    let mut rng = rand::thread_rng();
    let nombre_secret = rng.gen_range(1..=100);

    println!("Devinez le nombre !");

    // Boucle infinie
    loop {
        println!("Veuillez entrer un nombre :");

        let mut supposition = String::new();

        // Lit la saisie de l'utilisateur
        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");

        let supposition: u32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        // On vérifie si le numéro est inférieur ou supérieur
        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break
            },
        }
    }
}