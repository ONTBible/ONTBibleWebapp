//! Les pages — le propos.
//!
//! Une page assemble des composants du design system et n'écrit aucune valeur
//! de style : ni couleur, ni espacement, ni taille. Si une page a besoin d'une
//! forme qui n'existe pas, la forme se crée dans `design`, pas ici.
//!
//! Les pages légales font exception, et c'est assumé : un texte juridique est
//! écrit en balises ordinaires, et exiger un composant par paragraphe le
//! rendrait illisible à écrire comme à relire. `PageLegale` porte leur forme.

mod accueil;
// Compilé mais non routé : sa page attend une relecture. Voir `app.rs`.
#[allow(dead_code)]
mod auteur;
mod conditions;
mod fiche;
mod lexique;
mod lire;
mod livre;
pub mod confidentialite;
mod negations;
mod passage;
mod pourquoi;

pub use accueil::Accueil;
pub use conditions::Conditions;
pub use fiche::Fiche;
pub use lexique::Lexique;
pub use lire::Lire;
pub use livre::Livre;
pub use confidentialite::Confidentialite;
pub use negations::Negations;
pub use passage::Passage;
pub use pourquoi::Pourquoi;
