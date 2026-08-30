//! Les réglages de lecture — éteindre les niveaux du texte.
//!
//! ## Ce n'est pas une préférence d'affichage
//!
//! Le portage reprend le mot de l'app, dans `ONTKit/Reader/Reader.swift` :
//!
//! > Les deux premiers champs ne sont pas des préférences d'affichage : ce sont
//! > les **niveaux du texte**, et pouvoir les éteindre est la raison d'être de
//! > la liseuse.
//!
//! Le corps de la traduction ne s'éteint jamais. Ce qui s'éteint, c'est
//! l'appareil critique : la glose qui explicite l'implicite hébreu, et le mot
//! original avec sa translittération.
//!
//! ## Ce qui n'est **pas** réglable, et pourquoi
//!
//! Ni les intraduisibles, ni les accentuations. L'app n'en offre pas la
//! bascule non plus, et son moteur de rendu dit pourquoi :
//!
//! > Une accentuation survit à l'extinction des niveaux : elle appartient au
//! > corps, pas à l'appareil critique.
//!
//! Un intraduisible n'est pas un commentaire ajouté au texte — c'est le texte,
//! qu'on a refusé de traduire. L'éteindre ne rendrait pas la lecture plus
//! simple : il laisserait un trou. Et la promesse de l'or — une fiche, et le
//! lien la tient — cesserait d'être tenue selon un réglage.
//!
//! ## Le nettoyage, qui est le vrai travail
//!
//! Retirer un nœud laisse ses blancs derrière lui. Une glose se pose **après**
//! le mot qu'elle éclaire et **avant** la ponctuation qui suit : l'ôter donne
//! « habitant , et la face ». Une translittération est encadrée d'espaces :
//! l'ôter en laisse deux. C'est le même défaut que celui déjà corrigé sur les
//! aperçus de messagerie, et il se corrige au même endroit.

use serde::{Deserialize, Serialize};

use crate::domaine::texte::Noeud;

/// Ce que le lecteur a choisi de voir.
/// `serde(default)` sur chaque champ, et ce n'est pas une précaution de style :
/// ces valeurs viennent du **stockage du navigateur**, écrit par une version
/// antérieure du site. Le jour où un quatrième réglage apparaît, les réglages
/// déjà retenus n'en portent pas la clé — sans cette tolérance, ils seraient
/// tous jetés d'un coup, et chaque lecteur retrouverait les défauts. L'app fait
/// la même chose, champ par champ, dans son `init(from:)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    /// Niveau 2 — ce que le champ sémantique hébreu portait implicitement.
    pub gloses: bool,
    /// Niveau 3 — la translittération et l'hébreu.
    pub niveau_3: bool,
    /// Les versets coulent en prose, numéros en exposant.
    pub continu: bool,
    /// Nommer les livres et les sections dans le français reçu.
    ///
    /// **Vrai par défaut**, et c'est délibéré : un lecteur qui arrive doit
    /// pouvoir se repérer avec les mots qu'il connaît. « Apocalypse », « la
    /// Loi », « Actes des Apôtres ».
    ///
    /// À faux, il lit ce que le nom ONT veut dire — « le *machazeh* de
    /// Yohanan », « la Fondation », « les *gevurot* de YHWH par ses *neviim* ».
    /// Les intraduisibles y restent en hébreu, là où le français les rend.
    ///
    /// **L'écart entre les deux est le projet lui-même** : *torah*,
    /// l'instruction qui vise, est devenue *nomos*, le code qui contraint. Le
    /// réglage laisse le lecteur passer d'un monde à l'autre au lieu de le lui
    /// raconter.
    pub francais: bool,
}

impl Default for Preferences {
    /// Tout est montré, et les versets se tiennent séparés.
    ///
    /// Ce sont les défauts de l'app, repris tels quels — un lecteur qui ouvre
    /// l'un puis l'autre doit voir la même chose. Et c'est le seul défaut
    /// honnête : le site montre d'abord ce que la traduction fait, puis laisse
    /// en retirer.
    fn default() -> Self {
        Self {
            francais: true,
            gloses: true,
            niveau_3: true,
            continu: false,
        }
    }
}

impl Preferences {
    /// Le corps seul — ni glose, ni translittération.
    ///
    /// C'est ce qu'emploie une citation hors de la liseuse : un aperçu de
    /// messagerie, une carte de partage. Sortie de son appareil critique, où
    /// elle est consultable et attribuée, une glose devient une affirmation
    /// sans recours.
    ///
    /// L'app a la même chose, et sous le même nom d'intention —
    /// `composeBare` force les deux à faux, indépendamment des réglages.
    pub fn nu() -> Self {
        Self {
            // Le registre reste le français : une citation sortie de son
            // appareil critique doit être **reconnaissable**. « Apocalypse »
            // hors contexte situe le lecteur ; « le machazeh de Yohanan » le
            // laisse devant un mot qu'aucune fiche n'accompagne plus.
            francais: true,
            gloses: false,
            niveau_3: false,
            continu: false,
        }
    }
}

