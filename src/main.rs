use rand::Rng;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write};
use prettytable::{Table, row, Cell, Attr, color};

// ===== STRUCTURES ET TYPES =====

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Type {
    Feu,
    Eau,
    Plante,
    Electrik,
    Roche,
    Psy,
    Vol,
    Insecte,
    Normal,
    Combat,
    Poison,
    Spectre,
    Dragon,
    Glace,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Genre {
    Male,
    Femelle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Pokemon {
    nom: String,
    niveau: u8,
    type_: Type,
    xp: u32,
    genre: Genre,
    evolution: Option<String>,
    niveau_evolution: Option<u8>,
}

#[derive(Serialize, Deserialize)]
struct Elevage {
    pokemons: Vec<Pokemon>,
}

// ===== IMPLEMENTATIONS DES MÉTHODES =====

impl Pokemon {
    fn gagner_xp(&mut self, montant: u32) {
        self.xp += montant;
        let mut evolution_declenchee = false;
        
        while self.xp >= 100 {
            self.niveau += 1;
            self.xp -= 100;
            println!("{} est passé au niveau {} !", self.nom, self.niveau);
            
            // Vérifier si le Pokémon peut évoluer
            if let Some(niveau_requis) = self.niveau_evolution {
                if self.niveau >= niveau_requis && !evolution_declenchee {
                    if let Some(evolution) = &self.evolution {
                        println!("\n⭐️ ÉVOLUTION ! ⭐️");
                        println!("{} évolue en {} !", self.nom, evolution);
                        
                        // Changer le nom du Pokémon pour celui de son évolution
                        self.nom = evolution.clone();
                        // Réinitialiser l'évolution pour qu'elle ne se produise qu'une fois
                        self.evolution = None;
                        self.niveau_evolution = None;
                        
                        evolution_declenchee = true;
                    }
                }
            }
        }
    }

    fn peut_se_reproduire_avec(&self, autre: &Pokemon) -> bool {
        // true si types identiques ET genre différents ET niveau >= 5 pour les deux
        self.type_ == autre.type_ && self.genre != autre.genre && self.niveau >= 5 && autre.niveau >= 5
    }
}

impl Elevage {
    fn new() -> Self {
        Self {
            pokemons: Vec::new(),
        }
    }

    fn ajouter_pokemon(&mut self, pokemon: Pokemon) {
        println!("Ajout de {} à l'élevage.", pokemon.nom);
        self.pokemons.push(pokemon);
    }

    fn afficher_tous(&self) {
        if self.pokemons.is_empty() {
            println!("L'élevage est vide 🫠");
        } else {
            println!("=== Pokémons dans l'élevage ===\n");
            
            // Création du tableau avec prettytable
            let mut table = creer_tableau_pokemon_avec_indice();
            
            // Contenu du tableau
            for (i, p) in self.pokemons.iter().enumerate() {
                ajouter_pokemon_au_tableau(&mut table, p, Some(i + 1));
            }
            
            // Affichage du tableau
            table.printstd();
        }
    }

    fn entrainer_tous(&mut self, xp: u32) {
        println!("Entraînement collectif : +{} XP pour tout le monde !", xp);
        for p in self.pokemons.iter_mut() {
            p.gagner_xp(xp);
        }
    }

    fn tenter_reproduction(&mut self, i1: usize, i2: usize) {
        if i1 >= self.pokemons.len() || i2 >= self.pokemons.len() {
            println!("Indice invalide pour la reproduction !");
            return;
        }

        let p1 = &self.pokemons[i1];
        let p2 = &self.pokemons[i2];

        // Afficher les deux Pokémon dans un tableau
        println!("\n=== Pokémons sélectionnés pour la reproduction ===\n");
        let mut table = creer_tableau_pokemon();
        
        // Ajouter les deux Pokémon au tableau
        for &idx in &[i1, i2] {
            let p = &self.pokemons[idx];
            ajouter_pokemon_au_tableau(&mut table, p, None);
        }
        
        // Afficher le tableau
        table.printstd();

        match tenter_reproduction(p1, p2) {
            Some(bebe) => {
                println!("Un nouveau Pokémon est né !");
                
                // Afficher le nouveau Pokémon dans un tableau prettytable
                println!("\n=== Nouveau Pokémon né ===\n");
                let mut table = creer_tableau_pokemon();
                
                // Ajouter le nouveau Pokémon au tableau
                ajouter_pokemon_au_tableau(&mut table, &bebe, None);
                
                // Afficher le tableau
                table.printstd();
                
                self.ajouter_pokemon(bebe);
            }
            None => {
                println!("La reproduction a échoué.");
            }
        }
    }

    fn sauvegarder(&self, chemin: &str) {
        match File::create(chemin) {
            Ok(mut file) => {
                let data = serde_json::to_string_pretty(&self).unwrap();
                file.write_all(data.as_bytes()).unwrap();
                println!("✅ Élevage sauvegardé dans {}", chemin);
            }
            Err(e) => {
                println!("❌ Erreur de sauvegarde : {}", e);
            }
        }
    }

    fn charger(chemin: &str) -> Option<Self> {
        match std::fs::read_to_string(chemin) {
            Ok(content) => {
                let elevage: Elevage = serde_json::from_str(&content).unwrap();
                println!("✅ Élevage chargé depuis {}", chemin);
                Some(elevage)
            }
            Err(_) => {
                println!("⚠️ Impossible de charger le fichier '{}'.", chemin);
                None
            }
        }
    }
}

// ===== FONCTIONS UTILITAIRES =====

fn lire_choix() -> String {
    print!("> ");
    io::stdout().flush().unwrap(); // Affiche l'invite avant de bloquer
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();
    choix.trim().to_string()
}

fn attendre_touche() {
    println!("");
    println!("Appuiez sur une touche pour revenir au menu...");
    let mut _dummy: String = String::new();
    io::stdin().read_line(&mut _dummy).expect("Erreur de lecture");
}

// ===== FONCTIONS DE GESTION DE PRETTYTABLE =====

fn creer_tableau_pokemon() -> Table {
    let mut table = Table::new();
    
    // En-tête avec style
    table.add_row(row![
        FBb->"Nom", 
        FBb->"Niveau", 
        FBb->"XP", 
        FBb->"Type", 
        FBb->"Genre", 
        FBb->"Évolution"
    ]);
    
    table
}

fn creer_tableau_pokemon_avec_indice() -> Table {
    let mut table = Table::new();
    
    // En-tête avec style
    table.add_row(row![
        FBb->"#", 
        FBb->"Nom", 
        FBb->"Niveau", 
        FBb->"XP", 
        FBb->"Type", 
        FBb->"Genre", 
        FBb->"Évolution"
    ]);
    
    table
}

fn obtenir_cellule_type(type_: &Type) -> Cell {
    match type_ {
        Type::Feu => Cell::new(&format!("{:?}", type_)).with_style(Attr::ForegroundColor(color::RED)),
        Type::Eau => Cell::new(&format!("{:?}", type_)).with_style(Attr::ForegroundColor(color::BLUE)),
        Type::Plante => Cell::new(&format!("{:?}", type_)).with_style(Attr::ForegroundColor(color::GREEN)),
        Type::Electrik => Cell::new(&format!("{:?}", type_)).with_style(Attr::ForegroundColor(color::YELLOW)),
        _ => Cell::new(&format!("{:?}", type_))
    }
}

fn ajouter_pokemon_au_tableau(table: &mut Table, pokemon: &Pokemon, indice: Option<usize>) {
    let evolution = pokemon.evolution.clone().unwrap_or_else(|| "Aucune".to_string());
    let type_cell = obtenir_cellule_type(&pokemon.type_);
    
    if let Some(idx) = indice {
        table.add_row(row![
            idx,
            pokemon.nom,
            pokemon.niveau,
            pokemon.xp,
            type_cell,
            format!("{:?}", pokemon.genre),
            evolution
        ]);
    } else {
        table.add_row(row![
            pokemon.nom,
            pokemon.niveau,
            pokemon.xp,
            type_cell,
            format!("{:?}", pokemon.genre),
            evolution
        ]);
    }
}

// ===== FONCTIONS LIÉES AUX POKÉMON =====

fn tenter_reproduction(p1: &Pokemon, p2: &Pokemon) -> Option<Pokemon> {
    if p1.peut_se_reproduire_avec(p2) {
        let mut rng = rand::thread_rng();
        let genre = if rng.gen_bool(0.5) {
            Genre::Male
        } else {
            Genre::Femelle
        };

        // Demander le nom du nouveau Pokémon
        println!("Quel nom voulez-vous donner au nouveau Pokémon ?");
        let nom = lire_choix();
        let nom = if nom.is_empty() { "Mystère".to_string() } else { nom };

        // Demander l'évolution potentielle
        println!("Quelle sera son évolution ? (Laissez vide si aucune)");
        let evolution_nom = lire_choix();
        let evolution = if evolution_nom.is_empty() { 
            None 
        } else { 
            Some(evolution_nom) 
        };
        
        // Si une évolution est définie, demander le niveau requis
        let niveau_evolution = if evolution.is_some() {
            println!("À quel niveau évoluera-t-il ? (Par défaut: 15)");
            let niveau_str = lire_choix();
            if niveau_str.is_empty() {
                Some(15)
            } else {
                Some(niveau_str.parse().unwrap_or(15))
            }
        } else {
            None
        };

        let bebe = Pokemon {
            nom,
            niveau: 1,
            xp: 0,
            type_: p1.type_.clone(), // même type que les parents
            genre,
            evolution,
            niveau_evolution,
        };

        Some(bebe)
    } else {
        None
    }
}

// ===== FONCTIONS DE L'INTERFACE UTILISATEUR =====

fn afficher_menu() {
    println!("\n=== Menu de l'Élevage Pokémon ===");
    println!("1. Afficher tous les Pokémon");
    println!("2. Ajouter un Pokémon manuellement");
    println!("3. Entraîner tous les Pokémon");
    println!("4. Tenter une reproduction");
    println!("5. Sauvegarder dans un fichier");
    println!("6. Charger depuis un fichier");
    println!("7. Quitter");
}

fn afficher_pokemon(elevage: &Elevage) {
    elevage.afficher_tous();
}

fn ajouter_pokemon(elevage: &mut Elevage) {
    println!("Nom du Pokémon :");
    let nom = lire_choix();

    println!("Choisis un type : Feu, Eau, Plante, Electrik, Roche, Psy, Vol, Insecte, Normal, Combat, Poison, Spectre, Dragon, Glace");
    let type_str = lire_choix();
    let type_ = match type_str.to_lowercase().as_str() {
        "feu" => Type::Feu,
        "eau" => Type::Eau,
        "plante" => Type::Plante,
        "electrik" => Type::Electrik,
        "roche" => Type::Roche,
        "psy" => Type::Psy,
        "vol" => Type::Vol,
        "insecte" => Type::Insecte,
        "normal" => Type::Normal,
        "combat" => Type::Combat,
        "poison" => Type::Poison,
        "spectre" => Type::Spectre,
        "dragon" => Type::Dragon,
        "glace" => Type::Glace,
        _ => {
            println!("Type inconnu, ce sera Normal.");
            Type::Normal
        }
    };

    println!("Genre (1: Male, 2: Femelle, 3: Aléatoire) :");
    let genre_choix = lire_choix();
    let genre = match genre_choix.as_str() {
        "1" => Genre::Male,
        "2" => Genre::Femelle,
        "3" | "" => {
            // Option aléatoire (par défaut)
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.5) {
                println!("Genre aléatoire choisi : Male");
                Genre::Male
            } else {
                println!("Genre aléatoire choisi : Femelle");
                Genre::Femelle
            }
        },
        _ => {
            println!("Choix non reconnu, genre aléatoire attribué.");
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.5) {
                println!("Genre aléatoire choisi : Male");
                Genre::Male
            } else {
                println!("Genre aléatoire choisi : Femelle");
                Genre::Femelle
            }
        }
    };
    
    // Ajouter l'évolution
    println!("Quelle sera son évolution ? (Laissez vide si aucune)");
    let evolution_nom = lire_choix();
    let evolution = if evolution_nom.is_empty() { 
        None 
    } else { 
        Some(evolution_nom) 
    };
    
    // Si une évolution est définie, demander le niveau requis
    let niveau_evolution = if evolution.is_some() {
        println!("À quel niveau évoluera-t-il ? (Par défaut: 15)");
        let niveau_str = lire_choix();
        if niveau_str.is_empty() {
            Some(15)
        } else {
            Some(niveau_str.parse().unwrap_or(15))
        }
    } else {
        None
    };

    let nouveau = Pokemon {
        nom,
        niveau: 1,
        xp: 0,
        type_,
        genre,
        evolution,
        niveau_evolution,
    };

    elevage.ajouter_pokemon(nouveau);
}

