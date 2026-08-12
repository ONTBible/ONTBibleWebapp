//! Le design system — la forme, sans le propos.
//!
//! Un composant par fichier, et un fichier par composant. Chacun ignore ce
//! qu'il affiche : `Filet` ne sait pas s'il sépare deux versets ou deux
//! sections ; `Section` ne sait pas ce qu'elle contient. C'est ce qui permet
//! de les composer sans les modifier — et c'est la seule discipline qui
//! empêche un design system de devenir une collection de cas particuliers.
//!
//! Depuis le passage à Tailwind, la forme d'un composant vit **dans son
//! fichier**, à côté de son balisage. Il n'y a plus de feuille de style à
//! tenir en parallèle, donc plus de moyen qu'elles divergent.
//!
//! Les valeurs, elles, ne s'écrivent jamais ici : couleurs, fontes, échelle et
//! mesure viennent des jetons de `style/main.css`.

mod bandeau;
mod carte_verset;
mod entete;
mod exergue;
mod filet;
mod legende_niveaux;
mod liste_affirmations;
mod mention;
mod page_legale;
mod pied;
mod porte;
mod portes;
mod portrait;
mod principe;
mod section;
mod verset;

pub use bandeau::Bandeau;
pub use carte_verset::CarteVersetDuJour;
pub use entete::Entete;
pub use exergue::Exergue;
pub use filet::Filet;
pub use legende_niveaux::LegendeNiveaux;
pub use liste_affirmations::ListeAffirmations;
pub use mention::Mention;
pub use page_legale::PageLegale;
pub use pied::PiedDePage;
pub use porte::Porte;
pub use portes::Portes;
pub use portrait::Portrait;
pub use principe::Principe;
pub use section::Section;
pub use verset::Verset;
