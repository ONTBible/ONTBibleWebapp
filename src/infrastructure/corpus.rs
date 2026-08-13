//! Le corpus, embarqué à la compilation.
//!
//! ## Pourquoi embarqué, et non lu sur le disque
//!
//! Le corpus vit dans `ONTBibleApp/dist/`, produit par le pipeline. La règle du
//! projet est qu'il n'en existe **qu'une** copie : le dupliquer ici créerait
//! une seconde source de vérité que personne ne penserait à mettre à jour.
//!
//! `include_str!` ne duplique rien — il pointe le fichier voisin et en colle le
//! contenu dans le binaire au moment de la compilation. Le serveur qui tourne
//! n'a donc aucun dossier de données à côté de lui, et ne peut pas tomber parce
//! qu'un déploiement l'aurait oublié.
//!
//! Le prix est connu et assumé : rafraîchir le corpus demande de rejouer le
//! pipeline **puis** de recompiler. Voir `build.rs`, qui dresse la liste des
//! livres tout seul.
//!
//! ## Pourquoi les livres sont analysés à la demande
//!
//! Trois livres pèsent 912 Ko ; soixante-dix en pèseront une vingtaine de méga.
//! Les analyser au démarrage se paierait sur le **démarrage à froid** de la
//! Lambda, qui est déjà de ~450 ms — et pour rien, puisqu'une visite touche un
//! livre, pas soixante-dix.
//!
//! Chaque livre a donc son [`OnceLock`] : le premier lecteur paie l'analyse,
//! les suivants lisent le résultat. Le sommaire et le lexique, eux, sont
//! analysés au démarrage : ils sont petits, et toute page en a besoin.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use serde::Deserialize;

use crate::application::ports::{Corpus, Lexique};
use crate::domaine::corpus::{
    Bloc, Chapitre, Ensemble, Entree, EntreeDeLivre, Livre, Occurrence, Section, SousTitre, Statut,
};
use crate::domaine::texte::{Noeud, Verset};

// `LIVRES: &[(&str, &str)]` — l'identifiant du livre et son JSON.
include!(concat!(env!("OUT_DIR"), "/livres.rs"));

const PLAN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../ONTBibleApp/dist/corpus.json"
));

const GLOSSAIRE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../ONTBibleApp/dist/glossary.json"
));

const OCCURRENCES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../ONTBibleApp/dist/occurrences.json"
));

// ───────────────────────────── les formes du pipeline ─────────────────────────

/// Un nœud de texte, tel que le pipeline l'écrit.
///
/// L'étiquette est **dans** l'objet — `{"t":"term", …}` — d'où
/// `#[serde(tag = "t")]`.
#[derive(Debug, Deserialize)]
#[serde(tag = "t", rename_all = "lowercase")]
enum NoeudDto {
    Text {
        v: String,
    },
    Term {
        v: String,
        lemma: String,
    },
    Important {
        children: Vec<NoeudDto>,
    },
    Gloss {
        children: Vec<NoeudDto>,
    },
    Em {
        children: Vec<NoeudDto>,
    },
    Translit {
        translit: String,
        hebrew: String,
    },
    Heb {
        v: String,
    },
    Link {
        href: String,
        children: Vec<NoeudDto>,
    },
    Break,
    /// Un type que ce site ne connaît pas encore.
    ///
    /// Sans cette porte de sortie, un nouveau type de nœud ajouté au pipeline
    /// ferait échouer l'analyse du livre entier, et la page rendrait une
    /// erreur. Ici il est simplement omis : le reste du chapitre se lit.
    ///
    /// Et pour que cette tolérance ne devienne pas un trou silencieux, un test
    /// plus bas analyse **tout** le corpus et exige qu'aucun nœud n'y tombe.
    /// La bascule se voit donc à la compilation des tests, jamais en
    /// production.
    #[serde(other)]
    Inconnu,
}

#[derive(Debug, Deserialize)]
struct VersetDto {
    n: u32,
    nodes: Vec<NoeudDto>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "t", rename_all = "lowercase")]
enum BlocDto {
    Verses {
        verses: Vec<VersetDto>,
    },
    Heading {
        level: u8,
        nodes: Vec<NoeudDto>,
    },
    List {
        ordered: bool,
        items: Vec<Vec<NoeudDto>>,
    },
    Para {
        nodes: Vec<NoeudDto>,
    },
    Quote {
        nodes: Vec<NoeudDto>,
    },
    Table {
        headers: Vec<Vec<NoeudDto>>,
        rows: Vec<Vec<Vec<NoeudDto>>>,
    },
    Rule,
    /// Même raison, même garde-fou que [`NoeudDto::Inconnu`].
    #[serde(other)]
    Inconnu,
}

