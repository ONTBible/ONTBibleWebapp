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

/// Le paramètre `v` qui désigne exactement ces versets.
///
/// C'est l'inverse de [`versets`], et les deux doivent le rester : ce qu'on
/// écrit ici est relu là-bas — par ce site, par l'app iOS et par l'app Android,
/// qui ont chacune leur analyseur. La forme est donc un **contrat entre trois
/// dépôts**, et non un détail d'affichage.
///
/// ## Les suites se replient
///
/// `1,2,3,7` s'écrit `1-3,7`. Ce n'est pas de la cosmétique : une sélection
/// d'un chapitre entier ferait sinon une adresse de plusieurs centaines de
/// signes, que les messageries tronquent — et un lien tronqué ne désigne plus
/// le bon passage, il désigne un passage **plausible**. C'est le pire des deux :
/// il s'ouvre, il montre du texte, et ce n'est pas celui qu'on a partagé.
///
/// **Une paire s'écrit `4-5`**, comme le reste — et c'est l'app qui décide, pas
/// nous. `VerseRange.label` fait `start == end ? "\(start)" : "\(start)-\(end)"`,
/// donc deux versets consécutifs forment un intervalle au même titre que dix.
///
/// On avait d'abord écrit `4,5`, au motif que « un intervalle de deux se lit
/// moins bien qu'une paire ». C'était une invention, et elle aurait produit deux
/// adresses différentes pour la même sélection selon qu'on partage depuis le
/// site ou depuis le téléphone. Le format est un contrat : la règle vient de
/// celui qui l'a écrite en premier.
///
/// ## L'ordre et les doublons
///
/// La sortie est **triée et dédoublonnée**, quel que soit l'ordre d'entrée. Une
/// sélection se fait dans l'ordre où l'on touche les versets, pas dans celui du
/// chapitre : sans ce tri, deux personnes qui désignent les mêmes versets
/// produiraient deux adresses différentes, donc deux entrées de cache et deux
/// aperçus.
///
/// Une sélection vide rend une chaîne vide, à l'appelant de ne pas poser de `?v=`.
pub fn parametre(numeros: &[u32]) -> String {
    let mut tries: Vec<u32> = numeros.to_vec();
    tries.sort_unstable();
    tries.dedup();

    let mut morceaux: Vec<String> = Vec::new();
    let mut i = 0;
    while i < tries.len() {
        let debut = tries[i];
        let mut fin = debut;
        while i + 1 < tries.len() && tries[i + 1] == fin + 1 {
            i += 1;
            fin = tries[i];
        }
        // `borne` de l'app, à l'identique : deux bornes égales donnent un
        // nombre, sinon un intervalle. Aucun cas particulier pour la paire.
        morceaux.push(if fin == debut {
            debut.to_string()
        } else {
            format!("{debut}-{fin}")
        });
        i += 1;
    }
    morceaux.join(",")
}

