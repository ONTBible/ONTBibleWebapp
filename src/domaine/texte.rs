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

/// Un fragment de texte ONT.
///
/// L'arbre est volontairement fidèle à ce que produit le pipeline : ce qui
/// s'imbrique dans le `.md` s'imbrique ici. Aplatir ferait perdre les gloses
/// qui contiennent elles-mêmes un intraduisible — et il y en a.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Noeud {
    /// Niveau 1 — ce que l'hébreu dit directement.
    Texte(String),
    /// Un intraduisible. `lemme` désigne sa fiche de lexique.
    Intraduisible { mot: String, lemme: String },
    /// Un terme important — marqué, mais sans fiche.
    Important(String),
    /// Niveau 2 — ce que le champ sémantique hébreu porte implicitement.
    Glose(Vec<Noeud>),
    /// Niveau 3 — la translittération et l'hébreu, toujours les deux.
    Hebreu { translitteration: String, hebreu: String },
    /// Une emphase ordinaire, dans une glose.
    Emphase(Vec<Noeud>),
}

/// Un verset, avec son numéro d'unité.
#[derive(Debug, Clone, PartialEq, Eq)]
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
        fn parcourir(noeuds: &[Noeud], sortie: &mut String) {
            for noeud in noeuds {
                match noeud {
                    Noeud::Texte(t) | Noeud::Important(t) => sortie.push_str(t),
                    Noeud::Intraduisible { mot, .. } => sortie.push_str(mot),
                    Noeud::Emphase(enfants) => parcourir(enfants, sortie),
                    // Écartés : ce sont les niveaux 2 et 3.
                    Noeud::Glose(_) | Noeud::Hebreu { .. } => {}
                }
            }
        }

        let mut sortie = String::new();
        parcourir(&self.noeuds, &mut sortie);
        // Retirer les blancs que laisse la disparition des gloses.
        sortie.split_whitespace().collect::<Vec<_>>().join(" ")
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

    #[test]
    fn le_corps_ne_laisse_pas_de_blancs_doubles() {
        // La disparition d'une glose laisse deux espaces autour d'elle. Sur
        // une carte de partage, ça se voit.
        assert!(!verset().corps().contains("  "));
    }
}
