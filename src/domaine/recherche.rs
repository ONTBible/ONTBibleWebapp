//! Le classement des résultats de recherche.
//!
//! ## Pourquoi il vit ici et non dans l'infrastructure
//!
//! Le pliage d'une chaîne — enlever les diacritiques, passer en minuscules —
//! est une affaire de bibliothèque : c'est `ont::search::fold`, et le site
//! l'emprunte au pipeline plutôt que de le réécrire. Le **classement**, lui,
//! est une décision éditoriale : un titre pèse plus qu'un corps, un mot entier
//! plus qu'un fragment, une glose moins que le texte. Ces nombres se relisent,
//! se discutent, et doivent s'éprouver sans index ni fichier.
//!
//! ## Les valeurs viennent de l'app, à l'unité près
//!
//! Relevées dans `ONTKit/Search/Search.swift`. Ce n'est pas un hommage : un
//! lecteur qui cherche « ruach » sur son téléphone et sur le site doit voir les
//! mêmes résultats **dans le même ordre**. Deux barèmes voisins donneraient deux
//! listes plausibles, et l'on ne saurait pas laquelle est la bonne.

use serde::{Deserialize, Serialize};

/// Où l'on cherche.
///
/// Les libellés sont ceux de l'app — `SearchScope` — parce qu'ils s'affichent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Portee {
    /// « Dans le texte » — le corps de la traduction seul.
    Corps,
    /// « Dans les gloses » — le niveau 2 seul.
    Gloses,
    /// « Partout ».
    #[default]
    Partout,
}

impl Portee {
    pub fn cle(self) -> &'static str {
        match self {
            Self::Corps => "corps",
            Self::Gloses => "gloses",
            Self::Partout => "partout",
        }
    }

    pub fn libelle(self) -> &'static str {
        match self {
            Self::Corps => "Dans le texte",
            Self::Gloses => "Dans les gloses",
            Self::Partout => "Partout",
        }
    }

    pub fn depuis_cle(cle: &str) -> Self {
        match cle {
            "corps" => Self::Corps,
            "gloses" => Self::Gloses,
            _ => Self::Partout,
        }
    }

    pub fn toutes() -> [Self; 3] {
        [Self::Partout, Self::Corps, Self::Gloses]
    }
}

/// Le niveau où la trouvaille a eu lieu — le texte, ou sa glose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Niveau {
    Corps,
    Glose,
}

/// La longueur minimale d'une requête.
///
/// Une lettre ramènerait la moitié du corpus, et le temps de l'afficher
/// dépasserait celui de taper la seconde.
pub const MINIMUM: usize = 2;

/// Le nombre de résultats rendus au plus.
pub const PLAFOND: usize = 300;

/// Ce qu'un enregistrement vaut pour une requête, ou rien s'il ne dit rien.
///
/// Les trois voies sont exclusives dans cet ordre, et c'est celui de l'app :
///
/// 1. **l'hébreu** — la requête est en hébreu, on ne regarde que `h` ;
/// 2. **le corps**, puis **la glose** — les deux peuvent répondre ;
/// 3. **le lemme**, et seulement si rien d'autre n'a répondu : un mot qui est
///    à la fois un lemme et un mot courant ne doit pas compter deux fois.
pub fn juger(
    corps_plie: &str,
    glose_pliee: &str,
    hebreu_nu: &str,
    lemmes: &[String],
    est_titre: bool,
    aiguille: &str,
    aiguille_hebraique: Option<&str>,
    aiguille_lemme: Option<&str>,
    portee: Portee,
) -> Vec<(Niveau, i32)> {
    // L'hébreu court-circuite tout : on cherche une forme, pas un sens, et la
    // faire concourir avec la traduction mêlerait deux questions.
    if let Some(hebraique) = aiguille_hebraique {
        return if hebreu_nu.contains(hebraique) {
            vec![(Niveau::Corps, 500)]
        } else {
            Vec::new()
        };
    }

    let mut trouves = Vec::new();

    if portee != Portee::Gloses {
        if let Some(ou) = corps_plie.find(aiguille) {
            let mut points = 300;
            // Un texte qui **commence** par ce qu'on cherche répond mieux qu'un
            // texte qui le contient au milieu.
            if corps_plie.starts_with(aiguille) {
                points += 60;
            }
            // Et un mot entier mieux qu'un fragment : chercher « or » ne doit
            // pas remonter tout ce qui contient « corps » avant les vrais.
            if mot_entier(corps_plie, ou, aiguille.len()) {
                points += 40;
            }
            if est_titre {
                points += 30;
            }
            trouves.push((Niveau::Corps, points));
        }
    }

    if portee != Portee::Corps && !glose_pliee.is_empty() && glose_pliee.contains(aiguille) {
        trouves.push((Niveau::Glose, 150));
    }

    if trouves.is_empty() {
        if let Some(lemme) = aiguille_lemme {
            if lemmes.iter().any(|l| l == lemme) {
                trouves.push((Niveau::Corps, 100));
            }
        }
    }

    trouves
}