/// Applique les réglages à un arbre de nœuds.
///
/// Trois passes, et l'ordre compte :
///
/// 1. **retirer** les nœuds éteints, en descendant dans les enfants ;
/// 2. **fondre** les fragments de texte devenus voisins — c'est ce qui met les
///    blancs orphelins côte à côte, où on peut les voir ;
/// 3. **resserrer** les blancs.
///
/// Sans la deuxième, la troisième ne verrait rien : chaque espace serait seul
/// dans son fragment, et parfaitement légitime.
pub fn preparer(noeuds: &[Noeud], preferences: Preferences) -> Vec<Noeud> {
    resserrer(fondre(retirer(noeuds, preferences)))
}

fn retirer(noeuds: &[Noeud], p: Preferences) -> Vec<Noeud> {
    noeuds
        .iter()
        .filter_map(|noeud| match noeud {
            Noeud::Glose(_) if !p.gloses => None,
            Noeud::Hebreu { .. } | Noeud::HebreuNu(_) if !p.niveau_3 => None,

            // Les conteneurs sont conservés, mais leur contenu est nettoyé :
            // une glose peut en contenir une autre, et une accentuation peut
            // contenir une translittération.
            Noeud::Glose(enfants) => Some(Noeud::Glose(retirer(enfants, p))),
            Noeud::Accentuation(enfants) => Some(Noeud::Accentuation(retirer(enfants, p))),
            Noeud::Emphase(enfants) => Some(Noeud::Emphase(retirer(enfants, p))),
            Noeud::Lien { href, enfants } => Some(Noeud::Lien {
                href: href.clone(),
                enfants: retirer(enfants, p),
            }),

            autre => Some(autre.clone()),
        })
        .collect()
}

/// Fond les fragments de texte devenus voisins.
///
/// **Récursive**, et ça n'allait pas de soi : un niveau 3 retiré à l'intérieur
/// d'une glose laisse deux fragments voisins *dans la glose*. Une fusion qui ne
/// travaillerait qu'au premier niveau ne les verrait jamais, et la glose
/// garderait son double blanc.
fn fondre(noeuds: Vec<Noeud>) -> Vec<Noeud> {
    let mut sortie: Vec<Noeud> = Vec::with_capacity(noeuds.len());
    for noeud in noeuds {
        let noeud = match noeud {
            Noeud::Glose(enfants) => Noeud::Glose(fondre(enfants)),
            Noeud::Accentuation(enfants) => Noeud::Accentuation(fondre(enfants)),
            Noeud::Emphase(enfants) => Noeud::Emphase(fondre(enfants)),
            Noeud::Lien { href, enfants } => Noeud::Lien {
                href,
                enfants: fondre(enfants),
            },
            autre => autre,
        };
        match (sortie.last_mut(), noeud) {
            (Some(Noeud::Texte(precedent)), Noeud::Texte(suivant)) => precedent.push_str(&suivant),
            (_, autre) => sortie.push(autre),
        }
    }
    sortie
}

/// Resserre les blancs qu'a laissés le retrait.
///
/// Deux règles, et la seconde est de la typographie française :
///
/// * une suite de blancs devient **une** espace ;
/// * l'espace disparaît devant `,` `.` `)` `]` `…`, jamais devant `:` `;` `!`
///   `?` `»`, qui en veulent une.
///
/// Le premier fragment perd son blanc de tête, le dernier son blanc de queue :
/// une glose en début de verset laissait la ligne commencer par un blanc.
fn resserrer(noeuds: Vec<Noeud>) -> Vec<Noeud> {
    let dernier = noeuds.len().saturating_sub(1);
    noeuds
        .into_iter()
        .enumerate()
        .map(|(rang, noeud)| match noeud {
            Noeud::Texte(texte) => {
                Noeud::Texte(resserrer_texte(&texte, rang == 0, rang == dernier))
            }
            Noeud::Glose(enfants) => Noeud::Glose(resserrer(enfants)),
            Noeud::Accentuation(enfants) => Noeud::Accentuation(resserrer(enfants)),
            Noeud::Emphase(enfants) => Noeud::Emphase(resserrer(enfants)),
            Noeud::Lien { href, enfants } => Noeud::Lien {
                href,
                enfants: resserrer(enfants),
            },
            autre => autre,
        })
        // Un fragment devenu vide n'a plus rien à faire là : il ferait un nœud
        // de texte sans texte, que le rendu traduirait en balise vide.
        .filter(|noeud| !matches!(noeud, Noeud::Texte(t) if t.is_empty()))
        .collect()
}

