//! L'interface — ce que le lecteur voit.
//!
//! Elle dépend du domaine et de l'application, jamais l'inverse.
//!
//! `design` porte les composants de forme — un par fichier — et ils ignorent
//! ce qu'ils affichent. `pages` porte le propos, et n'écrit aucune valeur de
//! style. La séparation tient tant qu'une page ne contient ni couleur, ni
//! espacement, ni taille.

pub mod app;
pub mod design;
pub mod echantillon;
pub mod pages;
pub mod tete;
