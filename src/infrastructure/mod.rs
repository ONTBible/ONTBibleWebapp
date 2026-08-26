//! L'infrastructure — les réalisations concrètes des ports.
//!
//! Tout ce qui touche au monde extérieur vit ici, et nulle part ailleurs :
//! l'horloge, les fichiers, le réseau. C'est aussi la seule couche qui
//! n'existe pas dans le navigateur — d'où le drapeau `ssr`.
//!
//! Le sens de la dépendance ne s'inverse jamais : l'infrastructure connaît
//! l'application, l'application ne la connaît pas.

pub mod comptes;
pub mod corpus;
pub mod horloge;
pub mod vivier;