fn resserrer_texte(texte: &str, premier: bool, dernier: bool) -> String {
    let mut sortie = String::with_capacity(texte.len());
    let mut blanc_en_attente = false;

    for caractere in texte.chars() {
        if caractere.is_whitespace() {
            blanc_en_attente = true;
            continue;
        }
        if blanc_en_attente {
            blanc_en_attente = false;
            // L'espace ne survit pas devant la ponctuation qui n'en veut pas —
            // ni au tout début du verset.
            let ferme = matches!(caractere, ',' | '.' | ')' | ']' | '…');
            if !ferme && !(premier && sortie.is_empty()) {
                sortie.push(' ');
            }
        }
        sortie.push(caractere);
    }

    // Le blanc de queue ne survit qu'au milieu : il sépare de ce qui suit.
    //
    // La condition porte sur `premier`, **pas** sur « le fragment est vide ».
    // Un fragment qui ne contient qu'une espace est le cas le plus courant du
    // corpus : c'est celui qui sépare un intraduisible de sa translittération.
    // Le vider parce qu'il ne reste rien à sa gauche collait les deux mots —
    // et il l'aurait fait même quand on n'éteint rien du tout.
    if blanc_en_attente && !dernier && !(premier && sortie.is_empty()) {
        sortie.push(' ');
    }
    sortie
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Un verset réel, dans la forme que le pipeline produit : le mot, sa
    /// translittération, sa glose, puis la suite de la phrase.
    fn verset() -> Vec<Noeud> {
        vec![
            Noeud::Texte("Quand ".into()),
            Noeud::Intraduisible {
                mot: "Elohim".into(),
                lemme: "elohim".into(),
            },
            Noeud::Texte(" ".into()),
            Noeud::Hebreu {
                translitteration: "elohim".into(),
                hebreu: "אֱלֹהִים".into(),
            },
            Noeud::Texte(" ".into()),
            Noeud::Glose(vec![Noeud::Texte("nom divin laissé intact".into())]),
            Noeud::Texte(" commença à orchestrer".into()),
        ]
    }

    #[test]
    fn par_defaut_rien_n_est_retire() {
        assert_eq!(preparer(&verset(), Preferences::default()), verset());
    }

    /// Le fragment qui ne contient qu'une espace est le plus courant du
    /// corpus : celui qui sépare un intraduisible de sa translittération. Une
    /// version de `resserrer` le vidait, et collait les deux mots — sans qu'on
    /// ait rien éteint.
    #[test]
    fn une_espace_seule_entre_deux_noeuds_survit() {
        let noeuds = vec![
            Noeud::Intraduisible {
                mot: "Elohim".into(),
                lemme: "elohim".into(),
            },
            Noeud::Texte(" ".into()),
            Noeud::Accentuation(vec![Noeud::Texte("Cieux".into())]),
        ];
        assert_eq!(preparer(&noeuds, Preferences::default()), noeuds);
    }

    #[test]
    fn eteindre_les_gloses_ne_touche_pas_au_niveau_3() {
        let p = Preferences {
            gloses: false,
            ..Default::default()
        };
        let resultat = preparer(&verset(), p);
        assert!(!resultat.iter().any(|n| matches!(n, Noeud::Glose(_))));
        assert!(resultat.iter().any(|n| matches!(n, Noeud::Hebreu { .. })));
    }

    #[test]
    fn eteindre_le_niveau_3_ne_touche_pas_aux_gloses() {
        let p = Preferences {
            niveau_3: false,
            ..Default::default()
        };
        let resultat = preparer(&verset(), p);
        assert!(!resultat.iter().any(|n| matches!(n, Noeud::Hebreu { .. })));
        assert!(resultat.iter().any(|n| matches!(n, Noeud::Glose(_))));
    }

    /// Le point de tout ce module : le corps reste **une phrase**, pas une
    /// phrase trouée de blancs.
    #[test]
    fn le_retrait_ne_laisse_aucun_blanc_derriere_lui() {
        let resultat = preparer(&verset(), Preferences::nu());
        assert_eq!(
            resultat,
            vec![
                Noeud::Texte("Quand ".into()),
                Noeud::Intraduisible {
                    mot: "Elohim".into(),
                    lemme: "elohim".into(),
                },
                Noeud::Texte(" commença à orchestrer".into()),
            ]
        );
    }

    /// Le cas qui a mordu sur les aperçus de messagerie : la glose se pose
    /// avant la virgule.
    #[test]
    fn la_ponctuation_se_referme_sur_le_mot() {
        let noeuds = vec![
            Noeud::Texte("ni habitant ".into()),
            Noeud::Glose(vec![Noeud::Texte("tohu wa-bohu".into())]),
            Noeud::Texte(", et la face".into()),
        ];
        assert_eq!(
            preparer(&noeuds, Preferences::nu()),
            vec![Noeud::Texte("ni habitant, et la face".into())]
        );
    }

    /// Et le français garde l'espace qu'il exige devant les autres signes.
    #[test]
    fn le_deux_points_garde_son_espace() {
        let noeuds = vec![
            Noeud::Texte("Il dit ".into()),
            Noeud::Glose(vec![Noeud::Texte("wayomer".into())]),
            Noeud::Texte(" : que".into()),
        ];
        assert_eq!(
            preparer(&noeuds, Preferences::nu()),
            vec![Noeud::Texte("Il dit : que".into())]
        );
    }

    /// Une glose en tête de verset laissait la ligne commencer par un blanc.
    #[test]
    fn un_verset_ne_commence_pas_par_un_blanc() {
        let noeuds = vec![
            Noeud::Glose(vec![Noeud::Texte("incise".into())]),
            Noeud::Texte(" Et la Terre".into()),
        ];
        assert_eq!(
            preparer(&noeuds, Preferences::nu()),
            vec![Noeud::Texte("Et la Terre".into())]
        );
    }

    /// Une accentuation **survit** à l'extinction des niveaux — c'est la
    /// règle de l'app, et elle tient au sens : il appartient au corps.
    /// Mais ses enfants sont nettoyés.
    #[test]
    fn une_accentuation_survit_mais_son_contenu_est_nettoye() {
        let noeuds = vec![Noeud::Accentuation(vec![
            Noeud::Texte("Cieux".into()),
            Noeud::Hebreu {
                translitteration: "shamayim".into(),
                hebreu: "שָׁמַיִם".into(),
            },
        ])];
        assert_eq!(
            preparer(&noeuds, Preferences::nu()),
            vec![Noeud::Accentuation(vec![Noeud::Texte("Cieux".into())])]
        );
    }

    /// Un intraduisible ne s'éteint jamais : il est le texte, pas son
    /// commentaire.
    #[test]
    fn un_intraduisible_ne_s_eteint_jamais() {
        let resultat = preparer(&verset(), Preferences::nu());
        assert!(resultat
            .iter()
            .any(|n| matches!(n, Noeud::Intraduisible { .. })));
    }

    /// Une glose qui en contient une autre est retirée en entier, et une glose
    /// conservée voit ses enfants nettoyés.
    #[test]
    fn le_nettoyage_descend_dans_les_enfants() {
        let noeuds = vec![Noeud::Glose(vec![
            Noeud::Texte("forme de ".into()),
            Noeud::Hebreu {
                translitteration: "eloah".into(),
                hebreu: "אֱלוֹהַּ".into(),
            },
            Noeud::Texte(" au pluriel".into()),
        ])];
        let p = Preferences {
            niveau_3: false,
            ..Default::default()
        };
        assert_eq!(
            preparer(&noeuds, p),
            vec![Noeud::Glose(vec![Noeud::Texte(
                "forme de au pluriel".into()
            )])]
        );
    }
}

