use std::io;
use rand::Rng;

// main.rs
fn main() { numbers();}

fn numbers() {

    let mut rng = rand::thread_rng();
    let nombre = rng.gen_range(1..=100);

    println!("Devinez le nombre !");

    loop {
        println!("Veuillez entrer un nombre :");

        let mut saisie = String::new();

        io::stdin()
            .read_line(&mut saisie)
            .expect("Échec de la lecture de l'entrée utilisateur");

        // Convertit la saisie en i32
        let supposition: i32 = match saisie.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide !");
                continue; // redemande une saisie
            }
        };

        if supposition < nombre {
            println!("Trop petit !");
        } else if supposition > nombre {
            println!("Trop grand !");
        } else if supposition == nombre {
            println!("Bravo ! Vous avez trouvé 🎉");
            break;
        } else {
            println!("Erreur !");
            break;
        }
    }
}