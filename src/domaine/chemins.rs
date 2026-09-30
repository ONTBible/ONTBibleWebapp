//! Les adresses du site, composées — jamais écrites à la main.
//!
//! ## Pourquoi un module pour ça
//!
//! Le site sert deux arbres, `/fr/liseuse/…` et `/fr/webapp/…`, et **un lien doit
//! rester dans l'arbre où l'on est** : cliquer un verset depuis l'édition ne doit
//! pas jeter le lecteur dans l'app. Avant ce module, douze fichiers écrivaient
//! `format!("/fr/webapp/{livre}/{unite}")` chacun de son côté ; il aurait fallu
//! les tenir d'accord à la main, et le premier oublié aurait fait changer d'arbre
//! en silence — une navigation qui marche, mais qui ne ramène pas où l'on était.
//!
//! ## Ce qui est ici et ce qui n'y est pas
//!
//! Les chemins **des deux arbres** : le corpus, le lexique, les onglets, le
//! compte, la recherche. Chacun prend l'[`Arbre`] en premier argument — le module
//! ne devine pas où l'on est, il compose ce qu'on lui demande.
//!
//! Les pages qui n'appartiennent à aucun arbre n'y sont pas : `/fr`, `/fr/l-app`,
//! `/fr/le-pourquoi`, `/fr/ce-que-l-ont-n-est-pas` et les légales sont des
//! adresses fixes, sans jumelle, et les écrire en clair est juste.
//!
//! ## Le domaine, et pas l'interface
//!
//! `domaine/vivier.rs` compose déjà le chemin du verset du jour, et `api.rs` celui
//! d'une unité : ce sont des données qui voyagent, pas des vues. Le domaine est
//! donc le bon étage — il n'a besoin ni de Leptos ni d'un contexte, et il
//! s'éprouve sans navigateur.

use crate::domaine::lecture::Arbre;

/// L'entrée du corpus — le sommaire des livres.
pub fn bible(arbre: Arbre) -> String {
    format!("{}/bible", arbre.racine())
}

/// Une partie du corpus — la Torah, les Nevi'im.
pub fn partie(arbre: Arbre, id: &str) -> String {
    format!("{}/bible/partie/{id}", arbre.racine())
}

/// Un livre, et la liste de ses unités.
pub fn livre(arbre: Arbre, id: &str) -> String {
    format!("{}/bible/{id}", arbre.racine())
}

/// Une unité — un chapitre, une parashah.
pub fn unite(arbre: Arbre, livre: &str, unite: &str) -> String {
    format!("{}/bible/{livre}/{unite}", arbre.racine())
}

/// Une unité, un verset désigné.
///
/// **Le paramètre reste `?v=`**, et il ne bouge pas : c'est la forme que portent
/// les liens déjà partagés, que l'app reconnaît, et que les métadonnées d'aperçu
/// lisent pour composer leur extrait.
pub fn unite_au_verset(arbre: Arbre, livre: &str, unite: &str, versets: &str) -> String {
    format!("{}?v={versets}", self::unite(arbre, livre, unite))
}

/// Le lexique des intraduisibles.
pub fn lexique(arbre: Arbre) -> String {
    format!("{}/lexique", arbre.racine())
}

/// La fiche d'un lemme.
pub fn fiche(arbre: Arbre, lemme: &str) -> String {
    format!("{}/lexique/{lemme}", arbre.racine())
}

/// La feuille de prononciation.
///
/// **Sous le lexique et non à côté**, comme chez l'app : elle explique comment
/// lire ce que les fiches écrivent, elle n'est pas une sixième destination.
pub fn prononciation(arbre: Arbre) -> String {
    format!("{}/lexique/prononciation", arbre.racine())
}

/// Le Qahal — le rassemblement des lecteurs.
pub fn qahal(arbre: Arbre) -> String {
    format!("{}/qahal", arbre.racine())
}

/// Les chuqqot.
pub fn chuqqot(arbre: Arbre) -> String {
    format!("{}/chuqqot", arbre.racine())
}

/// « Vous » — le compte, et ce qui s'y règle.
pub fn compte(arbre: Arbre) -> String {
    format!("{}/compte", arbre.racine())
}

/// Les réglages de lecture, sous le compte.
pub fn reglages(arbre: Arbre) -> String {
    format!("{}/compte/lecture", arbre.racine())
}

/// La recherche, vide.
pub fn rechercher(arbre: Arbre) -> String {
    format!("{}/rechercher", arbre.racine())
}

/// Le même chemin, dans l'autre arbre.
///
/// C'est ce qui rend la bascule possible sans table : on ne cherche pas la page
/// jumelle, on **remplace la racine**. Un chemin qui n'appartient à aucun arbre
/// — `/fr/l-app`, `/fr` — se rend inchangé : il n'a pas de jumelle, et lui en
/// inventer une mènerait à une adresse que rien ne sert.
pub fn dans(arbre: Arbre, chemin: &str) -> String {
    match Arbre::du_chemin(chemin) {
        Some(actuel) if actuel != arbre => {
            let suite = &chemin[actuel.racine().len()..];
            format!("{}{suite}", arbre.racine())
        }
        _ => chemin.to_string(),
    }
}

