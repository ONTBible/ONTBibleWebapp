//! La frontière client ↔ serveur.
//!
//! C'est le seul module que les deux côtés partagent en entier, et c'est le
//! seul endroit où l'on sérialise. Les types qui voyagent sont donc des
//! **transports** et non des types du domaine : ils portent la forme du fil,
//! pas les invariants du métier.
//!
//! La conversion est ce qui protège le domaine. Le jour où le fil demandera un
//! champ de plus — un identifiant de fiche, un horodatage — c'est le transport
//! qui l'apprendra, pas `VersetQuotidien`.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Le verset du jour, tel qu'il voyage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersetDuJourDto {
    pub renvoi: String,
    pub texte: String,
    pub chemin: String,
}

/// Le verset du jour.
///
/// Rendu côté serveur, puis sérialisé dans la page : le navigateur n'émet
/// aucune requête pour l'obtenir. C'est ce qui permet à la carte d'être
/// lisible avant que le JavaScript n'arrive — et de le rester s'il n'arrive
/// jamais.
///
/// `Ok(None)` et non une erreur quand le vivier est vide : ce n'est pas une
/// panne, c'est un corpus qui n'a encore rien de verrouillé. La page se tait.
#[server(prefix = "/api", endpoint = "verset-du-jour")]
pub async fn verset_du_jour() -> Result<Option<VersetDuJourDto>, ServerFnError> {
    use std::sync::Arc;

    use crate::application::ports::{Horloge, Vivier};
    use crate::application::verset_du_jour::VersetDuJour;

    // Les deux dépendances viennent de la racine de composition, jamais d'un
    // global : c'est `main.rs` qui décide de quelle horloge et de quel vivier
    // il s'agit, et personne d'autre.
    let horloge = use_context::<Arc<dyn Horloge>>()
        .ok_or_else(|| ServerFnError::new("horloge absente du contexte"))?;
    let vivier = use_context::<Arc<dyn Vivier>>()
        .ok_or_else(|| ServerFnError::new("vivier absent du contexte"))?;

    Ok(VersetDuJour::new(horloge.as_ref(), vivier.as_ref())
        .aujourd_hui()
        .map(|v| VersetDuJourDto {
            renvoi: v.renvoi.clone(),
            texte: v.texte.clone(),
            chemin: v.chemin(),
        }))
}

// ───────────────────────────── la liseuse ─────────────────────────────────────

/// Le sommaire du corpus.
///
/// Les 70 livres, pas seulement les trois écrits : l'ampleur du chantier fait
/// partie de ce que le site dit. Un sommaire qui ne montrerait que l'écrit
/// laisserait croire que le corpus tient en trois livres.
#[server(prefix = "/api", endpoint = "sommaire")]
pub async fn sommaire() -> Result<Vec<crate::domaine::corpus::Ensemble>, ServerFnError> {
    Ok(corpus()?.sommaire().to_vec())
}

/// Une unité au sommaire d'un livre — sans son texte.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniteDto {
    pub id: String,
    pub titre: String,
    /// Le rang de l'unité dans son livre — 0 pour une introduction.
    ///
    /// La liste compose son libellé à partir de lui, au lieu de reprendre
    /// `titre` : celui-ci porte le nom du livre — « Bereshit 7 » — répété à
    /// chaque ligne alors qu'on est déjà dans *Bereshit*.
    pub numero: u32,
    /// Le renvoi classique — « 1:1 — 2:3 ». Absent sur une introduction.
    pub reference: Option<String>,
    pub brouillon: bool,
    pub versets: u32,
}

/// Un livre et la liste de ses unités.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivreDto {
    pub id: String,
    pub titre: String,
    pub francais: String,
    pub hebreu: String,
    pub unites: Vec<UniteDto>,
    pub versets: u32,
}

/// Le sommaire d'un livre.
///
/// Il ne porte **pas** le texte des chapitres. Un livre complet pèse plusieurs
/// centaines de kilo-octets ; les envoyer pour dresser une liste de dix-neuf
/// lignes se paierait sur chaque visite, et deux fois — une pour le rendu,
/// une pour l'hydratation.
#[server(prefix = "/api", endpoint = "livre")]
pub async fn livre(id: String) -> Result<Option<LivreDto>, ServerFnError> {
    let Some(livre) = corpus()?.livre(&id) else {
        return Ok(None);
    };

    // L'introduction d'abord : c'est sa place dans le livre.
    let unites = livre
        .intro
        .iter()
        .chain(livre.chapitres.iter())
        .map(|c| UniteDto {
            id: c.id.clone(),
            titre: c.titre.clone(),
            numero: c.numero,
            reference: c.sous_titre.as_ref().and_then(|s| s.reference.clone()),
            brouillon: c.statut.est_provisoire(),
            versets: c.nombre_de_versets,
        })
        .collect();

    Ok(Some(LivreDto {
        id: livre.id.clone(),
        titre: livre.titre.clone(),
        francais: livre.francais.clone(),
        hebreu: livre.hebreu.clone(),
        unites,
        versets: livre.nombre_de_versets(),
    }))
}

/// Un voisin d'unité, pour la navigation de bas de page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoisinDto {
    pub chemin: String,
    pub titre: String,
    /// Le rang de l'unité — pour la nommer dans le registre du lecteur.
    ///
    /// Le titre seul ne suffit pas : « Bereshit 2 » ne dit pas que c'est la
    /// deuxième, et la navigation afficherait le nom ONT là où le reste de la
    /// page suit le réglage.
    pub numero: u32,
}