#[derive(Debug, Deserialize)]
struct SousTitreDto {
    french: String,
    hebrew: String,
    reference: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PiedDto {
    #[serde(default)]
    notes: Vec<BlocDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChapitreDto {
    id: String,
    book_id: String,
    n: u32,
    title: String,
    subtitle: Option<SousTitreDto>,
    status: String,
    blocks: Vec<BlocDto>,
    footer: Option<PiedDto>,
    verse_count: u32,
}

#[derive(Debug, Deserialize)]
struct LivreDto {
    id: String,
    title: String,
    // Mêmes `null` que sur l'entrée de sommaire, et la correction y avait été
    // faite sans l'être ici. Un livre sans nom hébreu est légitime — le vault
    // en porte — et le pipeline écrit `null`, pas l'absence.
    french: Option<String>,
    hebrew: Option<String>,
    chapters: Vec<ChapitreDto>,
    intro: Option<ChapitreDto>,
}

#[derive(Debug, Deserialize)]
struct PlanDto {
    corpora: Vec<EnsembleDto>,
}

#[derive(Debug, Deserialize)]
struct EnsembleDto {
    id: String,
    title: String,
    modes: Vec<SectionDto>,
}

#[derive(Debug, Deserialize)]
struct SectionDto {
    id: String,
    title: String,
    books: Vec<EntreeDeLivreDto>,
}

#[derive(Debug, Deserialize)]
struct EntreeDeLivreDto {
    id: String,
    title: String,
    // Le pipeline écrit `null` — et non l'absence — quand le vault ne porte pas
    // le champ. Un `String` avec `#[serde(default)]` ne suffirait pas : il
    // couvre la clé manquante, pas la clé nulle.
    french: Option<String>,
    hebrew: Option<String>,
    empty: bool,
    #[serde(default)]
    chapters: Vec<PlanChapitreDto>,
}

#[derive(Debug, Deserialize)]
struct PlanChapitreDto {}

#[derive(Debug, Deserialize)]
struct GlossaireDto {
    entries: Vec<EntreeDto>,
}

#[derive(Debug, Deserialize)]
struct EntreeDto {
    lemma: String,
    title: String,
    hebrew: Option<String>,
    rendering: Option<String>,
    forms: Option<Vec<String>>,
    definition: Option<Vec<BlocDto>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OccurrencesDto {
    by_lemma: HashMap<String, Vec<OccurrenceDto>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OccurrenceDto {
    book_id: String,
    chapter_id: String,
    verse: Option<u32>,
    form: String,
    snippet: String,
}

// ───────────────────────────── la traduction ──────────────────────────────────

fn noeuds(dtos: Vec<NoeudDto>) -> Vec<Noeud> {
    dtos.into_iter().filter_map(noeud).collect()
}

fn noeud(dto: NoeudDto) -> Option<Noeud> {
    Some(match dto {
        NoeudDto::Text { v } => Noeud::Texte(v),
        NoeudDto::Term { v, lemma } => Noeud::Intraduisible { mot: v, lemme: lemma },
        NoeudDto::Important { children } => Noeud::Important(noeuds(children)),
        NoeudDto::Gloss { children } => Noeud::Glose(noeuds(children)),
        NoeudDto::Em { children } => Noeud::Emphase(noeuds(children)),
        NoeudDto::Translit { translit, hebrew } => Noeud::Hebreu {
            translitteration: translit,
            hebreu: hebrew,
        },
        NoeudDto::Heb { v } => Noeud::HebreuNu(v),
        NoeudDto::Link { href, children } => Noeud::Lien {
            href,
            enfants: noeuds(children),
        },
        NoeudDto::Break => Noeud::Saut,
        NoeudDto::Inconnu => return None,
    })
}

fn blocs(dtos: Vec<BlocDto>) -> Vec<Bloc> {
    dtos.into_iter().filter_map(bloc).collect()
}

fn bloc(dto: BlocDto) -> Option<Bloc> {
    Some(match dto {
        BlocDto::Verses { verses } => Bloc::Versets(
            verses
                .into_iter()
                .map(|v| Verset {
                    numero: v.n,
                    noeuds: noeuds(v.nodes),
                })
                .collect(),
        ),
        BlocDto::Heading { level, nodes } => Bloc::Titre {
            niveau: level,
            noeuds: noeuds(nodes),
        },
        BlocDto::List { ordered, items } => Bloc::Liste {
            ordonnee: ordered,
            items: items.into_iter().map(noeuds).collect(),
        },
        BlocDto::Para { nodes } => Bloc::Paragraphe(noeuds(nodes)),
        BlocDto::Quote { nodes } => Bloc::Citation(noeuds(nodes)),
        BlocDto::Table { headers, rows } => Bloc::Tableau {
            entetes: headers.into_iter().map(noeuds).collect(),
            lignes: rows
                .into_iter()
                .map(|ligne| ligne.into_iter().map(noeuds).collect())
                .collect(),
        },
        BlocDto::Rule => Bloc::Filet,
        BlocDto::Inconnu => return None,
    })
}

fn chapitre(dto: ChapitreDto) -> Chapitre {
    Chapitre {
        id: dto.id,
        livre: dto.book_id,
        numero: dto.n,
        titre: dto.title,
        sous_titre: dto.subtitle.map(|s| SousTitre {
            francais: s.french,
            hebreu: s.hebrew,
            reference: s.reference,
        }),
        // Le pipeline n'écrit que « locked » et « brouillon ». Tout ce qui
        // n'est pas explicitement verrouillé est traité comme provisoire :
        // c'est le sens qui protège le lecteur, pas l'inverse.
        statut: if dto.status == "locked" {
            Statut::Acheve
        } else {
            Statut::Brouillon
        },
        blocs: blocs(dto.blocks),
        notes: dto.footer.map(|p| blocs(p.notes)).unwrap_or_default(),
        nombre_de_versets: dto.verse_count,
    }
}

fn livre(dto: LivreDto) -> Livre {
    Livre {
        id: dto.id,
        titre: dto.title,
        francais: dto.french.unwrap_or_default(),
        hebreu: dto.hebrew.unwrap_or_default(),
        intro: dto.intro.map(chapitre),
        chapitres: dto.chapters.into_iter().map(chapitre).collect(),
    }
}

fn entree(dto: EntreeDto) -> Entree {
    Entree {
        lemme: dto.lemma,
        titre: dto.title,
        hebreu: dto.hebrew.unwrap_or_default(),
        rendu: dto.rendering.unwrap_or_default(),
        formes: dto.forms.unwrap_or_default(),
        definition: blocs(dto.definition.unwrap_or_default()),
    }
}

// ───────────────────────────── les réalisations ───────────────────────────────

/// Le corpus embarqué.
pub struct CorpusEmbarque {
    sommaire: Vec<Ensemble>,
    /// Le JSON de chaque livre, et son analyse une fois faite.
    livres: HashMap<&'static str, (&'static str, OnceLock<Option<Arc<Livre>>>)>,
}

impl CorpusEmbarque {
    pub fn charger() -> Result<Self, serde_json::Error> {
        let plan: PlanDto = serde_json::from_str(PLAN)?;

        let sommaire = plan
            .corpora
            .into_iter()
            .map(|e| Ensemble {
                id: e.id,
                titre: e.title,
                sections: e
                    .modes
                    .into_iter()
                    .map(|s| Section {
                        id: s.id,
                        titre: s.title,
                        livres: s
                            .books
                            .into_iter()
                            .map(|l| EntreeDeLivre {
                                id: l.id,
                                titre: l.title,
                                francais: l.french.unwrap_or_default(),
                                hebreu: l.hebrew.unwrap_or_default(),
                                ecrit: !l.empty,
                                unites: l.chapters.len() as u32,
                            })
                            .collect(),
                    })
                    .collect(),
            })
            .collect();

        let livres = LIVRES
            .iter()
            .map(|(id, source)| (*id, (*source, OnceLock::new())))
            .collect();

        Ok(Self { sommaire, livres })
    }
}

impl Corpus for CorpusEmbarque {
    fn sommaire(&self) -> &[Ensemble] {
        &self.sommaire
    }

    fn livre(&self, id: &str) -> Option<Arc<Livre>> {
        let (source, cache) = self.livres.get(id)?;
        // L'analyse peut échouer si le pipeline change de forme. Elle rend
        // alors `None`, et la page répond « introuvable » — au lieu de tomber
        // et d'emporter la requête. Le test `tout_le_corpus_s_analyse` interdit
        // que ça arrive sans qu'on le sache.
        cache
            .get_or_init(|| {
                serde_json::from_str::<LivreDto>(source)
                    .ok()
                    .map(|dto| Arc::new(livre(dto)))
            })
            .clone()
    }
}

/// Le lexique embarqué.
pub struct LexiqueEmbarque {
    entrees: Vec<Entree>,
    /// Le lemme vers son rang dans `entrees` — une recherche de fiche est un
    /// accès direct et non un parcours des 105 entrées.
    index: HashMap<String, usize>,
    occurrences: OnceLock<HashMap<String, Vec<Occurrence>>>,
}

impl LexiqueEmbarque {
    pub fn charger() -> Result<Self, serde_json::Error> {
        let glossaire: GlossaireDto = serde_json::from_str(GLOSSAIRE)?;

        let mut entrees: Vec<Entree> = glossaire.entries.into_iter().map(entree).collect();
        // L'ordre du pipeline suit le vault. Une page de lexique se lit par
        // ordre alphabétique du lemme — et c'est le lemme, pas le titre, qui
        // fait foi : c'est lui qui est dans l'adresse.
        entrees.sort_by(|a, b| a.lemme.cmp(&b.lemme));

        let index = entrees
            .iter()
            .enumerate()
            .map(|(rang, e)| (e.lemme.clone(), rang))
            .collect();

        Ok(Self {
            entrees,
            index,
            occurrences: OnceLock::new(),
        })
    }
}

impl Lexique for LexiqueEmbarque {
    fn entrees(&self) -> &[Entree] {
        &self.entrees
    }

    fn entree(&self, lemme: &str) -> Option<&Entree> {
        self.index.get(lemme).map(|rang| &self.entrees[*rang])
    }

    fn occurrences(&self, lemme: &str) -> Vec<Occurrence> {
        self.occurrences
            .get_or_init(|| {
                serde_json::from_str::<OccurrencesDto>(OCCURRENCES)
                    .map(|dto| {
                        dto.by_lemma
                            .into_iter()
                            .map(|(lemme, liste)| {
                                let liste = liste
                                    .into_iter()
                                    .map(|o| Occurrence {
                                        livre: o.book_id,
                                        chapitre: o.chapter_id,
                                        verset: o.verse,
                                        forme: o.form,
                                        extrait: o.snippet,
                                    })
                                    .collect();
                                (lemme, liste)
                            })
                            .collect()
                    })
                    .unwrap_or_default()
            })
            .get(lemme)
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Le corpus réel s'analyse en entier, et **aucun** nœud ni bloc ne tombe
    /// dans la porte de sortie.
    ///
    /// C'est le test qui rend la tolérance de [`NoeudDto::Inconnu`] acceptable.
    /// Sans lui, un nouveau type de bloc ajouté au pipeline disparaîtrait des
    /// pages sans un mot : le texte serait simplement plus court, et personne
    /// ne compare la longueur d'un chapitre d'une semaine à l'autre.
    ///
    /// On analyse ici en `Value` plutôt qu'en DTO, justement parce que le DTO
    /// est ce qu'on éprouve : il faut lire les étiquettes brutes.
    #[test]
    fn tout_le_corpus_s_analyse_sans_type_inconnu() {
        const NOEUDS: &[&str] = &[
            "text",
            "term",
            "important",
            "gloss",
            "em",
            "translit",
            "heb",
            "link",
            "break",
        ];
        const BLOCS: &[&str] = &["verses", "heading", "list", "para", "quote", "table", "rule"];

        fn etiquettes(valeur: &serde_json::Value, vues: &mut std::collections::BTreeSet<String>) {
            match valeur {
                serde_json::Value::Object(objet) => {
                    if let Some(serde_json::Value::String(t)) = objet.get("t") {
                        vues.insert(t.clone());
                    }
                    for v in objet.values() {
                        etiquettes(v, vues);
                    }
                }
                serde_json::Value::Array(liste) => {
                    for v in liste {
                        etiquettes(v, vues);
                    }
                }
                _ => {}
            }
        }

        let mut vues = std::collections::BTreeSet::new();
        for (_, source) in LIVRES {
            let valeur: serde_json::Value =
                serde_json::from_str(source).expect("un livre du pipeline est du JSON");
            etiquettes(&valeur, &mut vues);
        }
        let valeur: serde_json::Value = serde_json::from_str(GLOSSAIRE).unwrap();
        etiquettes(&valeur, &mut vues);

        let orphelines: Vec<&String> = vues
            .iter()
            .filter(|t| !NOEUDS.contains(&t.as_str()) && !BLOCS.contains(&t.as_str()))
            .collect();

        assert!(
            orphelines.is_empty(),
            "le pipeline produit des types que le site ignore, et qu'il jetterait \
             en silence : {orphelines:?} — les ajouter à `NoeudDto` ou `BlocDto`"
        );
    }

    /// Chaque livre listé par `build.rs` s'analyse réellement.
    ///
    /// `livre()` rend `None` sur échec plutôt que de tomber : sans ce test, un
    /// livre devenu illisible se manifesterait par un 404, ce qui ressemble
    /// beaucoup trop à « ce livre n'est pas encore écrit ».
    #[test]
    fn chaque_livre_embarque_s_analyse() {
        for (id, source) in LIVRES {
            // On analyse ici en direct plutôt que par `livre()` : celui-ci
            // avale l'erreur pour ne pas tomber en production, et un test qui
            // dit « ça ne marche pas » sans dire pourquoi coûte une heure.
            if let Err(erreur) = serde_json::from_str::<LivreDto>(source) {
                panic!("le livre « {id} » ne s'analyse pas : {erreur}");
            }
        }

        let corpus = CorpusEmbarque::charger().expect("corpus.json");
        for (id, _) in LIVRES {
            assert!(corpus.livre(id).is_some());
        }
    }

    /// Le sommaire porte les 70 livres, pas seulement les trois écrits.
    #[test]
    fn le_sommaire_porte_tout_le_plan() {
        let corpus = CorpusEmbarque::charger().unwrap();
        let total: usize = corpus
            .sommaire()
            .iter()
            .flat_map(|e| e.sections.iter())
            .map(|s| s.livres.len())
            .sum();
        assert_eq!(total, 70, "le plan du corpus a changé de taille");

        let ecrits: Vec<&str> = corpus
            .sommaire()
            .iter()
            .flat_map(|e| e.livres_ecrits())
            .map(|l| l.id.as_str())
            .collect();
        // Ce que le sommaire dit d'écrit doit être ce qui est embarqué. Une
        // divergence donnerait un lien de sommaire vers un livre absent.
        let embarques: Vec<&str> = LIVRES.iter().map(|(id, _)| *id).collect();
        for id in &ecrits {
            assert!(
                embarques.contains(id),
                "le sommaire annonce « {id} » comme écrit, mais aucun fichier ne l'accompagne"
            );
        }
    }

    /// Le fichier d'occurrences s'analyse.
    ///
    /// `occurrences()` rend une liste vide quand il échoue — c'est le bon
    /// comportement en production, où une fiche sans renvois vaut mieux qu'une
    /// page en erreur. Mais « vide » et « illisible » se ressemblent trop pour
    /// qu'un test s'en contente.
    #[test]
    fn les_occurrences_s_analysent() {
        serde_json::from_str::<OccurrencesDto>(OCCURRENCES).expect("occurrences.json");
    }

    /// Une fiche se trouve, et ses occurrences pointent des versets réels.
    #[test]
    fn une_fiche_porte_ses_occurrences() {
        let lexique = LexiqueEmbarque::charger().expect("glossary.json");
        let corpus = CorpusEmbarque::charger().unwrap();

        let elohim = lexique.entree("elohim").expect("la fiche « elohim »");
        assert!(!elohim.definition.is_empty());

        let occurrences = lexique.occurrences("elohim");
        assert!(!occurrences.is_empty(), "« elohim » paraît dans le corpus");

        // Le lien d'une occurrence doit mener quelque part. C'est exactement le
        // défaut qu'on répare : un or qui promet une fiche et n'y mène pas.
        //
        // Toutes les occurrences, pas seulement la première : une seule
        // référence morte suffit à produire une page vide, et rien ne la
        // signalerait.
        for occurrence in &occurrences {
            let livre = corpus
                .livre(&occurrence.livre)
                .unwrap_or_else(|| panic!("livre « {} » absent", occurrence.livre));
            let chapitre = livre
                .chapitre(&occurrence.chapitre)
                .unwrap_or_else(|| panic!("chapitre « {} » absent", occurrence.chapitre));
            if let Some(numero) = occurrence.verset {
                assert!(
                    chapitre.verset(numero).is_some(),
                    "l'occurrence cite {} v.{numero}, absent du chapitre",
                    occurrence.chapitre
                );
            }
        }
    }

    /// Le lexique est trié par lemme — c'est l'ordre de la page d'index.
    #[test]
    fn le_lexique_est_alphabetique() {
        let lexique = LexiqueEmbarque::charger().unwrap();
        let lemmes: Vec<&str> = lexique.entrees().iter().map(|e| e.lemme.as_str()).collect();
        let mut trie = lemmes.clone();
        trie.sort();
        assert_eq!(lemmes, trie);
    }
}
