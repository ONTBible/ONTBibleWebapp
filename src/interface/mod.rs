//! L'interface — ce que le lecteur voit.
//!
//! Elle dépend du domaine, jamais l'inverse. Un composant peut appeler
//! `domaine::verset_du_jour::indice` ; le domaine ne sait pas qu'une page
//! existe.
//!
//! ## Le design system
//!
//! `design` porte les composants de forme — filets, titres, cartes — et rien
//! d'autre : ils ne savent pas ce qu'ils affichent. `pages` porte le propos, et
//! n'écrit aucun style. La séparation tient tant que personne n'écrit de
//! couleur ni d'espacement dans une page.
//!
//! Les **jetons** eux-mêmes ne sont pas ici : ils vivent dans
//! `style/_jetons.scss`, en propriétés personnalisées CSS. Une seule source,
//! et c'est la feuille de style — la dupliquer en constantes Rust créerait
//! deux vérités qui divergeraient à la première retouche.

pub mod app;
pub mod design;
pub mod pages;
