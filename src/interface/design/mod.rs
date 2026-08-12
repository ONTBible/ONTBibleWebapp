//! Le design system — la forme, sans le propos.
//!
//! Un composant par fichier, et un fichier par composant. Chacun ignore ce
//! qu'il affiche : `Filet` ne sait pas s'il sépare deux versets ou deux
//! sections ; `Bloc` ne sait pas ce qu'il contient. C'est ce qui permet de les
//! composer sans les modifier — et c'est la seule discipline qui empêche un
//! design system de devenir une collection de cas particuliers.
//!
//! Depuis Tailwind, la forme d'un composant vit **dans son fichier**, à côté de
//! son balisage : il n'y a plus de feuille parallèle qui puisse diverger. Les
//! valeurs, elles, ne s'écrivent jamais ici — couleurs, fontes, échelle et
//! mesure viennent des jetons de `style/main.css`.
//!
//! ## La mise en page
//!
//! `Hero` ouvre, `Bloc` fait tout le reste. Une seule primitive de mise en
//! page : deux finissent toujours par diverger sur un espacement, et personne
//! ne sait plus laquelle fait foi.

mod bloc;
mod bouton;
mod carte_verset;
mod chiffres;
mod comparaison;
mod entete;
mod exergue;
mod filet;
mod hero;
mod legende_niveaux;
mod liste_affirmations;
mod mention;
mod page_legale;
mod pied;
mod portrait;
mod principe;
mod titre_de_page;
mod titre_de_section;
mod verset;

pub use bloc::Bloc;
pub use bouton::Bouton;
pub use carte_verset::CarteVersetDuJour;
pub use chiffres::Chiffres;
pub use comparaison::Comparaison;
pub use entete::Entete;
pub use exergue::Exergue;
pub use filet::Filet;
pub use hero::Hero;
pub use legende_niveaux::LegendeNiveaux;
pub use liste_affirmations::ListeAffirmations;
pub use mention::Mention;
pub use page_legale::PageLegale;
pub use pied::PiedDePage;
pub use portrait::Portrait;
pub use principe::Principe;
pub use titre_de_page::TitreDePage;
pub use titre_de_section::TitreDeSection;
pub use verset::Verset;
