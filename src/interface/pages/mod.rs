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
mod auteur;
mod conditions;
pub mod confidentialite;
mod negations;
mod pourquoi;

pub use accueil::Accueil;
pub use auteur::Auteur;
pub use conditions::Conditions;
pub use confidentialite::Confidentialite;
pub use negations::Negations;
pub use pourquoi::Pourquoi;