/// Un passage, avec ce qu'il faut pour le situer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PassageDto {
    pub livre_id: String,
    pub livre_titre: String,
    pub chapitre: crate::domaine::corpus::Chapitre,
    pub precedent: Option<VoisinDto>,
    pub suivant: Option<VoisinDto>,
}

/// Un passage de la liseuse.
///
/// C'est la route qu'ouvrent les liens partagés depuis l'app — et celle que
/// l'association d'app réserve à iOS (voir [`crate::interface::association`]).
/// Elle doit donc répondre quoi qu'il arrive, y compris pour un chapitre encore
/// en brouillon : quelqu'un qui suit un lien vers un texte en cours doit voir
/// ce texte et sa mention, pas une page d'erreur.
#[server(prefix = "/api", endpoint = "passage")]
pub async fn passage(livre: String, unite: String) -> Result<Option<PassageDto>, ServerFnError> {
    let Some(ouvrage) = corpus()?.livre(&livre) else {
        return Ok(None);
    };
    let Some(chapitre) = ouvrage.chapitre(&unite) else {
        return Ok(None);
    };

    // Les voisins se prennent dans l'ordre de lecture — introduction comprise,
    // puisqu'elle se lit avant le premier chapitre.
    let ordre: Vec<&crate::domaine::corpus::Chapitre> = ouvrage
        .intro
        .iter()
        .chain(ouvrage.chapitres.iter())
        .collect();
    let rang = ordre.iter().position(|c| c.id == unite);

    let voisin = |indice: Option<usize>| {
        indice
            .and_then(|i| ordre.get(i))
            .map(|c: &&crate::domaine::corpus::Chapitre| VoisinDto {
                chemin: format!("/fr/lire/{livre}/{}", c.id),
                titre: c.titre.clone(),
                numero: c.numero,
            })
    };

    Ok(Some(PassageDto {
        livre_id: ouvrage.id.clone(),
        livre_titre: ouvrage.titre.clone(),
        chapitre: chapitre.clone(),
        precedent: voisin(rang.and_then(|r| r.checked_sub(1))),
        suivant: voisin(rang.map(|r| r + 1)),
    }))
}

/// Quelques versets nommés, tirés du corpus.
///
/// C'est ce dont une page d'argumentation a besoin : citer *Bereshit* 1:2
/// pour montrer le **tohu vavohu**, sans embarquer les trente et un versets du
/// chapitre dans le HTML pour en montrer un.
///
/// Les numéros absents du chapitre sont **omis** plutôt que signalés : une
/// page qui cite se tait sur ce qu'elle ne trouve pas, elle ne tombe pas. Le
/// jour où une unité serait renumérotée, la démonstration perdrait un verset —
/// visible à la relecture, et sans page d'erreur pour le lecteur.
#[server(prefix = "/api", endpoint = "versets")]
pub async fn versets(
    livre: String,
    unite: String,
    numeros: Vec<u32>,
) -> Result<Vec<crate::domaine::texte::Verset>, ServerFnError> {
    let Some(ouvrage) = corpus()?.livre(&livre) else {
        return Ok(Vec::new());
    };
    let Some(chapitre) = ouvrage.chapitre(&unite) else {
        return Ok(Vec::new());
    };

    // L'ordre est celui de `numeros` et non celui du chapitre : une
    // démonstration cite parfois à rebours, et c'est elle qui décide.
    Ok(numeros
        .into_iter()
        .filter_map(|n| chapitre.verset(n).cloned())
        .collect())
}

/// Le résumé d'une fiche, pour l'index du lexique.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeDto {
    pub lemme: String,
    pub titre: String,
    pub hebreu: String,
    pub rendu: String,
}

/// L'index du lexique.
#[server(prefix = "/api", endpoint = "lexique")]
pub async fn lexique() -> Result<Vec<ResumeDto>, ServerFnError> {
    Ok(glossaire()?
        .entrees()
        .iter()
        .map(|e| ResumeDto {
            lemme: e.lemme.clone(),
            titre: e.titre.clone(),
            hebreu: e.hebreu.clone(),
            rendu: e.rendu.clone(),
        })
        .collect())
}

/// Une fiche et ses renvois.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FicheDto {
    pub entree: crate::domaine::corpus::Entree,
    pub occurrences: Vec<crate::domaine::corpus::Occurrence>,
}

/// Une fiche du lexique.
///
/// C'est ce que promet chaque mot d'or du corpus. Les occurrences en font
/// partie : une fiche qui définit sans montrer où le terme paraît laisse le
/// lecteur avec une définition et aucun moyen de la vérifier.
#[server(prefix = "/api", endpoint = "fiche")]
pub async fn fiche(lemme: String) -> Result<Option<FicheDto>, ServerFnError> {
    let lexique = glossaire()?;
    Ok(lexique.entree(&lemme).map(|entree| FicheDto {
        entree: entree.clone(),
        occurrences: lexique.occurrences(&lemme),
    }))
}

/// Le corpus, pris dans le contexte de la requête.
#[cfg(feature = "ssr")]
fn corpus() -> Result<std::sync::Arc<dyn crate::application::ports::Corpus>, ServerFnError> {
    use_context::<std::sync::Arc<dyn crate::application::ports::Corpus>>()
        .ok_or_else(|| ServerFnError::new("corpus absent du contexte"))
}

/// Le lexique, pris dans le contexte de la requête.
#[cfg(feature = "ssr")]
fn glossaire() -> Result<std::sync::Arc<dyn crate::application::ports::Lexique>, ServerFnError> {
    use_context::<std::sync::Arc<dyn crate::application::ports::Lexique>>()
        .ok_or_else(|| ServerFnError::new("lexique absent du contexte"))
}
