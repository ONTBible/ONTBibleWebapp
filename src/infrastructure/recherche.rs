//! L'index de recherche, et la requête qu'on lui pose.
//!
//! ## L'index est embarqué, comme le corpus
//!
//! `include_str!` pointe `dist/search.json` — 698 Ko, 1255 enregistrements
//! aujourd'hui. Rien n'est dupliqué : le fichier reste chez le pipeline, et un
//! binaire sans dossier de données à côté ne peut pas tomber parce qu'un
//! déploiement l'aurait oublié. C'est la règle du §8 bis.
//!
//! Il est analysé **à la première recherche** et non au démarrage, par un
//! `OnceLock` : la plupart des visites ne cherchent rien, et le démarrage à
//! froid de la Lambda se paie sur elles.
//!
//! ## Le pliage vient du pipeline, jamais d'ici
//!
//! `ont::search::fold` et `strip_hebrew` sont ceux qui ont produit l'index. Les
//! réécrire ferait deux normalisations à tenir d'accord — et le jour où elles
//! divergeraient d'un accent, la requête ne trouverait plus ce que l'index
//! contient, sans que rien ne le signale.

use std::collections::HashSet;
use std::sync::OnceLock;

use crate::domaine::recherche::{juger, Niveau, Portee, MINIMUM, PLAFOND};

const INDEX: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../ONTBibleApp/dist/search.json"
));

#[derive(serde::Deserialize)]
struct Fichier {
    records: Vec<ont::schema::SearchRecord>,
}

fn enregistrements() -> &'static [ont::schema::SearchRecord] {
    static LU: OnceLock<Vec<ont::schema::SearchRecord>> = OnceLock::new();
    LU.get_or_init(|| {
        serde_json::from_str::<Fichier>(INDEX)
            .map(|f| f.records)
            .unwrap_or_default()
    })
}

/// Les lemmes que l'index connaît, pour savoir si une requête en est un.
///
/// L'app passe ceux du **lexique** ; on prend ici l'union des `l` de l'index.
/// Le résultat est le même : un lemme du lexique qu'aucun enregistrement ne
/// porte ne pourrait de toute façon rien apparier, la règle du barème étant
/// « ce lemme est-il dans *ce* record ». La différence est qu'on n'a pas
/// besoin du lexique — donc pas d'un second chargement, ni d'un contexte que
/// cette fonction n'a aucune raison de connaître.
fn lemmes() -> &'static HashSet<String> {
    static LU: OnceLock<HashSet<String>> = OnceLock::new();
    LU.get_or_init(|| {
        enregistrements()
            .iter()
            .flat_map(|r| r.l.iter().cloned())
            .collect()
    })
}

/// Une trouvaille, telle que la page l'affiche.
pub struct Trouvaille {
    pub livre_id: String,
    pub unite_id: String,
    pub verset: u32,
    pub extrait: String,
    pub niveau: Niveau,
    pub points: i32,
}

/// Cherche, et rend les meilleurs d'abord.
///
/// ## Le départage n'est pas décoratif
///
/// À points égaux on classe par identifiant d'unité **croissant**, comme l'app.
/// Sans lui, l'ordre serait celui du parcours de l'index — stable, mais
/// arbitraire, et différent du sien. Deux listes plausibles dont on ne saurait
/// pas laquelle est la bonne.
pub fn chercher(requete: &str, portee: Portee) -> Vec<Trouvaille> {
    let brut = requete.trim();
    if brut.chars().count() < MINIMUM {
        return Vec::new();
    }

    let hebraique = est_hebreu(brut).then(|| ont::search::strip_hebrew(brut));
    let aiguille = ont::search::fold(brut);
    let aiguille_lemme = lemmes().contains(&aiguille).then(|| aiguille.clone());

    let mut trouvailles: Vec<Trouvaille> = Vec::new();
    for r in enregistrements() {
        for (niveau, points) in juger(
            &r.t,
            &r.g,
            &r.h,
            &r.l,
            matches!(r.k, ont::schema::RecordKind::Heading),
            &aiguille,
            hebraique.as_deref(),
            aiguille_lemme.as_deref(),
            portee,
        ) {
            trouvailles.push(Trouvaille {
                livre_id: r.b.clone(),
                unite_id: r.c.clone(),
                verset: r.v,
                extrait: r.x.clone(),
                niveau,
                points,
            });
        }
    }

    trouvailles.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| a.unite_id.cmp(&b.unite_id))
    });
    trouvailles.truncate(PLAFOND);
    trouvailles
}

/// Vrai si la chaîne porte au moins un caractère du bloc hébreu.
///
/// Un seul suffit : on ne tape pas une lettre hébraïque par accident, et un
/// mot mêlé — « le ברא » — se cherche mieux par l'hébreu que par le français.
fn est_hebreu(chaine: &str) -> bool {
    chaine
        .chars()
        .any(|c| ('\u{0590}'..='\u{05FF}').contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l_index_s_analyse_et_n_est_pas_vide() {
        let tous = enregistrements();
        assert!(
            tous.len() > 500,
            "l'index n'en porte que {} — il ne s'est pas analysé",
            tous.len()
        );
    }

    /// Une requête trop courte ne rend rien, et ne coûte pas le parcours.
    #[test]
    fn une_lettre_ne_cherche_pas() {
        assert!(chercher("a", Portee::Partout).is_empty());
        assert!(chercher(" ", Portee::Partout).is_empty());
    }

    /// **Le pliage doit être celui de l'index.**
    ///
    /// Si `fold` divergeait, une requête accentuée ne trouverait plus rien alors
    /// que le mot est là. On le vérifie sur un mot du corpus écrit des deux
    /// façons : c'est le seul cas dont on connaisse la réponse d'avance.
    #[test]
    fn un_accent_ne_change_pas_le_resultat() {
        let sans = chercher("etat", Portee::Partout).len();
        let avec = chercher("état", Portee::Partout).len();
        assert_eq!(sans, avec, "« etat » et « état » doivent trouver pareil");
        assert!(sans > 0, "le corpus porte « l'état sans ordre »");
    }

    #[test]
    fn les_meilleurs_viennent_d_abord() {
        let trouves = chercher("terre", Portee::Partout);
        assert!(trouves.len() > 1);
        for paire in trouves.windows(2) {
            assert!(
                paire[0].points >= paire[1].points,
                "le classement doit décroître"
            );
        }
    }

    /// La portée doit **retirer** et non déclasser.
    #[test]
    fn chercher_dans_les_gloses_ne_rend_que_des_gloses() {
        let gloses = chercher("commenc", Portee::Gloses);
        assert!(
            gloses.iter().all(|t| t.niveau == Niveau::Glose),
            "une trouvaille de corps a survécu à la portée « gloses »"
        );
    }

    #[test]
    fn le_plafond_est_tenu() {
        // « e » est trop court ; « es » traverse presque tout le corpus.
        assert!(chercher("es", Portee::Partout).len() <= PLAFOND);
    }
}
