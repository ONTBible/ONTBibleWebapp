//! Ce que la page du compte a besoin de savoir des deux côtés.
//!
//! ## Pourquoi ce module existe
//!
//! `interface::compte` est `ssr` : il manipule des cookies, des redirections et
//! un client HTTP, qui n'ont rien à faire dans un binaire wasm. Mais la **page**
//! du compte se compile des deux côtés, et elle doit savoir quels fournisseurs
//! afficher.
//!
//! On sépare donc le **fait** — ce fournisseur est-il déclaré — de la
//! **mécanique** qui l'utilise. Le premier voyage, la seconde reste au serveur.
//!
//! Le jour où un identifiant client entre ici, il faudra se souvenir qu'**il
//! n'est pas un secret** : il voyage en clair dans l'adresse d'autorisation, et
//! c'est le secret client — qui ne quitte pas la Lambda du backend — qui protège
//! l'échange.

use crate::domaine::compte::Fournisseur;

/// Vrai quand ce fournisseur est déclaré et utilisable.
///
/// La liste est tenue **ici** et lue par `interface::compte`, jamais l'inverse :
/// deux listes finiraient par diverger, et l'écart se verrait par un bouton qui
/// mène à une erreur du fournisseur — là où le lecteur ne peut rien faire.
pub fn disponible(fournisseur: Fournisseur) -> bool {
    match fournisseur {
        // Le client « Application Web » du backend accepte plusieurs adresses de
        // retour : il suffit d'y ajouter celle du site.
        Fournisseur::Google => true,
        // Demande un **Services ID** à créer dans le portail Apple, distinct de
        // l'App ID que l'app utilise. Le README du backend l'avait prévu : « il
        // ne redeviendra nécessaire que le jour où une version web signera des
        // comptes ».
        Fournisseur::Apple => false,
        // GitHub n'accepte **qu'une seule** adresse de retour par application, et
        // celle-ci est prise par l'app. Il en faut une seconde pour le site.
        Fournisseur::Github => false,
    }
}