/// Le même chemin dans l'arbre **canonique**.
///
/// Une page servie sous `/fr/webapp/…` déclare son canonique sous
/// `/fr/liseuse/…`. Sans quoi les moteurs voient deux adresses pour le même
/// texte, choisissent eux-mêmes laquelle montrer, et la réputation des liens
/// entrants se coupe en deux — le facteur qui décide vraiment du référencement,
/// et le seul qu'aucune ligne de Rust ne fabrique.
pub fn canonique(chemin: &str) -> String {
    dans(Arbre::CANONIQUE, chemin)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Chaque composeur rend un chemin sous la racine qu'on lui donne.
    ///
    /// L'épreuve ne compare pas à des littéraux : elle vérifie la **propriété**,
    /// de sorte qu'un renommage de segment la laisse verte. Un test qui répète
    /// la table qu'il contrôle ne contrôle que lui-même.
    #[test]
    fn tout_chemin_tombe_sous_la_racine_de_son_arbre() {
        for arbre in Arbre::TOUS {
            let racine = arbre.racine();
            for chemin in [
                bible(arbre),
                partie(arbre, "torah"),
                livre(arbre, "bereshit"),
                unite(arbre, "bereshit", "bereshit-1"),
                unite_au_verset(arbre, "bereshit", "bereshit-1", "1-3"),
                lexique(arbre),
                fiche(arbre, "bara"),
                prononciation(arbre),
                qahal(arbre),
                chuqqot(arbre),
                compte(arbre),
                reglages(arbre),
                rechercher(arbre),
            ] {
                assert!(
                    chemin.starts_with(&format!("{racine}/")),
                    "« {chemin} » ne tombe pas sous « {racine} »"
                );
                assert_eq!(
                    Arbre::du_chemin(&chemin),
                    Some(arbre),
                    "« {chemin} » ne se relit pas comme {arbre:?}"
                );
            }
        }
    }

    /// Le corpus passe par `bible`, et pas directement sous la racine.
    ///
    /// C'est la demande du 30 septembre — `/fr/webapp/bible/{livre}` et non
    /// `/fr/webapp/{livre}` — et elle a une conséquence mécanique : sans ce
    /// segment, un livre nommé « compte » ou « lexique » entrerait en collision
    /// avec un onglet. ==Un paramètre à la racine d'un arbre rend tout segment
    /// statique ambigu.==
    #[test]
    fn le_corpus_vit_sous_bible() {
        let a = Arbre::Liseuse;
        assert_eq!(livre(a, "bereshit"), "/fr/liseuse/bible/bereshit");
        assert_eq!(
            unite(a, "bereshit", "bereshit-1"),
            "/fr/liseuse/bible/bereshit/bereshit-1"
        );
        assert_eq!(partie(a, "torah"), "/fr/liseuse/bible/partie/torah");
    }

    /// `dans` remplace la racine, et laisse tranquille ce qui n'en a pas.
    #[test]
    fn basculer_d_arbre_remplace_la_racine() {
        let sous_webapp = unite(Arbre::Webapp, "bereshit", "bereshit-3");
        assert_eq!(
            dans(Arbre::Liseuse, &sous_webapp),
            unite(Arbre::Liseuse, "bereshit", "bereshit-3")
        );
        // Aller-retour : la bascule est involutive.
        assert_eq!(
            dans(Arbre::Webapp, &dans(Arbre::Liseuse, &sous_webapp)),
            sous_webapp
        );
        // Ce qui n'appartient à aucun arbre ne bouge pas.
        for hors in ["/fr", "/fr/l-app", "/fr/le-pourquoi", "/fr/conditions"] {
            assert_eq!(dans(Arbre::Webapp, hors), hors);
            assert_eq!(dans(Arbre::Liseuse, hors), hors);
        }
    }

    /// Le canonique ramène toujours au même arbre, d'où qu'on parte.
    #[test]
    fn le_canonique_est_un_point_fixe() {
        let webapp = unite(Arbre::Webapp, "bereshit", "bereshit-1");
        let liseuse = unite(Arbre::Liseuse, "bereshit", "bereshit-1");
        assert_eq!(canonique(&webapp), canonique(&liseuse));
        assert_eq!(canonique(&canonique(&webapp)), canonique(&webapp));
        assert_eq!(
            Arbre::du_chemin(&canonique(&webapp)),
            Some(Arbre::CANONIQUE)
        );
    }

    /// La chaîne de requête survit à la bascule.
    ///
    /// Perdre `?v=1-3` rendrait le passage entier là où le lien désignait trois
    /// lignes — le défaut que la redirection de `/fr/lire` avait déjà eu à tenir.
    #[test]
    fn le_verset_designe_survit_a_la_bascule() {
        let depart = unite_au_verset(Arbre::Webapp, "bereshit", "bereshit-1", "1-3");
        assert!(dans(Arbre::Liseuse, &depart).ends_with("?v=1-3"));
    }
}
