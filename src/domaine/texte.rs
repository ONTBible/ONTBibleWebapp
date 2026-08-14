//! Les trois niveaux du texte ONT.
//!
//! C'est la raison d'être de tout le pipeline, et donc le cœur de ce que le
//! site doit rendre. Un affichage qui les confond détruit ce que la traduction
//! a construit.
//!
//! | dans le `.md` | niveau | rendu |
//! |---|---|---|
//! | texte nu | 1 — le corps | encre |
//! | `**mot**` | intraduisible | or, et **cliquable** vers sa fiche |
//! | `==mot==` | terme important | bordeaux clair, et **inerte** |
//! | `*[glose]*` | 2 | plus petit, encre atténuée |
//! | `(*translit* / hébreu)` | 3 | italique + Ezra SIL |
//!
//! La distinction entre l'or et le bordeaux n'est pas décorative : **l'or
//! promet une fiche et la tient**, le bordeaux marque sans rien promettre. Les
//! confondre ferait mentir l'un des deux.
//!
//! ## Pourquoi ces types portent `Serialize`
//!
//! `api.rs` pose une règle : ce qui voyage sur le fil est un **transport**, pas
//! un type du domaine — et la conversion protège le domaine des exigences du
//! fil. Elle vaut, et elle est tenue pour le verset du jour, qui est plat.
//!
//! Elle est **écartée ici**, et c'est un arbitrage, pas un oubli. Recopier cet
//! arbre en transport ferait deux énumérations récursives à tenir d'accord, et
//! surtout **trois** endroits à modifier au prochain type de nœud que le
//! pipeline inventera : le domaine, le transport, et la conversion. Le coût est
//! certain, le bénéfice nul — un `Noeud` n'a pas d'invariant que la
//! sérialisation pourrait violer, c'est une forme, pas une règle.
//!
//! Et `derive` ne fait entrer aucun type étranger dans la couche : la règle du
//! domaine — ne dépendre que de soi et de la bibliothèque standard — reste
//! vraie de ses **types**, qui sont ce qu'elle protège.

use serde::{Deserialize, Serialize};

/// Un fragment de texte ONT.
///
/// L'arbre est volontairement fidèle à ce que produit le pipeline : ce qui
/// s'imbrique dans le `.md` s'imbrique ici. Aplatir ferait perdre les gloses
/// qui contiennent elles-mêmes un intraduisible — et il y en a.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Noeud {
    /// Niveau 1 — ce que l'hébreu dit directement.
    Texte(String),
    /// Un intraduisible. `lemme` désigne sa fiche de lexique.
    Intraduisible { mot: String, lemme: String },
    /// Un terme important — marqué, mais sans fiche.
    ///
    /// Il porte des enfants et non une chaîne, parce que le pipeline en met :
    /// un `==…==` peut contenir un intraduisible. L'aplatir ici perdrait le
    /// lien vers la fiche, en silence.
    Important(Vec<Noeud>),
    /// Niveau 2 — ce que le champ sémantique hébreu porte implicitement.
    Glose(Vec<Noeud>),
    /// Niveau 3 — la translittération et l'hébreu, toujours les deux.
    Hebreu {
        translitteration: String,
        hebreu: String,
    },
    /// De l'hébreu **seul**, sans translittération.
    ///
    /// Il sert dans les fiches de lexique, où l'on cite parfois un fragment
    /// isolé — un suffixe, une racine — qui n'a pas de translittération propre.
    /// Un cas distinct de [`Noeud::Hebreu`] : composer un suffixe entre
    /// parenthèses avec une barre oblique et rien à sa gauche donnerait une
    /// forme absurde.
    HebreuNu(String),
    /// Un lien vers l'extérieur — une source, un manuscrit en ligne.
    Lien { href: String, enfants: Vec<Noeud> },
    /// Une emphase ordinaire, dans une glose.
    Emphase(Vec<Noeud>),
    /// Un retour à la ligne **dans** un verset.
    ///
    /// Rare — quatre dans tout le vault — mais il porte du sens : c'est la
    /// coupe d'un parallélisme poétique. L'écraser en espace ferait d'un
    /// distique une phrase.
    Saut,
}

/// Un verset, avec son numéro d'unité.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Verset {
    pub numero: u32,
    pub noeuds: Vec<Noeud>,
}

impl Verset {
    /// Le corps seul — ce qu'on cite hors de la liseuse.
    ///
    /// Ni gloses, ni translittérations. Sorti de son appareil critique, où il
    /// est consultable et attribué, le niveau 2 devient une affirmation sans
    /// recours pour qui ne connaît pas le projet.
    pub fn corps(&self) -> String {
        crate::domaine::lecture::corps(&self.noeuds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verset() -> Verset {
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
                Noeud::Texte(" ".into()),
                Noeud::Glose(vec![Noeud::Texte("nom divin laissé intact".into())]),
                Noeud::Texte(" commença à orchestrer".into()),
            ],
        }
    }

    #[test]
    fn le_corps_ecarte_les_niveaux_2_et_3() {
        assert_eq!(verset().corps(), "Quand Elohim commença à orchestrer");
    }

    /// Le cas réel : une glose posée entre un mot et sa virgule.
    #[test]
    fn le_corps_referme_la_ponctuation_que_la_glose_laissait_ouverte() {
        let verset = Verset {
            numero: 2,
            noeuds: vec![
                Noeud::Texte("ni habitant ".into()),
                Noeud::Glose(vec![Noeud::Texte("tohu wa-bohu".into())]),
                Noeud::Texte(", et la face des eaux.".into()),
            ],
        };
        assert_eq!(verset.corps(), "ni habitant, et la face des eaux.");
    }

    /// Et le français garde l'espace qu'il exige devant les autres signes.
    #[test]
    fn le_corps_garde_l_espace_avant_le_deux_points_et_les_guillemets() {
        let verset = Verset {
            numero: 3,
            noeuds: vec![Noeud::Texte("Il dit : « que la lumière soit ! »".into())],
        };
        assert_eq!(verset.corps(), "Il dit : « que la lumière soit ! »");
    }

    #[test]
    fn le_corps_ne_laisse_pas_de_blancs_doubles() {
        // La disparition d'une glose laisse deux espaces autour d'elle. Sur
        // une carte de partage, ça se voit.
        assert!(!verset().corps().contains("  "));
    }
}
