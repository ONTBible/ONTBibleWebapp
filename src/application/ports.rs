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

/// Ce que le site sait demander au backend de l'app, à propos d'un compte.
///
/// ## Pourquoi un port, alors qu'il n'y a qu'une seule réalisation
///
/// Pour la même raison que `Vivier` : **l'éprouver sans réseau**. Un flux
/// d'authentification a des cas qu'on ne peut pas provoquer chez un vrai
/// fournisseur — un code déjà consommé, un jeton de rafraîchissement révoqué,
/// un backend qui répond `500`. Ce sont précisément ceux où le site doit se
/// comporter correctement, et les seuls qu'on ne verra jamais en développement.
///
/// ## Le site ne détient aucun secret
///
/// L'échange du code contre une session est fait par le backend, qui garde les
/// secrets clients. Ce port ne transporte donc jamais qu'un code d'autorisation
/// — inutilisable seul — et des jetons déjà délivrés.
#[cfg(feature = "ssr")]
#[async_trait::async_trait]
pub trait Comptes: Send + Sync {
    /// Échange un code d'autorisation contre une session.
    ///
    /// `redirect_uri` doit être **exactement** celle envoyée au fournisseur à
    /// l'aller : il la recompare, et le moindre écart — un `/` final, un
    /// paramètre en plus — donne un `invalid_grant` dont le message ne dit pas
    /// lequel des deux diffère.
    async fn ouvrir(
        &self,
        fournisseur: crate::domaine::compte::Fournisseur,
        code: &str,
        redirect_uri: &str,
        verificateur: Option<&str>,
    ) -> Result<crate::domaine::compte::Session, ErreurDeCompte>;

    /// Rend une session neuve à partir d'un jeton de rafraîchissement.
    ///
    /// **Le jeton ne sert qu'une fois.** Le backend le révoque en le
    /// consommant : deux rafraîchissements concurrents avec le même jeton
    /// donnent une session et une déconnexion, pas deux sessions.
    async fn renouveler(
        &self,
        jeton: &str,
    ) -> Result<crate::domaine::compte::Session, ErreurDeCompte>;
}

/// Ce qui peut mal se passer, du point de vue du site.
///
/// Trois cas et pas davantage, parce que le site n'en distingue que trois dans
/// ce qu'il montre : on redemande de se connecter, on réessaie plus tard, ou on
/// dit que le service ne répond pas. Un catalogue plus fin serait une précision
/// que rien ne consommerait.
#[cfg(feature = "ssr")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurDeCompte {
    /// Le code ou le jeton n'est pas — ou n'est plus — valide. On reconnecte.
    Refuse,
    /// Le backend a répondu autre chose qu'un succès. On réessaie.
    Indisponible,
    /// La réponse n'avait pas la forme attendue. C'est une rupture de contrat,
    /// et elle se distingue des deux autres : elle ne se résout ni en
    /// reconnectant ni en réessayant, elle se corrige dans le code.
    ContratRompu(String),
}

#[cfg(feature = "ssr")]
impl std::fmt::Display for ErreurDeCompte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Refuse => write!(f, "la connexion a été refusée"),
            Self::Indisponible => write!(f, "le service de comptes ne répond pas"),
            Self::ContratRompu(quoi) => write!(f, "réponse inattendue du backend : {quoi}"),
        }
    }
}

/// La synchronisation des surlignages, telle que le site en a besoin.
///
/// Deux gestes seulement — tirer et pousser —, parce que le backend n'en offre
/// pas d'autres et qu'un troisième serait une invention du site.
#[cfg(feature = "ssr")]
#[async_trait::async_trait]
pub trait Synchronisation: Send + Sync {
    /// Ce que le serveur a, depuis `depuis` s'il est donné.
    ///
    /// **`depuis` ne filtre pas la position** : le backend la rend à chaque
    /// appel, quel que soit le paramètre. C'est délibéré de sa part, et il faut
    /// le savoir — une position qu'on croirait inchangée parce qu'on n'a rien
    /// demandé serait en fait toujours la plus récente.
    ///
    /// Et la réponse **contient les pierres tombales**. Les écarter ici casserait
    /// la propagation des suppressions ; elles s'écartent à l'affichage.
    async fn tirer(&self, jeton: &str, depuis: Option<i64>) -> Result<Moisson, ErreurDeCompte>;

    /// Envoie ce qu'on a changé.
    ///
    /// Le backend rend `204` et un corps vide : il n'y a rien à relire, et
    /// attendre du JSON là où il n'y en a pas ferait échouer un envoi réussi.
    async fn pousser(
        &self,
        jeton: &str,
        surlignages: &[crate::domaine::surlignage::Surlignage],
        position: Option<&crate::domaine::surlignage::Position>,
    ) -> Result<(), ErreurDeCompte>;
}

/// Ce qu'un `tirer` rapporte.
#[cfg(feature = "ssr")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Moisson {
    #[serde(default)]
    pub highlights: Vec<crate::domaine::surlignage::Surlignage>,
    /// La position de lecture, s'il y en a une.
    ///
    /// **`depuis` ne la filtre pas** : le backend la rend à chaque appel, quel
    /// que soit le paramètre. Une position qu'on croirait inchangée parce qu'on
    /// n'a rien demandé est en fait toujours la plus récente.
    #[serde(default)]
    pub position: Option<crate::domaine::surlignage::Position>,
    /// L'horodatage du serveur — à renvoyer tel quel au prochain `depuis`.
    ///
    /// On renvoie **celui du serveur** et non notre propre horloge : deux
    /// machines ne sont jamais d'accord à la milliseconde, et un décalage de
    /// quelques secondes ferait manquer des changements sans que rien ne le
    /// signale.
    pub server_time: i64,
}
