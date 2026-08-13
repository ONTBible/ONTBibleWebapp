//! Le corpus — sa structure, telle que la liseuse doit la montrer.
//!
//! Ces types sont le **portage** de ce que produit le pipeline. Ils lui sont
//! délibérément fidèles : ce qui s'imbrique dans le `.md` s'imbrique ici, et
//! une structure inventée pour le site aurait fini par diverger de la sienne.
//!
//! Mais fidèle n'est pas identique. Le pipeline sort du JSON à champs
//! optionnels, où « pas de sous-titre » et « sous-titre vide » se ressemblent.
//! Ici, un `Option` dit lequel des deux, et un `enum` interdit un type de bloc
//! qui n'existe pas. C'est l'analyse qui porte le doute, une fois, à la
//! frontière — pas chaque composant d'affichage.
//!
//! Ces types portent `Serialize` : ce sont eux qui voyagent du serveur au
//! navigateur, sans transport intermédiaire. La raison est écrite dans
//! [`crate::domaine::texte`] — c'est le même arbitrage.
//!
//! Aucune dépendance de données : ce module compile aussi bien pour le
//! navigateur que pour le serveur.

use serde::{Deserialize, Serialize};

use crate::domaine::texte::{Noeud, Verset};

/// L'état d'un chapitre dans le vault.
///
/// Le pipeline n'en connaît que deux — `locked` et `brouillon` — et le vault
/// en compte aujourd'hui 33 contre 6.
///
/// Le site montre **les deux**. Un brouillon caché donnerait un corpus plus
/// petit qu'il n'est, et le lecteur qui suit un lien vers un chapitre en cours
/// tomberait sur un 404 sans comprendre pourquoi. Il porte donc une mention :
/// le texte est là, et il est annoncé pour ce qu'il est.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Statut {
    /// `locked` — la traduction est arrêtée.
    Acheve,
    /// `brouillon` — elle peut encore bouger. Une mention l'accompagne.
    Brouillon,
}

impl Statut {
    /// Vrai si le chapitre doit porter une mention de brouillon.
    pub fn est_provisoire(self) -> bool {
        matches!(self, Statut::Brouillon)
    }
}

/// Un bloc de chapitre — le niveau au-dessus du verset.
///
/// Cinq formes, et c'est tout ce que le pipeline produit : versets, titres
/// intercalaires, listes, paragraphes de commentaire, filets de séparation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bloc {
    /// Une suite de versets. C'est la forme dominante — 613 blocs sur 806.
    Versets(Vec<Verset>),
    /// Un titre intercalaire. `niveau` vient du nombre de `#` du Markdown.
    Titre { niveau: u8, noeuds: Vec<Noeud> },
    /// Une liste. Chaque entrée est une suite de nœuds, donc un intraduisible
    /// dans une puce reste un intraduisible.
    Liste {
        ordonnee: bool,
        items: Vec<Vec<Noeud>>,
    },
    /// Un paragraphe de prose — une note, une remarque de traduction.
    Paragraphe(Vec<Noeud>),
    /// Une citation détachée — une question posée, une source rapportée.
    Citation(Vec<Noeud>),
    /// Un tableau. Chaque cellule est une suite de nœuds, donc un
    /// intraduisible reste cliquable dans une colonne.
    Tableau {
        entetes: Vec<Vec<Noeud>>,
        lignes: Vec<Vec<Vec<Noeud>>>,
    },
    /// Un filet de séparation.
    Filet,
}

/// Le sous-titre d'un chapitre : ce que la tradition en dit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SousTitre {
    /// Le nom français du livre — « Genèse ».
    pub francais: String,
    /// Son nom hébreu — « בְּרֵאשִׁית ».
    pub hebreu: String,
    /// Le renvoi classique — « 1:1 — 2:3 ».
    ///
    /// Il compte : les unités de l'ONT ne coïncident pas avec les chapitres
    /// reçus, et sans ce renvoi personne ne saurait où il se trouve.
    ///
    /// Absent sur une **introduction**, qui ne recouvre aucun verset. D'où
    /// l'`Option` : une chaîne vide se serait affichée comme un tiret orphelin.
    pub reference: Option<String>,
}

/// Un chapitre — l'unité de lecture, et l'unité de partage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chapitre {
    /// Son identifiant — « bereshit-1 ». C'est lui qui est dans l'adresse.
    pub id: String,
    /// L'identifiant du livre auquel il appartient.
    pub livre: String,
    /// Son rang dans le livre.
    pub numero: u32,
    /// Son titre — « Bereshit 1 ».
    pub titre: String,
    pub sous_titre: Option<SousTitre>,
    pub statut: Statut,
    pub blocs: Vec<Bloc>,
    /// Les notes de bas de chapitre, si le vault en porte.
    pub notes: Vec<Bloc>,
    pub nombre_de_versets: u32,
}

impl Chapitre {
    /// Un verset par son numéro.
    ///
    /// Il faut traverser les blocs : le pipeline découpe un chapitre en
    /// plusieurs blocs de versets séparés par des titres intercalaires, donc la
    /// position d'un verset dans le chapitre n'est pas son numéro.
    pub fn verset(&self, numero: u32) -> Option<&Verset> {
        self.blocs.iter().find_map(|bloc| match bloc {
            Bloc::Versets(versets) => versets.iter().find(|v| v.numero == numero),
            _ => None,
        })
    }

    /// Tous les versets du chapitre, dans l'ordre, blocs confondus.
    pub fn versets(&self) -> impl Iterator<Item = &Verset> {
        self.blocs.iter().flat_map(|bloc| match bloc {
            Bloc::Versets(versets) => versets.as_slice(),
            _ => &[],
        })
    }
}

