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

/// Toute ancre vers une route du **serveur** porte `rel="external"`.
///
/// ## Le défaut qu'il garde ne se voit qu'au clic
///
/// Le routeur de Leptos intercepte tous les liens internes : il cherche le
/// chemin dans ses pages, et rend sa page d'erreur s'il ne l'y trouve pas. Or
/// `/fr/compte/aller/…` et `/fr/compte/partir` sont posées **avant** lui, dans
/// `main.rs` — ce sont des redirections qui écrivent des cookies, pas des pages.
///
/// Le symptôme est déroutant : **la page ne s'affiche qu'au rechargement**. Au
/// rechargement, le navigateur fait une vraie requête et le serveur répond ; au
/// clic, le routeur s'interpose. On cherche donc un défaut côté serveur, où il
/// n'y en a pas.
///
/// Il ne se voit ni à la compilation, ni dans le HTML servi, ni au simulateur —
/// seulement en cliquant. C'est Gloire qui l'a trouvé, et sa description tenait
/// le diagnostic entier : « la page google ne s'affiche que quand je reload ».
///
/// `leptos_router`, `location/mod.rs:346` : le routeur rend la main sur
/// `download` ou un `rel` contenant `external`.
#[cfg(all(test, feature = "ssr"))]
mod tests {
    /// Les chemins servis **avant** le routeur, qu'une ancre ne doit pas lui
    /// laisser intercepter.
    ///
    /// La liste est courte et tenue à la main : elle doit l'être, puisque
    /// `main.rs` les pose une à une. Le jour où une route s'y ajoute, c'est ici
    /// qu'elle entre — et le test dira aussitôt quelle ancre l'a oubliée.
    const HORS_ROUTEUR: [&str; 2] = ["/fr/compte/aller/", "/fr/compte/partir"];

    #[test]
    fn les_ancres_vers_le_serveur_laissent_passer_le_navigateur() {
        let pages = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/interface");
        let mut vues = 0;

        for entree in walk(&pages) {
            let source = std::fs::read_to_string(&entree).expect("un fichier");
            let nom = entree.file_name().unwrap().to_string_lossy().to_string();

            // Une ancre par bloc `<a`, et l'on regarde ce qu'elle porte jusqu'à
            // son `>` fermant.
            for morceau in source
                .split("<a\n")
                .skip(1)
                .chain(source.split("<a ").skip(1))
            {
                let balise = morceau.split('>').next().unwrap_or("");
                let Some(chemin) = HORS_ROUTEUR.iter().find(|c| balise.contains(*c)) else {
                    continue;
                };
                vues += 1;
                assert!(
                    balise.contains("rel=\"external\""),
                    "{nom} : une ancre mène à `{chemin}`, servie avant le routeur, \
                     sans `rel=\"external\"`. Le routeur l'interceptera et rendra \
                     sa page d'erreur — et le défaut ne se verra qu'au clic, \
                     jamais au rechargement."
                );
            }
        }

        assert!(
            vues >= 2,
            "seulement {vues} ancres relevées — le relevé est cassé"
        );
    }

    /// Tous les fichiers `.rs` sous un dossier, récursivement.
    fn walk(racine: &std::path::Path) -> Vec<std::path::PathBuf> {
        let mut trouves = Vec::new();
        let Ok(entrees) = std::fs::read_dir(racine) else {
            return trouves;
        };
        for entree in entrees.flatten() {
            let chemin = entree.path();
            if chemin.is_dir() {
                trouves.extend(walk(&chemin));
            } else if chemin.extension().is_some_and(|e| e == "rs") {
                trouves.push(chemin);
            }
        }
        trouves
    }
}
