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

use crate::application::ports::{Corpus, Lexique};
use crate::domaine::corpus::{
    Bloc, Chapitre, Conteneur, Ensemble, Entree, EntreeDeLivre, Livre, Occurrence, Section,
    SousTitre, Statut,
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

// ───────────────────────────── la traduction ──────────────────────────────────
//
// Les formes du pipeline **ne sont plus décrites ici**. Elles viennent de
// `ont::schema`, la caisse du pipeline, qui est désormais l'unique description
// du contrat côté Rust.
//
// Ce qu'il y avait avant : 450 lignes de DTO qui redisaient, champ par champ,
// ce que le pipeline écrit déjà. Une troisième description après le TypeScript
// et le Swift — et trois descriptions du même contrat finissent par diverger.
//
// Quand elles divergeaient, le défaut était **muet**. Un type de nœud absent du
// relevé tombait dans une porte de sortie `#[serde(other)]`, la page
// s'affichait, et il manquait un mot. C'est arrivé sur `heb`, `link`, `quote`
// et `table`, qui ne vivent que dans les définitions du lexique et jamais dans
// un chapitre : le premier relevé n'avait parcouru que les chapitres.
//
// Cette porte de sortie n'existe plus, et c'est le vrai gain. Un type ajouté au
// pipeline **casse la compilation du site** — le `match` ci-dessous n'est plus
// exhaustif. Le garde-fou n'est plus un test qu'il faut penser à écrire ; c'est
// le compilateur.
//
// Le prix, honnêtement : un `dist/` périmé — produit par un pipeline plus
// récent que le site qu'on compile — ne s'analyse plus « en partie », il
// échoue. C'est le comportement voulu. Les deux naissent du même commit dans la
// CI, et en local `cargo` recompile dès que `dist/` bouge.

use ont::schema as pipeline;

fn noeuds(sources: Vec<pipeline::Inline>) -> Vec<Noeud> {
    sources.into_iter().map(noeud).collect()
}

fn noeud(source: pipeline::Inline) -> Noeud {
    match source {
        pipeline::Inline::Text { v } => Noeud::Texte(v),
        pipeline::Inline::Term { v, lemma } => Noeud::Intraduisible {
            mot: v,
            lemme: lemma,
        },
        pipeline::Inline::Accentuation { children } => Noeud::Accentuation(noeuds(children)),
        pipeline::Inline::Gloss { children } => Noeud::Glose(noeuds(children)),
        pipeline::Inline::Em { children } => Noeud::Emphase(noeuds(children)),
        pipeline::Inline::Translit { translit, hebrew } => Noeud::Hebreu {
            translitteration: translit,
            hebreu: hebrew,
        },
        pipeline::Inline::Heb { v } => Noeud::HebreuNu(v),
        pipeline::Inline::Link { href, children } => Noeud::Lien {
            href,
            enfants: noeuds(children),
        },
        pipeline::Inline::Break => Noeud::Saut,
    }
}

fn blocs(sources: Vec<pipeline::Block>) -> Vec<Bloc> {
    sources.into_iter().map(bloc).collect()
}

fn bloc(source: pipeline::Block) -> Bloc {
    match source {
        pipeline::Block::Verses { verses } => Bloc::Versets(
            verses
                .into_iter()
                .map(|v| Verset {
                    numero: v.n,
                    noeuds: noeuds(v.nodes),
                })
                .collect(),
        ),
        pipeline::Block::Heading { level, nodes } => Bloc::Titre {
            niveau: level,
            noeuds: noeuds(nodes),
        },
        pipeline::Block::List { ordered, items } => Bloc::Liste {
            ordonnee: ordered,
            items: items.into_iter().map(noeuds).collect(),
        },
        pipeline::Block::Para { nodes } => Bloc::Paragraphe(noeuds(nodes)),
        pipeline::Block::Quote { nodes } => Bloc::Citation(noeuds(nodes)),
        pipeline::Block::Table { headers, rows } => Bloc::Tableau {
            entetes: headers.into_iter().map(noeuds).collect(),
            lignes: rows
                .into_iter()
                .map(|ligne| ligne.into_iter().map(noeuds).collect())
                .collect(),
        },
        pipeline::Block::Rule => Bloc::Filet,
    }
}

fn chapitre(source: pipeline::Chapter) -> Chapitre {
    Chapitre {
        id: source.id,
        livre: source.book_id,
        numero: source.n,
        titre: source.title,
        sous_titre: source.subtitle.map(|s| SousTitre {
            francais: s.french,
            hebreu: s.hebrew,
            reference: s.reference,
        }),
        // Deux états, et le contrat le dit maintenant en type plutôt qu'en
        // chaîne. La comparaison à `"locked"` qu'il y avait ici tenait sur une
        // orthographe : le jour où le pipeline aurait écrit `"Locked"`, tout le
        // corpus serait passé en brouillon sans qu'aucun test ne bronche.
        statut: match source.status {
            pipeline::Status::Locked => Statut::Acheve,
            pipeline::Status::Brouillon => Statut::Brouillon,
        },
        blocs: blocs(source.blocks),
        notes: source.footer.map(|p| blocs(p.notes)).unwrap_or_default(),
        nombre_de_versets: source.verse_count,
    }
}

fn livre(source: pipeline::Book) -> Livre {
    Livre {
        id: source.id,
        titre: source.title,
        // `french` est un `String` dans le contrat, et non un `Option` : le
        // pipeline en garantit un pour les soixante-dix slots. C'était un
        // `Option` ici, par prudence — une prudence que le contrat rend
        // inutile, puisqu'il l'affirme. `hebrew`, lui, manque réellement sur
        // dix-huit livres.
        francais: source.french,
        hebreu: source.hebrew.unwrap_or_default(),
        intro: source.intro.map(chapitre),
        chapitres: source.chapters.into_iter().map(chapitre).collect(),
    }
}

fn entree(source: pipeline::GlossaryEntry) -> Entree {
    Entree {
        lemme: source.lemma,
        titre: source.title,
        hebreu: source.hebrew.unwrap_or_default(),
        rendu: source.rendering.unwrap_or_default(),
        formes: source.forms,
        definition: blocs(source.definition.unwrap_or_default()),
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
        let plan: pipeline::CorpusFile = serde_json::from_str(PLAN)?;

        let sommaire = plan
            .corpora
            .into_iter()
            .map(|e| Ensemble {
                id: e.id,
                // Le pont vers le français est **facultatif** dans le schéma, et
                // il le faut : le corpus publié atteint des liseuses plus
                // anciennes que lui, qui n'ont pas cette clé.
                //
                // Le site, lui, embarque le corpus à la compilation — il ne
                // lira jamais un corpus d'hier. Mais il partage la structure du
                // pipeline, donc il doit décider quoi faire de l'absence.
                //
                // Il retombe sur le titre ONT plutôt que sur rien : c'est la
                // règle déjà suivie deux lignes plus bas pour la glose, et un
                // sommaire qui affiche une ligne vide se lit comme une panne.
                francais: e.french.unwrap_or_else(|| e.title.clone()),
                titre: e.title,
                glose: e.glose,
                sections: e
                    .modes
                    .into_iter()
                    .map(|s| Section {
                        id: s.id,
                        francais: s.french.unwrap_or_else(|| s.title.clone()),
                        titre: s.title,
                        glose: s.glose,
                        conteneurs: s
                            .groups
                            .into_iter()
                            .map(|g| Conteneur {
                                id: g.id,
                                titre: g.title,
                                francais: g.french,
                                glose: g.glose,
                                rupture: g.rupture,
                            })
                            .collect(),
                        livres: s
                            .books
                            .into_iter()
                            .map(|l| EntreeDeLivre {
                                id: l.id,
                                titre: l.title,
                                francais: l.french,
                                hebreu: l.hebrew.unwrap_or_default(),
                                ecrit: !l.empty,
                                unites: l.chapters.len() as u32,
                                conteneur: l.group_id,
                                glose: l.glose,
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
                serde_json::from_str::<pipeline::Book>(source)
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
        let glossaire: pipeline::GlossaryFile = serde_json::from_str(GLOSSAIRE)?;

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
                serde_json::from_str::<pipeline::OccurrencesFile>(OCCURRENCES)
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

    // Le test `tout_le_corpus_s_analyse_sans_type_inconnu` vivait ici.
    //
    // Il parcourait tout le corpus en JSON brut et vérifiait qu'aucune
    // étiquette `t` n'échappait à deux listes écrites à la main. Il rendait
    // acceptable la porte de sortie `#[serde(other)]` des anciens DTO : sans
    // lui, un type de bloc ajouté au pipeline disparaissait des pages sans un
    // mot — le texte était simplement plus court, et personne ne compare la
    // longueur d'un chapitre d'une semaine à l'autre.
    //
    // Les deux ont disparu ensemble. Le site lit maintenant `ont::schema`,
    // donc un type inconnu n'est plus omis : le `match` de `noeud` cesse
    // d'être exhaustif et **la compilation échoue**. Et ces deux listes
    // d'étiquettes étaient elles-mêmes une quatrième description du contrat,
    // à tenir à jour à la main — exactement ce qu'on venait de supprimer.

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
            if let Err(erreur) = serde_json::from_str::<pipeline::Book>(source) {
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
        serde_json::from_str::<pipeline::OccurrencesFile>(OCCURRENCES).expect("occurrences.json");
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
