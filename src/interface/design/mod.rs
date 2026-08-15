//! Le design system — la forme, sans le propos.
//!
//! Un composant par fichier, et un fichier par composant. Chacun ignore ce
//! qu'il affiche : `Bloc` ne sait pas ce qu'il contient, `Portrait` ne sait pas
//! qui il montre. C'est ce qui permet de les composer sans les modifier — et
//! c'est la seule discipline qui empêche un design system de devenir une
//! collection de cas particuliers.
//!
//! ## Rien ne reste ici qui ne serve
//!
//! Un composant que plus aucune page n'emploie est retiré, pas conservé « au
//! cas où ». Sinon quelqu'un le retrouve six mois plus tard, le croit partie du
//! système, et bâtit dessus. `TitreDePage`, `Filet` et `Mention` sont sortis
//! comme ça, chacun après une refonte qui les avait rendus inutiles.
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
mod blocs;
mod bouton;
mod carte_verset;
mod chiffres;
mod comparaison;
mod entete;
mod exergue;
mod hero;
mod image;
mod legende_niveaux;
mod lien;
mod liste_affirmations;
mod liste_unites;
mod mention_brouillon;
mod occurrences;
mod page_de_lecture;
mod page_legale;
mod pied;
mod porte;
mod portrait;
mod principe;
mod reglages_de_lecture;
mod sommaire;
mod titre_de_section;
pub mod verset;

pub use bloc::Bloc;
pub use blocs::Blocs;
pub use bouton::Bouton;
pub use carte_verset::CarteVersetDuJour;
pub use chiffres::Chiffres;
pub use comparaison::Comparaison;
pub use entete::Entete;
pub use exergue::Exergue;
pub use hero::Hero;
pub use image::image;
pub use legende_niveaux::LegendeNiveaux;
pub use lien::Lien;
pub use liste_affirmations::ListeAffirmations;
pub use liste_unites::ListeDUnites;
pub use mention_brouillon::MentionBrouillon;
pub use occurrences::Occurrences;
pub use page_de_lecture::PageDeLecture;
pub use page_legale::PageLegale;
pub use pied::PiedDePage;
pub use porte::{traverser, Porte};
pub use portrait::Portrait;
pub use principe::Principe;
pub use reglages_de_lecture::{fournir_preferences, preferences, ReglagesDeLecture};
pub use sommaire::Sommaire;
pub use titre_de_section::TitreDeSection;
pub use verset::Verset;
