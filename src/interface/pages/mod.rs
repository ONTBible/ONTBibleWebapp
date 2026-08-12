//! Les pages — le propos.
//!
//! Une page assemble des composants du design system et n'écrit aucun style :
//! ni couleur, ni espacement, ni taille. Si une page a besoin d'une forme qui
//! n'existe pas, la forme se crée dans `design`, pas ici.

mod accueil;

pub use accueil::Accueil;
