//! Les ports — ce dont l'application a besoin, et rien de plus.
//!
//! Chaque trait est taillé au besoin d'un seul cas d'usage. C'est le `I` de
//! SOLID : un port large obligerait ses implémentations à porter des méthodes
//! dont personne ne se sert, et un faux de test à les inventer.

use crate::domaine::vivier::VersetQuotidien;

/// Savoir quel jour on est.
///
/// Le port ne rend ni une date ni une heure : un **numéro de jour**, compté
/// depuis le 1ᵉʳ janvier 1970. C'est tout ce dont le choix du verset a besoin,
/// et ça évite de faire entrer un type de date dans le domaine.
pub trait Horloge: Send + Sync {
    fn jour(&self) -> i64;
}

/// Accéder au vivier des versets du jour.
pub trait Vivier: Send + Sync {
    fn versets(&self) -> &[VersetQuotidien];
}