/// Le corps d'une suite de nœuds, en texte plat.
///
/// C'est [`Preferences::nu`] suivi d'un aplatissement : ce qu'on cite hors de
/// la liseuse — un aperçu de messagerie, une carte de partage.
///
/// Il vit ici et non dans [`crate::domaine::texte`] pour que la règle de
/// nettoyage n'existe **qu'une fois**. Elle y était écrite une seconde fois, et
/// deux copies d'une règle typographique finissent toujours par diverger d'un
/// signe que personne ne remarque.
pub fn corps(noeuds: &[Noeud]) -> String {
    fn aplatir(noeuds: &[Noeud], sortie: &mut String) {
        for noeud in noeuds {
            match noeud {
                Noeud::Texte(t) => sortie.push_str(t),
                Noeud::Intraduisible { mot, .. } | Noeud::Shem { mot, .. } => sortie.push_str(mot),
                Noeud::Accentuation(enfants)
                | Noeud::Emphase(enfants)
                | Noeud::Lien { enfants, .. } => aplatir(enfants, sortie),
                Noeud::Saut => sortie.push(' '),
                // `preparer` les a déjà retirés ; le cas reste pour que le
                // compilateur signale un nouveau niveau qu'on oublierait.
                Noeud::Glose(_) | Noeud::Hebreu { .. } | Noeud::HebreuNu(_) => {}
            }
        }
    }

    let mut sortie = String::new();
    aplatir(&preparer(noeuds, Preferences::nu()), &mut sortie);
    sortie
}
