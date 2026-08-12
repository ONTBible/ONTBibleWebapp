//! Le design system — la forme, sans le propos.
//!
//! Chaque composant d'ici ignore ce qu'il affiche. `Filet` ne sait pas s'il
//! sépare deux versets ou deux sections ; `Section` ne sait pas ce qu'elle
//! contient. C'est ce qui permet de les composer sans les modifier — et c'est
//! la seule discipline qui empêche un design system de se transformer en
//! collection de cas particuliers.
//!
//! Les couleurs et les espacements ne s'écrivent jamais ici en dur : ils
//! viennent de `style/_jetons.scss`, par des classes.

mod filet;
mod section;

pub use filet::Filet;
pub use section::Section;
