//! La sélection de versets d'un lien partagé — le `?v=` des adresses.
//!
//! Un lien partagé depuis l'app ressemble à ceci :
//!
//! ```text
//! /fr/lire/bereshit/bereshit-1?v=1-3
//! ```
//!
//! Le paramètre dit **ce que la personne a partagé** : elle n'a pas envoyé un
//! chapitre, elle a envoyé trois versets, et la page doit les montrer comme
//! tels. Sans lui, quelqu'un qui reçoit le lien arrive en haut d'un chapitre de
//! trente-quatre versets et doit deviner lequel on lui montrait.
//!
//! C'est du calcul, pas de l'affichage : il vit donc dans le domaine, et il est
//! éprouvé sans navigateur.

/// Les numéros de versets désignés par un paramètre `v`.
///
/// Trois formes, et elles se combinent — c'est ce que produit une sélection
/// dans l'app, où l'on peut cocher des versets qui ne se suivent pas :
///
/// | paramètre | désigne |
/// |---|---|
/// | `6` | le verset 6 |
/// | `1-3` | les versets 1, 2 et 3 |
/// | `1,4-6` | les versets 1, 4, 5 et 6 |
///
/// ## Ce qui est refusé, et pourquoi en silence
///
/// Un paramètre illisible rend une sélection **vide**, jamais une erreur. Ce
/// paramètre vient d'une adresse, donc de l'extérieur : il est recopié à la
/// main, tronqué par une messagerie, bricolé. Le chapitre, lui, existe. Répondre
/// par une page d'erreur parce qu'un fragment d'adresse est abîmé priverait le
/// lecteur d'un texte parfaitement disponible.
///
/// Les bornes inversées — `9-3` — sont lues à l'endroit : c'est une faute de
/// frappe évidente, et l'intention ne fait aucun doute.
///
/// L'étendue est **bornée**. `1-999999999` demanderait au serveur de construire
/// un milliard d'entiers pour un chapitre qui en compte trente-quatre : c'est
/// une manière de le mettre à genoux avec une seule adresse.
pub fn versets(parametre: &str) -> Vec<u32> {
    /// Aucun chapitre du corpus n'approche ce nombre — le plus long en compte
    /// 46. La borne n'existe que pour qu'une adresse ne puisse pas demander une
    /// allocation démesurée.
    const MAXIMUM: u32 = 1_000;

    let mut numeros: Vec<u32> = parametre
        .split(',')
        .filter_map(|fragment| {
            let fragment = fragment.trim();
            match fragment.split_once('-') {
                Some((debut, fin)) => {
                    let debut: u32 = debut.trim().parse().ok()?;
                    let fin: u32 = fin.trim().parse().ok()?;
                    let (debut, fin) = if debut <= fin {
                        (debut, fin)
                    } else {
                        (fin, debut)
                    };
                    Some((debut..=fin.min(debut.saturating_add(MAXIMUM))).collect::<Vec<_>>())
                }
                None => fragment.parse().ok().map(|n| vec![n]),
            }
        })
        .flatten()
        .filter(|n| *n > 0)
        .collect();

    // Un même verset peut être désigné deux fois — « 1,1-3 ». La page en fait
    // un ensemble, pas une liste : le trier et le dédoublonner ici évite que
    // chaque appelant y pense.
    numeros.sort_unstable();
    numeros.dedup();
    numeros
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_verset_seul() {
        assert_eq!(versets("6"), [6]);
    }

    #[test]
    fn une_etendue() {
        assert_eq!(versets("1-3"), [1, 2, 3]);
    }

    #[test]
    fn des_fragments_qui_se_combinent() {
        assert_eq!(versets("1,4-6"), [1, 4, 5, 6]);
    }

    #[test]
    fn les_doublons_disparaissent_et_l_ordre_est_retabli() {
        assert_eq!(versets("3,1-2,1"), [1, 2, 3]);
    }

    #[test]
    fn des_bornes_inversees_se_lisent_a_l_endroit() {
        assert_eq!(versets("5-3"), [3, 4, 5]);
    }

    /// Le point qui compte : un paramètre abîmé n'empêche pas de lire.
    #[test]
    fn un_parametre_illisible_ne_designe_rien() {
        for cassé in ["", "abc", "-", "1-", "-3", "0", "1;3"] {
            assert!(
                versets(cassé).is_empty(),
                "« {cassé} » devrait ne rien désigner"
            );
        }
    }

    /// Un fragment valide survit à un fragment cassé — une messagerie qui coupe
    /// la fin d'une adresse ne doit pas emporter le début.
    #[test]
    fn un_fragment_casse_n_emporte_pas_les_autres() {
        assert_eq!(versets("2,abc,4"), [2, 4]);
    }

    /// Une étendue démesurée est coupée, pas honorée.
    #[test]
    fn une_etendue_demesuree_est_bornee() {
        let choisis = versets("1-999999999");
        assert_eq!(choisis.len(), 1_001);
        assert_eq!(choisis[0], 1);
    }
}
