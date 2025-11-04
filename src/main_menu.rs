include!("initiation/jeu_nombres.rs");
include!("initiation/variables.rs");
include!("initiation/une_autre_fonction.rs");
include!("initiation/branches.rs");
include!("initiation/possession.rs");
include!("initiation/references.rs");

// main_menu.rs
fn main_menu() {
    loop {
        println!("=== MENU ===");
        println!("1. Jeu de devinette (numbers)");
        println!("2. Variables (variables)");
        println!("3. Une autre fonction (une_autre_fonction)");
        println!("4. Branches (branches)");
        println!("5. Possession (possession)");
        println!("6. References (references)");
        println!("10. Quitter");

        let mut choix = String::new();
        println!("Entrez votre choix :");
        io::stdin()
            .read_line(&mut choix)
            .expect("Erreur de lecture");

        let choix = choix.trim(); // enlever \n et espaces

        match choix {
            "1" => numbers(),
            "2" => variables(),
            "3" => une_autre_fonction(32),
            "4" => branches(),
            "5" => possession(),
            "6" => references(),
            "10" => {
                println!("Au revoir !");
                break; // sortir de la boucle et terminer le programme
            }
            _ => println!("Choix invalide, réessayez."),
        }
    }
}