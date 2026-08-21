//! Le verset de démonstration.
//!
//! **Provisoire, et destiné à disparaître.**
//!
//! Deux pages montrent Bereshit 1:1 — l'accueil pour la comparaison, « Le
//! pourquoi » pour les trois niveaux. Le texte est écrit ici en dur, une seule
//! fois, le temps que le site sache lire `dist/` — le corpus construit par
//! `ONTBibleApp`.
//!
//! C'est la seule duplication du texte de la traduction dans ce dépôt. Elle
//! est isolée dans ce module pour qu'on sache exactement quoi supprimer le jour
//! où l'adaptateur de corpus existera : les composants, eux, prennent déjà des
//! `Noeud` du domaine et n'auront rien à changer.

use crate::domaine::texte::{Noeud, Verset};

/// Bereshit 1:1, tel que le pipeline le produit.
pub fn bereshit_1_1() -> Verset {
    Verset {
        numero: 1,
        noeuds: vec![
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
            Noeud::Texte(" commença à orchestrer ".into()),
            Noeud::Hebreu {
                translitteration: "bara".into(),
                hebreu: "בָּרָא".into(),
            },
            Noeud::Texte(" ".into()),
            Noeud::Glose(vec![Noeud::Texte(
                "à inaugurer dans l'existence fonctionnelle, à attribuer des rôles et des \
                 fonctions comme un roi investit son royaume"
                    .into(),
            )]),
            Noeud::Texte(" les ".into()),
            Noeud::Accentuation(vec![Noeud::Texte("Cieux".into())]),
            Noeud::Texte(" et la ".into()),
            Noeud::Accentuation(vec![Noeud::Texte("Terre".into())]),
            Noeud::Texte(" ".into()),
            Noeud::Hebreu {
                translitteration: "hashamayim ve'ha'aretz".into(),
                hebreu: "הַשָּׁמַיִם וְהָאָרֶץ".into(),
            },
            Noeud::Texte(" ".into()),
            Noeud::Glose(vec![Noeud::Texte(
                "c'est-à-dire la totalité du cosmos, du plus haut au plus bas".into(),
            )]),
            Noeud::Texte(" —".into()),
        ],
    }
}

/// Le même verset chez Louis Segond, 1910 — dans le domaine public.
///
/// Citer une traduction moderne serait une contrefaçon, et une démonstration
/// n'en a pas besoin : c'est justement une traduction ancienne et respectée
/// qui rend l'écart parlant.
pub const SEGOND_1910: &str = "Au commencement, Dieu créa les cieux et la terre.";
pub const SEGOND_SOURCE: &str = "Louis Segond, 1910 — domaine public";