/// Un livre, avec ses chapitres.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Livre {
    pub id: String,
    /// Le titre translittéré — « Bereshit ».
    pub titre: String,
    /// Le nom reçu en français — « Genèse ».
    pub francais: String,
    pub hebreu: String,
    /// L'introduction, quand le livre en a une. Deux en portent aujourd'hui.
    pub intro: Option<Chapitre>,
    pub chapitres: Vec<Chapitre>,
}

impl Livre {
    /// Un chapitre par son identifiant — l'introduction comprise.
    ///
    /// L'introduction est rangée à part dans le pipeline mais elle est servie
    /// par la même adresse : pour un lecteur, c'est une unité comme une autre.
    pub fn chapitre(&self, id: &str) -> Option<&Chapitre> {
        self.chapitres
            .iter()
            .chain(self.intro.iter())
            .find(|c| c.id == id)
    }

    /// Le nombre de versets du livre.
    pub fn nombre_de_versets(&self) -> u32 {
        self.chapitres.iter().map(|c| c.nombre_de_versets).sum()
    }
}

/// L'entrée d'un livre au sommaire — sans son texte.
///
/// Le sommaire cite les 70 livres ; trois seulement sont écrits. Charger leur
/// contenu pour dresser une liste coûterait le corpus entier à chaque visite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntreeDeLivre {
    pub id: String,
    pub titre: String,
    pub francais: String,
    pub hebreu: String,
    /// Faux tant que le livre n'a pas une ligne. Il reste au sommaire :
    /// l'ampleur du chantier fait partie de ce que le site dit.
    pub ecrit: bool,
    /// Le nombre d'unités déjà traduites.
    pub unites: u32,
}

/// Une section du sommaire — Torah, Nevi'im, Ketouvim, Nistarot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Section {
    pub id: String,
    pub titre: String,
    pub livres: Vec<EntreeDeLivre>,
}

/// L'un des deux ensembles — Kenesset, Berit Hadashah.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ensemble {
    pub id: String,
    pub titre: String,
    pub sections: Vec<Section>,
}

impl Ensemble {
    /// Les livres qui ont du texte, toutes sections confondues.
    pub fn livres_ecrits(&self) -> impl Iterator<Item = &EntreeDeLivre> {
        self.sections
            .iter()
            .flat_map(|s| s.livres.iter())
            .filter(|l| l.ecrit)
    }
}

/// Une fiche de lexique.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entree {
    pub lemme: String,
    pub titre: String,
    pub hebreu: String,
    /// Comment le terme est rendu, et où — « l'Être façonné du sol
    /// (Bereshit 1-7) ».
    pub rendu: String,
    /// Les formes attestées, quand elles diffèrent du lemme.
    pub formes: Vec<String>,
    pub definition: Vec<Bloc>,
}

/// Un endroit du corpus où le terme paraît.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Occurrence {
    pub livre: String,
    pub chapitre: String,
    /// Le verset, quand le terme y tombe.
    ///
    /// `None` pour les 319 occurrences qui vivent dans un titre intercalaire ou
    /// une note de chapitre : elles sont réelles, mais elles ne pointent aucun
    /// numéro. Le lien mène alors au chapitre.
    pub verset: Option<u32>,
    /// La forme employée là — elle peut différer du lemme.
    pub forme: String,
    /// Le contexte, tel que le pipeline le coupe.
    pub extrait: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verset(numero: u32) -> Verset {
        Verset {
            numero,
            noeuds: vec![Noeud::Texte(format!("verset {numero}"))],
        }
    }

    fn chapitre() -> Chapitre {
        Chapitre {
            id: "bereshit-1".into(),
            livre: "bereshit".into(),
            numero: 1,
            titre: "Bereshit 1".into(),
            sous_titre: None,
            statut: Statut::Brouillon,
            // Deux blocs de versets séparés par un titre : c'est la forme
            // réelle d'un chapitre du vault, et celle qui piège une recherche
            // naïve par indice.
            blocs: vec![
                Bloc::Versets(vec![verset(1), verset(2)]),
                Bloc::Titre {
                    niveau: 2,
                    noeuds: vec![Noeud::Texte("L'état sans ordre".into())],
                },
                Bloc::Versets(vec![verset(3)]),
            ],
            notes: vec![],
            nombre_de_versets: 3,
        }
    }

    #[test]
    fn un_verset_se_trouve_par_dela_les_blocs() {
        // Le troisième verset vit dans le second bloc. Le chercher à l'indice 2
        // du premier échouerait.
        assert_eq!(chapitre().verset(3), Some(&verset(3)));
        assert_eq!(chapitre().verset(9), None);
    }

    #[test]
    fn les_versets_sortent_dans_l_ordre_du_chapitre() {
        let c = chapitre();
        let numeros: Vec<u32> = c.versets().map(|v| v.numero).collect();
        assert_eq!(numeros, [1, 2, 3]);
    }

    #[test]
    fn l_introduction_est_un_chapitre_comme_un_autre() {
        let mut intro = chapitre();
        intro.id = "bereshit-intro".into();
        let livre = Livre {
            id: "bereshit".into(),
            titre: "Bereshit".into(),
            francais: "Genèse".into(),
            hebreu: "בְּרֵאשִׁית".into(),
            intro: Some(intro),
            chapitres: vec![chapitre()],
        };
        // Rangée à part par le pipeline, elle se trouve par la même adresse.
        assert!(livre.chapitre("bereshit-intro").is_some());
        assert!(livre.chapitre("bereshit-1").is_some());
    }
}
