// Structures et méthodes en Rust (équivalent des classes dans d'autres langages)
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}
// Méthodes
impl Rectangle {
    fn aire(&self) -> u32 {
        self.largeur * self.hauteur
    }
}
// Fin de la classe Rectangle

fn structure() {
    let rect1 = Rectangle { largeur: 30, hauteur: 50 };

    println!(
        "L'aire du rectangle est de {} pixels carrés.",
        rect1.aire()
    );
}

