include!("initiation/jeu_nombres.rs");
// main.rs

fn main() {
    loop {
        println!("=== MENU ===");
        println!("1. Jeu de devinette (numbers)");
        println!("2. Autre fonction (exemple)");
        println!("3. Quitter");

        let mut choix = String::new();
        println!("Entrez votre choix :");
        io::stdin()
            .read_line(&mut choix)
            .expect("Erreur de lecture");

        let choix = choix.trim(); // enlever \n et espaces

        match choix {
            "1" => numbers(),
            "2" => numbers(),
            "3" => {
                println!("Au revoir !");
                break; // sortir de la boucle et terminer le programme
            }
            _ => println!("Choix invalide, réessayez."),
        }
    }
}