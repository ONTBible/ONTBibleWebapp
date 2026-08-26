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
    /// Le nom français du livre — « Genèse » pour *Bereshit*.
    ///
    /// Il voyage pour les **métadonnées**, pas pour la page : le corps du site
    /// nomme les livres par leur titre hébreu, et c'est une décision. Mais
    /// personne ne cherche « Bereshit 1 » sans connaître déjà le projet, et
    /// c'est précisément ceux qui ne le connaissent pas qu'un moteur amène.
    pub livre_francais: String,
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
        livre_francais: ouvrage.francais.clone(),
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

// ───────────────────────────── le compte ──────────────────────────────────────

/// Ce que le navigateur sait du compte — c'est-à-dire presque rien.
///
/// **Ni jeton, ni identité, ni adresse.** Le navigateur n'a besoin que de savoir
/// s'il faut proposer de se connecter ou de se déconnecter ; tout le reste vit
/// dans le cookie `HttpOnly`, que rien dans la page ne peut lire. Faire voyager
/// l'adresse électronique du lecteur pour l'afficher serait une donnée de plus à
/// protéger pour un gain d'affichage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct EtatDuCompte {
    pub connecte: bool,
}

/// Le compte est-il ouvert ?
///
/// Lu dans le cookie, côté serveur. Une session périmée compte comme fermée :
/// le jeton d'accès ne vaut qu'une heure, et proposer « se déconnecter » à
/// quelqu'un dont la session est morte l'enverrait vers une erreur.
#[server(prefix = "/api", endpoint = "mon-compte")]
pub async fn mon_compte() -> Result<EtatDuCompte, ServerFnError> {
    Ok(EtatDuCompte {
        connecte: session_valide().await.is_some(),
    })
}

/// Les surlignages du lecteur pour une unité.
///
/// Vide quand il n'y a pas de compte — et **pas une erreur** : lire sans compte
/// est le cas normal du site, pas une panne. Une erreur ferait afficher un
/// message d'échec à quelqu'un qui n'a rien demandé.
#[server(prefix = "/api", endpoint = "mes-surlignages")]
pub async fn mes_surlignages(
    unite: String,
) -> Result<Vec<crate::domaine::surlignage::Surlignage>, ServerFnError> {
    let Some(session) = session_valide().await else {
        return Ok(Vec::new());
    };
    let sync = use_context::<std::sync::Arc<dyn crate::application::ports::Synchronisation>>()
        .ok_or_else(|| ServerFnError::new("synchronisation absente du contexte"))?;

    match sync.tirer(&session.access_token, None).await {
        Ok(moisson) => Ok(moisson
            .highlights
            .into_iter()
            // On filtre sur l'unité **et** sur la visibilité : le backend rend
            // tout le corpus et les pierres tombales avec. Les garder ici
            // dessinerait des surlignages que le lecteur a effacés ailleurs.
            .filter(|s| s.chapter_id == unite && s.visible())
            .collect()),
        // Une synchronisation qui échoue ne doit pas empêcher de lire : la page
        // s'affiche sans les marques, ce qui est exactement l'état de quelqu'un
        // qui n'a pas de compte.
        Err(_) => Ok(Vec::new()),
    }
}

/// Pose, change ou retire un surlignage.
///
/// `couleur` absente veut dire **retirer** — et c'est une pierre tombale qu'on
/// envoie, pas une absence. Le commentaire de l'app dit pourquoi :
///
/// > « Supprimer physiquement un surlignage ne se synchronise pas : l'appareil
/// > qui efface n'a plus rien à envoyer, et celui qui reçoit ne voit qu'un objet
/// > manquant — indistinguable d'un objet qu'il n'a pas encore. Il le renvoie
/// > donc, et le surlignage ressuscite au prochain échange. »
#[server(prefix = "/api", endpoint = "poser-surlignage")]
pub async fn poser_surlignage(
    livre: String,
    unite: String,
    versets: Vec<u32>,
    couleur: Option<String>,
    note: Option<String>,
) -> Result<bool, ServerFnError> {
    let Some(session) = session_valide().await else {
        return Ok(false);
    };
    let sync = use_context::<std::sync::Arc<dyn crate::application::ports::Synchronisation>>()
        .ok_or_else(|| ServerFnError::new("synchronisation absente du contexte"))?;

    let maintenant = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);

    let marques: Vec<crate::domaine::surlignage::Surlignage> = versets
        .into_iter()
        .map(|verset| crate::domaine::surlignage::Surlignage {
            // L'identifiant est composé et non tiré au hasard : le backend
            // apparie par `(chapter_id, verse)`, donc un identifiant neuf à
            // chaque envoi ne créerait pas de doublon — mais il rendrait deux
            // envois du même geste indiscernables dans les journaux.
            id: format!("{unite}-{verset}"),
            book_id: livre.clone(),
            chapter_id: unite.clone(),
            verse: verset,
            // Une pierre tombale garde une couleur, et l'app note qu'elle n'a
            // plus d'importance : « si elle est inconnue, on retombe sur une
            // valeur quelconque plutôt que de perdre la suppression ».
            color: couleur.clone().unwrap_or_else(|| "gold".to_string()),
            note: note.clone().filter(|n| !n.trim().is_empty()),
            updated_at: maintenant,
            deleted: couleur.is_none(),
        })
        .collect();

    // Aucune position ici : poser une couleur n'est pas lire. Elles voyagent
    // par la même route parce que le backend n'en offre qu'une, pas parce
    // qu'elles vont ensemble — mêler les deux ferait avancer le signet du
    // lecteur chaque fois qu'il surligne un verset qu'il vient de retrouver.
    Ok(sync
        .pousser(&session.access_token, &marques, None)
        .await
        .is_ok())
}