/// Vrai quand la trouvaille est bordée de non-lettres des deux côtés.
///
/// On regarde des **octets** bornés par `find`, donc les indices tombent sur des
/// frontières de caractères ; les voisins se relisent en caractères pour que
/// « é » compte comme une lettre.
fn mot_entier(foin: &str, debut: usize, longueur: usize) -> bool {
    let avant = foin[..debut].chars().next_back();
    let apres = foin[debut + longueur..].chars().next();
    !avant.is_some_and(char::is_alphabetic) && !apres.is_some_and(char::is_alphabetic)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn juge(corps: &str, aiguille: &str) -> Vec<(Niveau, i32)> {
        juger(
            corps,
            "",
            "",
            &[],
            false,
            aiguille,
            None,
            None,
            Portee::Partout,
        )
    }

    #[test]
    fn un_mot_entier_pese_plus_qu_un_fragment() {
        let entier = juge("la lumiere", "lumiere")[0].1;
        let fragment = juge("les lumieres", "lumiere")[0].1;
        assert!(
            entier > fragment,
            "« lumiere » dans « la lumiere » doit devancer « lumieres »"
        );
    }

    #[test]
    fn un_debut_de_texte_pese_plus_qu_un_milieu() {
        assert!(juge("bara elohim", "bara")[0].1 > juge("elohim bara", "bara")[0].1);
    }

    /// Le barème de l'app, à l'unité — c'est la seule chose qui garantit deux
    /// listes identiques sur les deux plateformes.
    #[test]
    fn le_bareme_est_celui_de_l_app() {
        assert_eq!(juge("bara elohim", "bara"), vec![(Niveau::Corps, 400)]);
        assert_eq!(juge("elohim bara ciel", "bara"), vec![(Niveau::Corps, 340)]);
        assert_eq!(juge("elohim barasse", "bara"), vec![(Niveau::Corps, 300)]);
    }

    #[test]
    fn un_titre_devance_un_corps_egal() {
        let titre = juger(
            "bara",
            "",
            "",
            &[],
            true,
            "bara",
            None,
            None,
            Portee::Partout,
        )[0]
        .1;
        assert_eq!(titre, 400 + 30);
    }

    #[test]
    fn la_glose_repond_moins_fort_que_le_texte() {
        let deux = juger(
            "bara",
            "commencer",
            "",
            &[],
            false,
            "bara",
            None,
            None,
            Portee::Partout,
        );
        assert_eq!(deux.len(), 1, "la glose ne contient pas l'aiguille");
        let glose = juger(
            "",
            "bara",
            "",
            &[],
            false,
            "bara",
            None,
            None,
            Portee::Partout,
        );
        assert_eq!(glose, vec![(Niveau::Glose, 150)]);
    }

    /// Une portée qui exclut un niveau ne doit pas seulement le déclasser.
    #[test]
    fn une_portee_ecarte_vraiment_ce_qu_elle_exclut() {
        let corps_seul = juger(
            "bara",
            "bara",
            "",
            &[],
            false,
            "bara",
            None,
            None,
            Portee::Corps,
        );
        assert!(corps_seul.iter().all(|(n, _)| *n == Niveau::Corps));
        let gloses_seules = juger(
            "bara",
            "bara",
            "",
            &[],
            false,
            "bara",
            None,
            None,
            Portee::Gloses,
        );
        assert!(gloses_seules.iter().all(|(n, _)| *n == Niveau::Glose));
    }

    /// **Le lemme ne compte que si rien d'autre n'a répondu.**
    ///
    /// Sans cette exclusion, un mot qui est à la fois un lemme et un mot du
    /// texte remonterait deux fois pour la même trouvaille.
    #[test]
    fn le_lemme_ne_double_pas_une_trouvaille_du_texte() {
        let lemmes = vec!["bara".to_string()];
        let dans_le_texte = juger(
            "bara elohim",
            "",
            "",
            &lemmes,
            false,
            "bara",
            None,
            Some("bara"),
            Portee::Partout,
        );
        assert_eq!(dans_le_texte.len(), 1);

        let hors_du_texte = juger(
            "au commencement",
            "",
            "",
            &lemmes,
            false,
            "bara",
            None,
            Some("bara"),
            Portee::Partout,
        );
        assert_eq!(hors_du_texte, vec![(Niveau::Corps, 100)]);
    }

    /// L'hébreu ne concourt pas avec la traduction : il la remplace.
    #[test]
    fn une_requete_hebraique_ne_regarde_que_l_hebreu() {
        let trouve = juger(
            "bara",
            "",
            "ברא אלהים",
            &[],
            false,
            "bara",
            Some("ברא"),
            None,
            Portee::Partout,
        );
        assert_eq!(trouve, vec![(Niveau::Corps, 500)]);

        let absent = juger(
            "bara",
            "",
            "אלהים",
            &[],
            false,
            "bara",
            Some("ברא"),
            None,
            Portee::Partout,
        );
        assert!(absent.is_empty(), "le corps ne doit pas rattraper l'hébreu");
    }

    #[test]
    fn une_lettre_accentuee_est_une_lettre() {
        // « ete » dans « etre » n'est pas un mot entier ; le voisin est « r ».
        assert!(!mot_entier("etre", 0, 3));
        assert!(mot_entier("ete", 0, 3));
    }
}
