//! Les ports — ce dont l'application a besoin, et rien de plus.
//!
//! Chaque trait est taillé au besoin d'un seul cas d'usage. C'est le `I` de
//! SOLID : un port large obligerait ses implémentations à porter des méthodes
//! dont personne ne se sert, et un faux de test à les inventer.

use std::sync::Arc;

use crate::domaine::corpus::{Ensemble, Entree, Livre, Occurrence};
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

/// Accéder au corpus — la liseuse.
///
/// Les livres sortent en `Arc` et non par référence : une réalisation qui les
/// analyse **à la demande** doit pouvoir écrire dans son cache, ce qu'une
/// méthode rendant `&Livre` lui interdirait sans verrou. Et le prix d'un `Arc`
/// est un incrément de compteur, contre plusieurs milliers de nœuds recopiés.
pub trait Corpus: Send + Sync {
    /// Le plan du corpus entier — les 70 livres, écrits ou non.
    ///
    /// Le sommaire cite tout : l'ampleur du chantier fait partie de ce que le
    /// site dit. Il ne porte pas le texte, seulement de quoi dresser la liste.
    fn sommaire(&self) -> &[Ensemble];

    /// Un livre avec tout son texte. `None` s'il n'est pas écrit.
    fn livre(&self, id: &str) -> Option<Arc<Livre>>;
}

/// Accéder au lexique.
///
/// Un port distinct de [`Corpus`], et pas par purisme : une page de fiche n'a
/// aucun besoin du corpus, et une page de lecture aucun besoin du lexique.
/// Les fondre obligerait chaque faux de test à porter les deux.
pub trait Lexique: Send + Sync {
    /// Toutes les fiches, dans l'ordre alphabétique du lemme.
    fn entrees(&self) -> &[Entree];

    /// Une fiche par son lemme.
    fn entree(&self, lemme: &str) -> Option<&Entree>;

    /// Où le terme paraît dans le corpus.
    ///
    /// Analysé à la demande : c'est le plus gros fichier du pipeline, et la
    /// plupart des visites ne consultent aucune fiche.
    fn occurrences(&self, lemme: &str) -> Vec<Occurrence>;
}
