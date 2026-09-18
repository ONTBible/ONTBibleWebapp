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
use crate::domaine::texte::{
    CibleDeLaReference, CibleDuNiveauTrois, Noeud, PorteeDeLaReference, Verset,
};

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

/// Les fiches des **Shemot** — les noms propres hébreux.
///
/// Un fichier à part du glossaire, et ce n'est pas un détail d'organisation :
/// un Shem n'a ni forme fléchie, ni rendu français, ni hébreu à citer, puisque
/// c'est **le nom lui-même** qui est l'hébreu. Le pipeline les émet donc dans
/// leur propre fichier, avec trois champs au lieu de treize.
const SHEMOT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../ONTBibleApp/dist/shemot.json"
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
        // Un Shem porte la même forme qu'un `Term` — un mot et le lemme de sa
        // fiche — et s'en distingue par ce qu'il promet : un nom propre, non un
        // intraduisible. Le rendu les sépare par la couleur, pas par la forme.
        pipeline::Inline::Shem { v, lemma } => Noeud::Shem {
            mot: v,
            lemme: lemma,
        },
        pipeline::Inline::Accentuation { children } => Noeud::Accentuation(noeuds(children)),
        pipeline::Inline::Gloss { children } => Noeud::Glose(noeuds(children)),
        pipeline::Inline::Em { children } => Noeud::Emphase(noeuds(children)),
        pipeline::Inline::Translit {
            translit,
            hebrew,
            cible,
        } => Noeud::Hebreu {
            translitteration: translit,
            hebreu: hebrew,
            // Le `match` est exhaustif : une troisième destination ajoutée au
            // pipeline casserait la compilation du site au lieu de s'y perdre.
            cible: cible.map(|c| match c {
                pipeline::CibleDuNiveauTrois::Term { lemma } => CibleDuNiveauTrois::Terme(lemma),
                pipeline::CibleDuNiveauTrois::Shem { lemma } => CibleDuNiveauTrois::Shem(lemma),
            }),
        },
        pipeline::Inline::Renvoi { v, cible } => Noeud::Renvoi { libelle: v, cible },
        // Une référence biblique. `systeme` et `chapitre` ne traversent pas :
        // le libellé les porte déjà sous la forme que le lecteur lit, et la
        // navigation n'a besoin que de `cible`. Les recopier donnerait deux
        // écritures du même fait, dont l'une finirait par mentir.
        //
        // `portee`, elle, traverse — non pour servir, mais pour que son `match`
        // exhaustif fasse rougir la compilation si le pipeline en ajoute une
        // quatrième.
        pipeline::Inline::Reference {
            v,
            livre,
            portee,
            cible,
            ..
        } => Noeud::Reference {
            libelle: v,
            livre_cite: livre,
            portee: match portee {
                pipeline::PorteeDeLaReference::Chapitre => PorteeDeLaReference::Chapitre,
                pipeline::PorteeDeLaReference::Verset { n } => PorteeDeLaReference::Verset { n },
                pipeline::PorteeDeLaReference::Plage { premier, dernier } => {
                    PorteeDeLaReference::Plage { premier, dernier }
                }
            },
            cible: cible.map(|c| CibleDeLaReference {
                livre: c.livre,
                unite: c.unite,
                verset: c.verset,
            }),
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

/// Une fiche de Shem, ramenée à la forme d'une entrée de lexique.
///
/// Les trois champs manquants restent vides, et le rendu les tolère déjà : le
/// §8 bis note que `hebrew`, `rendering` et `forms` sont nuls sur plusieurs
/// fiches du glossaire lui-même. Un Shem est simplement le cas où ils le sont
/// toujours.
///
/// **Le lemme est la clé, et il est le même des deux côtés.** C'est ce qui
/// permet à `/fr/lexique/{lemme}` de servir les deux sans que le lecteur ait à
/// savoir lequel il consulte — et c'est ce que le corpus suppose déjà, puisque
/// `Noeud::Shem` et `Noeud::Intraduisible` pointent la même route.
fn entree_de_shem(source: pipeline::ShemEntry) -> Entree {
    Entree {
        lemme: source.lemma,
        titre: source.title,
        hebreu: String::new(),
        rendu: String::new(),
        formes: Vec::new(),
        est_un_nom: true,
        definition: blocs(source.definition),
    }
}

fn entree(source: pipeline::GlossaryEntry) -> Entree {
    Entree {
        lemme: source.lemma,
        titre: source.title,
        hebreu: source.hebrew.unwrap_or_default(),
        rendu: source.rendering.unwrap_or_default(),
        formes: source.forms,
        est_un_nom: false,
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
            // **Déstructuration exhaustive, et c'est une garde.**
            //
            // Le mappage se faisait par accès de champ — `e.id`, `e.title`. Un
            // champ ajouté au schéma du pipeline passait alors en silence : le
            // site ne le voyait jamais, et rien ne le disait. C'est déjà
            // arrivé, `groups` a existé en amont bien avant d'atteindre le
            // sommaire.
            //
            // Écrit ainsi, le compilateur devient le contrôle : un champ neuf
            // donne `error[E0027]: pattern does not mention field`, sur la
            // ligne du mappage, sans test à écrire ni à se rappeler d'écrire.
            //
            // Et un champ qu'on ignore **délibérément** s'écrit `_` : la
            // décision est dans le code au lieu d'être une omission. `order` en
            // est un — le pipeline range déjà les corpus dans l'ordre voulu.
            //
            // **Ne jamais mettre `..` dans ces motifs** : ça rétablit
            // exactement le silence qu'on vient de supprimer.
            .map(
                |pipeline::CorpusOutline {
                     id,
                     title,
                     french,
                     glose,
                     order: _,
                     modes,
                 }| Ensemble {
                    id,
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
                    francais: french.unwrap_or_else(|| title.clone()),
                    titre: title,
                    glose,
                    sections: modes
                        .into_iter()
                        .map(
                            |pipeline::ModeOutline {
                                 id,
                                 title,
                                 french,
                                 glose,
                                 order: _,
                                 groups,
                                 books,
                             }| Section {
                                id,
                                francais: french.unwrap_or_else(|| title.clone()),
                                titre: title,
                                glose,
                                conteneurs: groups
                                    .into_iter()
                                    .map(
                                        |pipeline::Group {
                                             id,
                                             title,
                                             french,
                                             glose,
                                             rupture,
                                         }| Conteneur {
                                            id,
                                            titre: title,
                                            francais: french,
                                            glose,
                                            rupture,
                                        },
                                    )
                                    .collect(),
                                livres: books
                                    .into_iter()
                                    .map(
                                        |pipeline::BookOutline {
                                             id,
                                             slot: _,
                                             title,
                                             french,
                                             glose,
                                             hebrew,
                                             group_id,
                                             empty,
                                             intro: _,
                                             chapters,
                                         }| EntreeDeLivre {
                                            id,
                                            titre: title,
                                            francais: french,
                                            hebreu: hebrew.unwrap_or_default(),
                                            ecrit: !empty,
                                            unites: chapters.len() as u32,
                                            conteneur: group_id,
                                            glose,
                                        },
                                    )
                                    .collect(),
                            },
                        )
                        .collect(),
                },
            )
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
        let shemot: pipeline::ShemotFile = serde_json::from_str(SHEMOT)?;

        // ── Les deux fichiers font un seul lexique ───────────────────────────
        //
        // Le site publiait `shemot.json` pour les liseuses depuis l'audit (A10)
        // et **ne le consultait pas lui-même**. Les 2 878 liens de Shem du
        // corpus menaient donc tous à « Fiche introuvable » — mesuré : la page
        // d'`eden` faisait 9 752 octets, exactement celle d'un lemme inventé.
        //
        // Le commentaire d'`Absente` disait pourtant qu'on ne pouvait pas y
        // arriver, « les liens d'or venant du même pipeline que le lexique ».
        // Vrai des intraduisibles, faux des Shemot — et c'est la phrase qui a
        // fermé la question pendant dix jours.
        // ── Une fiche peut sortir par les deux portes ───────────────────────
        //
        // Ce ne sont pas deux fichiers, mais **deux emplois** de la même fiche
        // dans le corpus, et le pipeline les compte séparément :
        //
        //     glossary.json   les intraduisibles déclarés au §2.5, balisés `**gras**`
        //     shemot.json     les lemmes cibles d'un `[[lien]]` du corpus
        //
        // `moreh` est les deux : le §2.5 le déclare — « celui qui pointe du doigt
        // la direction » — et *Bereshit* 12 écrit « jusqu'au chêne de [[Moreh]] ».
        // Le concept et le lieu, ce que la fiche affirme elle-même : « ce n'est
        // pas une collision de graphies : le lieu porte le nom du concept ».
        //
        // **Ce n'est donc pas un défaut à réparer, c'est un fait à fusionner.**
        // Les deux entrées portent la même définition, au même octet — on n'en
        // garde qu'une, et `chaque_lemme_n_a_qu_une_fiche` refuse le jour où
        // elles différeraient, parce qu'alors ce serait deux fiches rivales pour
        // une seule adresse.
        //
        // **Le glossaire l'emporte**, et ce n'est pas un choix : c'est celui du
        // pipeline, `niveau_trois.rs::le_glossaire_passe_avant_les_shemot`. Son
        // commentaire dit pourquoi l'ordre doit être fixé — « sans quoi la même
        // entrée ouvrirait une fiche ou l'autre selon l'ordre de parcours d'un
        // `HashSet`, qui n'est pas stable ». Trancher autrement ici ferait ouvrir
        // au site une fiche que l'app n'ouvre pas, pour le même mot touché.
        let mut vus: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut entrees: Vec<Entree> = Vec::new();
        for entree in glossaire
            .entries
            .into_iter()
            .map(entree)
            .chain(shemot.entries.into_iter().map(entree_de_shem))
        {
            if vus.insert(entree.lemme.clone()) {
                entrees.push(entree);
            }
        }
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
    /// Deux entrées d'un même lemme portent la **même** fiche.
    ///
    /// ## Ce que la garde d'avant demandait, et pourquoi c'était la mauvaise
    ///
    /// Elle exigeait qu'aucun lemme ne soit servi deux fois. Elle rougissait donc
    /// sur `moreh`, et j'ai cru tenir un défaut du vault — au point d'arrêter une
    /// session voisine qui s'apprêtait à couper la fiche en deux.
    ///
    /// **Ce n'est pas un défaut.** Une fiche sort par **deux portes**, et le
    /// critère n'est ni la casse de son nom de fichier ni rien qui la concerne :
    /// c'est son **emploi dans le corpus**, relevé par la session qui tient le
    /// pipeline —
    ///
    /// ```text
    /// glossary.json   les intraduisibles déclarés au §2.5, balisés `**gras**`
    /// shemot.json     les lemmes cibles d'un `[[lien]]`   (inline.rs:455)
    /// ```
    ///
    /// `moreh` est les deux : déclaré au §2.5, et lié depuis *Bereshit* 12 —
    /// « jusqu'au chêne de [[Moreh]] ». Le concept **et** le lieu.
    ///
    /// ## Ce que cette garde demande à la place
    ///
    /// Que les deux portes mènent au **même texte**. Deux entrées identiques sont
    /// un emploi double, que `charger` fusionne ; deux entrées **différentes**
    /// seraient deux fiches rivales pour une seule adresse, et l'une deviendrait
    /// injoignable sans que rien ne dise laquelle.
    ///
    /// C'est la question que l'ancien message posait à l'envers : il *répondait*
    /// « l'une est injoignable » au lieu de **demander** si les contenus
    /// diffèrent. Une garde qui détecte bien et explique mal envoie chercher au
    /// mauvais endroit.
    ///
    /// ## Elle relit les sources, pas le lexique fusionné
    ///
    /// Après `charger`, il n'y a plus de doublon à voir — c'est tout l'objet de
    /// la fusion. Une garde posée sur le résultat mesurerait donc son propre
    /// effet, et passerait toujours.
    #[test]
    fn deux_portes_d_un_lemme_mènent_au_meme_texte() {
        let glossaire: pipeline::GlossaryFile =
            serde_json::from_str(GLOSSAIRE).expect("le glossaire s'ouvre");
        let shemot: pipeline::ShemotFile =
            serde_json::from_str(SHEMOT).expect("les Shemot s'ouvrent");

        let par_lemme: std::collections::BTreeMap<String, Entree> = glossaire
            .entries
            .into_iter()
            .map(entree)
            .map(|e| (e.lemme.clone(), e))
            .collect();

        let mut partages = 0usize;
        let mut divergents: Vec<String> = Vec::new();
        for shem in shemot.entries.into_iter().map(entree_de_shem) {
            if let Some(autre) = par_lemme.get(&shem.lemme) {
                partages += 1;
                if autre.definition != shem.definition {
                    divergents.push(shem.lemme.clone());
                }
            }
        }

        // Le témoin positif, et il est **inversé** par rapport aux autres gardes
        // de ce fichier : ici, zéro partage est l'état normal. Ce qu'on refuse de
        // laisser passer en silence, c'est un relevé qui ne lirait aucune des deux
        // sources — auquel cas il n'y aurait rien à comparer et tout paraîtrait
        // sain.
        assert!(
            !par_lemme.is_empty(),
            "aucune entrée lue au glossaire — le relevé est cassé, pas le lexique"
        );

        assert!(
            divergents.is_empty(),
            "{} lemme(s) servis par deux fiches de textes DIFFÉRENTS : {divergents:?}\n\
             Deux portes mènent au même lemme, ce qui est normal — le §2.5 le \
             déclare et le corpus le lie. Mais elles doivent mener au même texte : \
             une seule adresse pour deux contenus en rend un injoignable, et rien \
             ne dit lequel.\n\
             La réparation est à la source, pas ici : les deux fiches doivent être \
             discernables avant d'être émises.",
            divergents.len()
        );

        // Ce que la fusion a effectivement absorbé, dit à voix haute plutôt que
        // constaté en silence — c'est ce nombre qui bougera le jour où un second
        // mot sera déclaré **et** lié.
        println!("  {partages} lemme(s) employés des deux côtés, fusionnés");
    }

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
