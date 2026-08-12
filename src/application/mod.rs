//! L'application — ce que le site *fait*, sans savoir avec quoi.
//!
//! Cette couche déclare des **ports** : ce dont un cas d'usage a besoin,
//! exprimé en traits. Elle ne connaît aucune implémentation. Savoir l'heure
//! est un besoin ; lire l'horloge du système en est une réalisation, et elle
//! vit dans `infrastructure`.
//!
//! Ce n'est pas de la cérémonie. C'est ce qui permet d'éprouver « quel verset
//! le 12 août ? » sans attendre le 12 août — et donc d'avoir une réponse
//! vérifiée plutôt qu'espérée.

pub mod ports;
pub mod verset_du_jour;