/// Où le lecteur en était, la dernière fois.
///
/// Vide sans compte, et **pas une erreur** — même règle que pour les
/// surlignages : lire sans compte est le cas normal du site.
#[server(prefix = "/api", endpoint = "ma-position")]
pub async fn ma_position() -> Result<Option<crate::domaine::surlignage::Position>, ServerFnError> {
    let Some(session) = session_valide().await else {
        return Ok(None);
    };
    let sync = use_context::<std::sync::Arc<dyn crate::application::ports::Synchronisation>>()
        .ok_or_else(|| ServerFnError::new("synchronisation absente du contexte"))?;

    Ok(sync
        .tirer(&session.access_token, None)
        .await
        .ok()
        .and_then(|m| m.position))
}

/// Retient où le lecteur en est.
///
/// ## Pourquoi ce n'est pas fait à chaque verset visible
///
/// L'app suit la visibilité des lignes et enregistre en continu ; le site
/// appelle cette fonction serveur, donc chaque appel est une requête réseau.
/// Le faire au défilement produirait des dizaines d'écritures par chapitre,
/// dont une seule compterait — la dernière.
///
/// Elle est donc appelée **à l'ouverture d'une unité**, et une fois. C'est plus
/// grossier que l'app, et c'est assumé : « Reprendre » ramène au début du
/// dernier chapitre ouvert plutôt qu'au verset exact. Un signet qui vise le
/// chapitre juste vaut mieux qu'un signet précis qui coûte cinquante requêtes.
#[server(prefix = "/api", endpoint = "retenir-la-position")]
pub async fn retenir_la_position(
    livre: String,
    unite: String,
    titre: String,
    verset: u32,
) -> Result<bool, ServerFnError> {
    let Some(session) = session_valide().await else {
        return Ok(false);
    };
    let sync = use_context::<std::sync::Arc<dyn crate::application::ports::Synchronisation>>()
        .ok_or_else(|| ServerFnError::new("synchronisation absente du contexte"))?;

    let maintenant = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);

    let position = crate::domaine::surlignage::Position {
        book_id: livre,
        chapter_id: unite,
        chapter_title: titre,
        verse: verset,
        updated_at: maintenant,
    };

    // Aucun surlignage : on ne pousse que la position. Le backend accepte une
    // liste vide — son `highlights` porte `#[serde(default)]` — et une liste
    // vide ne supprime rien, elle n'apparie simplement avec rien.
    Ok(sync
        .pousser(&session.access_token, &[], Some(&position))
        .await
        .is_ok())
}

/// La session du cookie, si elle vaut encore.
///
/// Trois causes d'absence, indistinguables ici et c'est voulu : pas de cookie,
/// cookie illisible, session périmée. Dans les trois cas le site se comporte
/// comme s'il n'y avait pas de compte — le seul comportement qui ne ment pas.
#[cfg(feature = "ssr")]
async fn session_valide() -> Option<crate::domaine::compte::Session> {
    use crate::interface::compte::{lire_cookie, COOKIE_SESSION};

    // `extract` plutôt que le contexte : Leptos n'y met pas la requête, et
    // l'extracteur d'axum est le chemin prévu depuis une fonction serveur.
    let entetes: axum::http::HeaderMap = leptos_axum::extract().await.ok()?;
    let brut = lire_cookie(&entetes, COOKIE_SESSION)?;
    let decode = percent_decode(&brut);
    let session: crate::domaine::compte::Session = serde_json::from_str(&decode).ok()?;

    let maintenant = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_millis() as i64;

    // Périmée : on ne renouvelle pas ici. Le renouvellement écrit un cookie, et
    // une fonction serveur ne compose pas la réponse — elle rendrait un jeton
    // neuf que rien ne garderait. C'est la route `/fr/compte/retour` qui pose
    // les cookies, et elle seule.
    (!session.perimee(maintenant)).then_some(session)
}

/// Décodage d'un composant d'adresse — l'inverse de `interface::compte::encoder`.
#[cfg(feature = "ssr")]
fn percent_decode(valeur: &str) -> String {
    let octets = valeur.as_bytes();
    let mut sortie = Vec::with_capacity(octets.len());
    let mut i = 0;
    while i < octets.len() {
        if octets[i] == b'%' && i + 2 < octets.len() {
            if let Ok(octet) = u8::from_str_radix(
                std::str::from_utf8(&octets[i + 1..i + 3]).unwrap_or("zz"),
                16,
            ) {
                sortie.push(octet);
                i += 3;
                continue;
            }
        }
        sortie.push(octets[i]);
        i += 1;
    }
    String::from_utf8_lossy(&sortie).into_owned()
}