fn entrainer_pokemon(elevage: &mut Elevage) {
    println!("Combien de points d'XP veux-tu distribuer ?");
    let xp: u32 = lire_choix().parse().unwrap_or(50);
    elevage.entrainer_tous(xp);
}

fn reproduction_pokemon(elevage: &mut Elevage) -> bool {
    elevage.afficher_tous(); // Afficher les Pokémons pour voir les indices
    
    println!("Indice du 1er Pokémon ?");
    let i1_input: usize = lire_choix().parse().unwrap_or(0);
    
    // Ajuster l'indice (les indices commencent à 1 pour l'utilisateur, mais à 0 dans le vecteur)
    let i1 = if i1_input > 0 { i1_input - 1 } else { 0 };
    
    // Vérifier si l'indice est valide et afficher le nom
    if i1 >= elevage.pokemons.len() {
        println!("Indice invalide !");
        return false;
    }
    println!("Pokémon sélectionné : {}", elevage.pokemons[i1].nom);
    
    println!("Indice du 2ème Pokémon ?");
    let i2_input: usize = lire_choix().parse().unwrap_or(0);
    
    // Ajuster l'indice
    let i2 = if i2_input > 0 { i2_input - 1 } else { 0 };
    
    // Vérifier si l'indice est valide et afficher le nom
    if i2 >= elevage.pokemons.len() {
        println!("Indice invalide !");
        return false;
    }
    println!("Pokémon sélectionné : {}", elevage.pokemons[i2].nom);
    
    // Afficher les deux Pokémon dans un tableau
    println!("\n=== ❤️ Pokémons sélectionnés pour la reproduction ❤️ ===\n");
    let mut table = creer_tableau_pokemon();
    
    // Ajouter les deux Pokémon au tableau
    for &idx in &[i1, i2] {
        let p = &elevage.pokemons[idx];
        ajouter_pokemon_au_tableau(&mut table, p, None);
    }
    
    // Afficher le tableau
    table.printstd();
    
    // Tenter la reproduction
    elevage.tenter_reproduction(i1, i2);
    
    true
}

fn sauvegarder_elevage(elevage: &Elevage) {
    elevage.sauvegarder("elevage.json");
}

fn charger_elevage(elevage: &mut Elevage) {
    if let Some(chargé) = Elevage::charger("elevage.json") {
        *elevage = chargé;
    }
}

// ===== FONCTION PRINCIPALE =====

fn main() {
    let mut elevage = Elevage::new();
    Elevage::charger("elevage.json");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    loop {
        afficher_menu();

        let choix = lire_choix();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        match choix.as_str() {
            "1" => afficher_pokemon(&elevage),
            "2" => ajouter_pokemon(&mut elevage),
            "3" => entrainer_pokemon(&mut elevage),
            "4" => {
                if !reproduction_pokemon(&mut elevage) {
                    continue;
                }
            },
            "5" => sauvegarder_elevage(&elevage),
            "6" => charger_elevage(&mut elevage),
            "7" => {
                println!("À bientôt, dresseur !");
                break;
            }
            _ => println!("Choix invalide, réessaie."),
        }
        attendre_touche();
    }
}