use rand::Rng;  // Pour générer des labyrinthes aléatoires

#[derive(Debug)]
pub struct Labyrinthe {
    pub largeur: usize,
    pub hauteur: usize,
    pub grille: Vec<Vec<char>>,  // ' ' pour vide, '#' pour mur
}

impl Labyrinthe {
    // Constructeur pour créer un labyrinthe de dimensions données
    pub fn new(largeur: usize, hauteur: usize) -> Self {
        let mut grille = vec![vec!['#'; largeur]; hauteur];
        // Générer le labyrinthe ici (par exemple, avec un algorithme de génération aléatoire)
        Self { largeur, hauteur, grille }
    }

    // Méthode pour afficher le labyrinthe (utile pour le debug)
    pub fn afficher(&self) {
        for ligne in &self.grille {
            println!("{}", ligne.iter().collect::<String>());
        }
    }

    // Méthode pour récupérer une case spécifique (ex. un mur ou un espace libre)
    pub fn get_case(&self, x: usize, y: usize) -> Option<char> {
        if x < self.largeur && y < self.hauteur {
            Some(self.grille[y][x])
        } else {
            None
        }
    }
    
    // Autres méthodes pour manipuler ou afficher le labyrinthe...
}