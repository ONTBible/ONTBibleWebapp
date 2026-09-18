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
//! | `==mot==` | accentuation | bordeaux clair, et **inerte** |
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

/// Ce qu'une translittération de niveau 3 ouvre.
///
/// **Deux destinations qui ne se confondent pas.** Le site les mène toutes deux
/// vers `/fr/lexique/{lemme}` — l'adresse est la même —, mais la couleur les
/// sépare : l'accent pour un intraduisible, la teinte des Shemot pour un nom
/// propre. C'est déjà la règle du corps du texte, et le niveau 3 la reprend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CibleDuNiveauTrois {
    /// Une entrée du glossaire.
    Terme(String),
    /// Une fiche de Shem.
    Shem(String),
}

/// Ce qu'une référence biblique désigne, quand le corpus porte le passage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CibleDeLaReference {
    /// Le livre qui porte l'unité — `bereshit`.
    pub livre: String,
    /// L'unité à ouvrir — `bereshit-7`.
    pub unite: String,
    /// Le verset à désigner en arrivant, dans la numérotation de l'unité.
    ///
    /// Nul quand la référence vise un chapitre entier, et nul aussi quand le
    /// pipeline n'a pas pu confirmer son calcul. **Mieux vaut ouvrir la bonne
    /// unité sans rien désigner que d'en désigner un faux** — c'est sa règle,
    /// et le site n'a pas à la rejouer.
    pub verset: Option<u32>,
}

/// L'étendue que la référence nomme — ce que le lecteur lit dans le libellé.
///
/// Conservée jusqu'ici alors que la navigation n'emploie que `cible`, pour une
/// raison de garde : le `match` qui la traduit est **exhaustif**, donc une
/// quatrième étendue ajoutée au pipeline casserait la compilation du site au
/// lieu de s'y perdre. C'est la règle déjà appliquée à `CibleDuNiveauTrois`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PorteeDeLaReference {
    /// `Genèse 3` — l'unité entière.
    Chapitre,
    /// `Genèse 3:24` — un verset.
    Verset { n: u32 },
    /// `Genèse 1:11-12` — une plage. La navigation vise son ouverture.
    Plage { premier: u32, dernier: u32 },
}

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
    /// Un **Shem** — un nom propre hébreu qui porte une fiche.
    ///
    /// Distinct de [`Noeud::Accentuation`], qui marque un nom propre sans rien
    /// promettre. Le Shem promet une fiche et la tient, comme l'intraduisible :
    /// ce sont les deux marques du corpus sur lesquelles on peut cliquer.
    ///
    /// Il porte un `mot` et non des enfants, pour la même raison que
    /// l'intraduisible : c'est un nom, pas un conteneur.
    Shem { mot: String, lemme: String },
    /// Une accentuation — marquée, mais sans fiche.
    ///
    /// Il porte des enfants et non une chaîne, parce que le pipeline en met :
    /// un `==…==` peut contenir un intraduisible. L'aplatir ici perdrait le
    /// lien vers la fiche, en silence.
    Accentuation(Vec<Noeud>),
    /// Niveau 2 — ce que le champ sémantique hébreu porte implicitement.
    Glose(Vec<Noeud>),
    /// Niveau 3 — la translittération et l'hébreu, toujours les deux.
    Hebreu {
        translitteration: String,
        hebreu: String,
        /// La fiche que la translittération ouvre, **quand elle en ouvre une**.
        ///
        /// Le lecteur est sur le mot hébreu : c'est le moment où il veut sa
        /// fiche, et l'appareil s'arrêtait au corps du texte.
        ///
        /// **`None` est le cas ordinaire**, et il est honnête. Le pipeline ne
        /// résout que l'exact — un lemme, une forme déclarée au §2.5, un Shem
        /// publié — et laisse inerte le reste : une règle morphologique qui se
        /// trompe ne rend pas le mot inerte, elle le rend cliquable **vers la
        /// mauvaise fiche**, ce que le lecteur ne peut pas voir.
        cible: Option<CibleDuNiveauTrois>,
    },
    /// Un renvoi d'une **chuqqah** vers une autre — `((cible|libellé))`.
    ///
    /// ## Il ne mène nulle part, et c'est délibéré
    ///
    /// Le site n'a pas de section chuqqot : son espace d'adresses va de
    /// `/fr/lire` à `/fr/lexique`, et rien entre les deux. Fabriquer un
    /// `<a href="/fr/chuqqot/…">` donnerait un lien vers un 404 — un mot
    /// coloré qui n'ouvre rien, exactement le défaut que le pipeline refuse
    /// partout ailleurs en laissant une translittération inerte plutôt que de
    /// la renvoyer vers une fiche absente.
    ///
    /// Le libellé garde donc sa teinte — le lecteur voit qu'il désigne autre
    /// chose — et ne se clique pas.
    ///
    /// **La condition pour le lever tient en une ligne** : le jour où le site
    /// publie les chuqqot, ce nœud devient un lien vers `cible`. C'est le seul
    /// changement à faire, et il est ici.
    Renvoi { libelle: String, cible: String },
    /// Une **référence biblique** — `*Genèse* 9:27` dans le corps d'un texte.
    ///
    /// Distincte de [`Noeud::Renvoi`], et la distinction n'est pas de forme mais
    /// de destination : un `Renvoi` vise une *chuqqah*, une `Reference` vise un
    /// *passage du corpus*. L'une attend que le site publie les chuqqot, l'autre
    /// peut aboutir dès aujourd'hui.
    ///
    /// `cible` est nulle quand le livre cité **n'est pas traduit** — 208 des 915
    /// références. Le pipeline le dit explicitement plutôt que de laisser le
    /// client chercher un livre qu'il ne trouvera pas.
    ///
    /// **Les deux cas portent la même apparence**, décidé par l'auteur le
    /// 11 septembre 2026 : une référence qui n'aboutit pas reste une référence,
    /// et la farder autrement apprendrait au lecteur à ne plus les voir.
    Reference {
        libelle: String,
        /// Le livre tel que la référence le nomme — `Genèse`. Sert à dire
        /// *lequel* manque quand la cible est nulle : « Ésaïe n'est pas encore
        /// traduit » est une réponse, « ce lien ne mène nulle part » n'en est
        /// pas une.
        livre_cite: String,
        portee: PorteeDeLaReference,
        cible: Option<CibleDeLaReference>,
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
                    cible: None,
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