/// Le même ensemble, écrit **pour être lu** — « 1, 4-6 ».
///
/// ## Pourquoi ce n'est pas [`parametre`]
///
/// Une virgule française prend une espace après elle ; une adresse n'en veut
/// pas. `?v=1, 4-6` obligerait à encoder l'espace en `%20`, ce qui donne un lien
/// que les messageries coupent au mauvais endroit et qu'un lecteur ne reconnaît
/// plus comme le sien.
///
/// Les deux grammaires vivent donc côte à côte, et **ce fichier est le seul
/// endroit où leur différence est décidée**. Écrire le libellé ailleurs, à
/// partir de `parametre`, ferait apparaître « 1,4-6 » sous un texte français —
/// ou pire, deux écritures différentes du même ensemble selon la page.
///
/// C'est aussi la grammaire qu'affiche l'app iOS — `VerseRange.label` rend
/// « 1, 4-6 » pour le même ensemble. Le lecteur qui partage depuis son
/// téléphone et retrouve le lien sur le site doit lire la même chose des deux
/// côtés ; sinon il croit avoir partagé autre chose.
///
/// Le **groupement est identique** à celui de l'adresse, puisqu'il en vient :
/// une paire reste énumérée, une suite de trois se replie. Deux repliements
/// distincts feraient dire au libellé autre chose qu'au lien qu'il accompagne.
pub fn libelle(numeros: &[u32]) -> String {
    parametre(numeros).replace(',', ", ")
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

    /// Ce que `parametre` écrit, `versets` le relit à l'identique.
    ///
    /// C'est **la** propriété qui compte, et elle vaut plus que n'importe quel
    /// cas particulier : les deux fonctions sont l'une l'inverse de l'autre, et
    /// le jour où l'une évolue sans l'autre, ce test le dit avant que trois
    /// dépôts ne se retrouvent en désaccord sur ce qu'une adresse désigne.
    ///
    /// Les cas couvrent ce qu'une sélection réelle produit : un verset seul,
    /// une suite, une paire, des îlots séparés, un chapitre entier, et un ordre
    /// de saisie quelconque avec doublons.
    #[test]
    fn ce_qui_est_ecrit_se_relit_a_l_identique() {
        let cas: Vec<Vec<u32>> = vec![
            vec![6],
            vec![1, 2, 3],
            vec![4, 5],
            vec![1, 4, 5, 6],
            vec![1, 3, 5, 7, 9],
            (1..=31).collect(),
            vec![7, 2, 2, 9, 3, 7],
        ];
        for entree in cas {
            let ecrit = parametre(&entree);
            let relu = versets(&ecrit);
            let mut attendu = entree.clone();
            attendu.sort_unstable();
            attendu.dedup();
            assert_eq!(
                relu, attendu,
                "« {ecrit} » ne se relit pas comme {attendu:?} mais comme {relu:?}"
            );
        }
    }

    /// Une suite longue se replie, une paire non.
    ///
    /// Le repliement n'est pas cosmétique : une sélection de chapitre entier
    /// ferait sinon une adresse de plusieurs centaines de signes, que les
    /// messageries tronquent. Et un lien tronqué ne désigne plus le bon
    /// passage — il en désigne un **plausible**, ce qui est pire, parce qu'il
    /// s'ouvre et montre du texte.
    #[test]
    fn les_suites_se_replient_sans_mentir() {
        assert_eq!(parametre(&[1, 2, 3]), "1-3");
        assert_eq!(
            parametre(&[4, 5]),
            "4-5",
            "une paire s'écrit comme le reste — c'est `borne` de l'app"
        );
        assert_eq!(parametre(&[1, 4, 5, 6]), "1,4-6");
        assert_eq!(parametre(&[6]), "6");
        assert_eq!(parametre(&[]), "");

        let entier: Vec<u32> = (1..=31).collect();
        assert_eq!(parametre(&entier), "1-31");
        assert!(
            parametre(&entier).len() < 8,
            "un chapitre entier doit tenir en quelques signes, pas en centaines"
        );
    }

    /// L'ordre de saisie ne change pas l'adresse produite.
    ///
    /// On sélectionne dans l'ordre où l'on touche les versets, pas dans celui du
    /// chapitre. Sans tri, deux personnes qui désignent le même passage
    /// produiraient deux adresses différentes — donc deux entrées de cache, deux
    /// aperçus, et deux liens qu'on croirait distincts.
    #[test]
    fn l_ordre_de_saisie_ne_change_rien() {
        assert_eq!(parametre(&[3, 1, 2]), parametre(&[1, 2, 3]));
        assert_eq!(parametre(&[6, 4, 5, 1]), "1,4-6");
        assert_eq!(parametre(&[2, 2, 2]), "2");
    }

    /// Le libellé et l'adresse disent le même ensemble, dans deux grammaires.
    ///
    /// La propriété qui les lie : ôter les espaces du libellé rend exactement
    /// l'adresse. C'est ce qui garantit qu'ils ne peuvent pas grouper
    /// différemment — un lecteur qui lit « 1, 4-6 » sous un lien `?v=1,4-6`
    /// voit deux écritures du même ensemble, pas deux ensembles.
    #[test]
    fn le_libelle_et_l_adresse_disent_le_meme_ensemble() {
        let cas: Vec<Vec<u32>> = vec![
            vec![6],
            vec![4, 5],
            vec![1, 2, 3],
            vec![1, 4, 5, 6],
            vec![1, 3, 5, 7],
            (1..=31).collect(),
        ];
        for entree in cas {
            let adresse = parametre(&entree);
            let lisible = libelle(&entree);
            assert_eq!(
                lisible.replace(' ', ""),
                adresse,
                "« {lisible} » et « {adresse} » ne groupent pas pareil"
            );
            assert_eq!(versets(&adresse), versets(&lisible.replace(' ', "")));
        }
    }

    /// La virgule française prend son espace, l'adresse non.
    ///
    /// `?v=1, 4-6` obligerait à encoder l'espace en `%20` : un lien que les
    /// messageries coupent au mauvais endroit et qu'un lecteur ne reconnaît plus
    /// comme le sien.
    #[test]
    fn la_virgule_prend_son_espace_dans_le_libelle_seulement() {
        assert_eq!(libelle(&[1, 4, 5, 6]), "1, 4-6");
        assert_eq!(parametre(&[1, 4, 5, 6]), "1,4-6");
        assert_eq!(libelle(&[4, 5]), "4-5", "une paire n'a pas de virgule");
        assert_eq!(libelle(&[6]), "6", "un verset seul n'a pas de virgule");
        assert!(
            !parametre(&[1, 4, 5, 6]).contains(' '),
            "une adresse ne porte jamais d'espace"
        );
    }

    /// Imprime ce que le site produit, pour l'éprouver hors du site.
    ///
    /// Sert à donner à une session voisine un lien **réellement produit** plutôt
    /// qu'un lien écrit à la main — c'est la seule façon d'éprouver la chaîne
    /// bout en bout, chacun n'ayant jusque-là vérifié que sa moitié.
    #[test]
    fn imprimer_un_lien_reel() {
        for cas in [
            vec![1u32, 2, 3, 7],
            vec![6],
            vec![4, 5],
            vec![7, 2, 1, 3, 7],
        ] {
            println!(
                "  {:?} → https://ontbible.com/fr/lire/bereshit/bereshit-1?v={}  ({})",
                cas,
                parametre(&cas),
                libelle(&cas)
            );
        }
    }
}
