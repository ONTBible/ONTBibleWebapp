use leptos::prelude::*;

use crate::api::lexique;
use crate::interface::design::{CarteDePrononciation, PageDeLecture};
use crate::interface::tete::Tete;

/// `/fr/lexique` — l'index des intraduisibles.
///
/// La liste de ce que la traduction a décidé de **ne pas** traduire. C'est
/// peut-être la page qui dit le mieux ce qu'est l'ONT : cent cinq mots qu'une
/// traduction ordinaire aurait rendus, et qui sont restés debout parce que les
/// rendre aurait coûté ce qu'ils portent.
///
/// Chaque entrée montre son **rendu** — « l'Être façonné du sol
/// (Bereshit 1-7) » — et non sa définition. Le rendu est la décision ; la
/// définition est ce qui la justifie, et elle a sa page.
#[component]
pub fn Lexique() -> impl IntoView {
    let entrees = Resource::new_blocking(|| (), |_| async { lexique().await });

    view! {
        <Tete
            // Le titre indexable dit ce que la page contient ; le titre
            // visible reste « Lexique », qui suffit à qui est arrivé.
            // « Lexique » seul ne répond à aucune recherche : c'est un mot
            // de sommaire, pas un mot de question.
            titre="Intraduisibles et noms propres hébreux"
            description="Les intraduisibles et les noms propres hébreux de La Bible ONT — \
                         les mots laissés debout, et pourquoi."
            chemin="/fr/lexique"
        />

        // **Ni œil-de-bœuf ni chapeau**, comme la Bible. L'app ouvre son
        // lexique sur « Lexique » et rien d'autre : une liste ne s'introduit
        // pas. Ce que le chapeau disait — « chaque mot d'or et chaque nom du
        // corpus mène ici » — se lit dans la forme : chaque ligne porte son
        // chevron.
        <PageDeLecture liste=true titre="Lexique">
            // **Avant la première fiche, et c'est tout son propos.** La
            // translittération donne les lettres, pas les sons — et rien dans
            // sa graphie n'avertit quand on se trompe. Lue après coup, cette
            // feuille ne répare rien.
            <CarteDePrononciation />
            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    match entrees.await {
                        Ok(entrees) => view! { <ParLettre entrees /> }.into_any(),
                        Err(_) => ().into_any(),
                    }
                })}
            </Suspense>
        </PageDeLecture>
    }
}

/// Les quatre segments de l'app — `LexiconModel.Scope`.
///
/// « Tout » avait été retiré chez elle, puis rendu sur demande de l'auteur, et
/// son commentaire vaut ici : *lu à sa place, entre « Vocabulaire fixé » et
/// « Shemot », il se comprend comme tout le vocabulaire, ce qui est exact.*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Segment {
    Marques,
    Fixes,
    Tout,
    Shemot,
}

impl Segment {
    fn garde(self, entree: &crate::api::ResumeDto) -> bool {
        match self {
            Segment::Marques => entree.marque,
            Segment::Fixes => !entree.marque && !entree.est_un_nom,
            Segment::Tout => !entree.est_un_nom,
            Segment::Shemot => entree.est_un_nom,
        }
    }
}

/// Le lexique groupé par initiale, comme chez l'app.
///
/// ## Ce que le groupement fait, et que la liste plate ne faisait pas
///
/// Trois cent quatre-vingts entrées à plat se parcourent en défilant. Groupées
/// par lettre, elles se **parcourent par sauts** : l'œil cherche « C », pas la
/// centième ligne. C'est ce que l'app fait, et c'est ce qui rend un lexique de
/// cette taille utilisable sans recherche.
///
/// ## Le segment par défaut n'est pas celui de l'app, et c'est une règle du site
///
/// L'app ouvre sur « Intraduisibles ». Le site ouvre sur « Tout », et le §8 bis
/// dit pourquoi :
///
/// > Le serveur rend toujours tout : c'est le rendu honnête pour qui n'a pas de
/// > JavaScript, et c'est ce qu'un moteur doit indexer.
///
/// Un filtre appliqué au rendu du serveur retirerait cinquante-cinq entrées de
/// vocabulaire fixé et tous les Shemot de la page servie. Le lecteur sans
/// JavaScript ne les verrait jamais, et aucun bouton ne les lui rendrait.
///
/// C'est le seul écart assumé avec l'app sur cet écran, et il tient à ce qui
/// les distingue : l'app est un **lecteur**, le site est aussi un **index**.
///
/// ## L'initiale se prend sur le lemme, pas sur le titre
///
/// Le titre est capitalisé — « Ahyah » —, le lemme ne l'est pas. Prendre l'un
/// ou l'autre donne la même lettre ici, mais le lemme est ce qui **ordonne**
/// la liste côté serveur : grouper sur autre chose que la clé de tri
/// produirait des sections dans le désordre dès la première divergence.
#[component]
fn ParLettre(entrees: Vec<crate::api::ResumeDto>) -> impl IntoView {
    use crate::interface::design::{EnteteDeSection, Groupe, Ligne, RailDeLettres, Segments};

    let segment = RwSignal::new(Segment::Tout);

    let sections = Signal::derive(move || {
        // **Une table ordonnée, et non des suites consécutives.**
        //
        // Le premier jet groupait les entrées voisines de même initiale. Ça
        // suppose que la liste soit triée *sur cette initiale* — et elle ne
        // l'est pas : `ʾelohim` et `ʿolam` portent un diacritique qui les
        // range après Z, alors que `initiale` les rend sous E et O.
        //
        // Le rail affichait donc « … T Y Z A E I L A E I O » — les mêmes
        // lettres deux et trois fois, chacune ouvrant une section de deux
        // entrées. Un index qui répète ses lettres n'est plus un index.
        //
        // Une `BTreeMap` range par lettre, quel que soit l'ordre d'arrivée.
        let choisi = segment.get();
        let mut par_lettre: std::collections::BTreeMap<char, Vec<crate::api::ResumeDto>> =
            std::collections::BTreeMap::new();
        for entree in entrees.iter().filter(|e| choisi.garde(e)) {
            par_lettre
                .entry(initiale(&entree.lemme))
                .or_default()
                .push(entree.clone());
        }
        par_lettre.into_iter().collect::<Vec<_>>()
    });

    view! {
        <Segments
            nom="Ce que le lexique montre"
            choisi=segment
            parts=vec![
                (Segment::Marques, "Intraduisibles"),
                (Segment::Fixes, "Vocabulaire fixé"),
                (Segment::Tout, "Tout"),
                (Segment::Shemot, "Shemot"),
            ]
        />

        // Le rail suit le segment : filtrer ne doit pas laisser des lettres qui
        // ne mènent plus nulle part.
        {move || {
            let lettres = sections.get().into_iter().map(|(l, _)| l).collect::<Vec<_>>();
            view! { <RailDeLettres lettres /> }
        }}

        {move || {
            sections
                .get()
                .into_iter()
                .map(|(lettre, entrees)| {
                    view! {
                        <div id=format!("lettre-{lettre}") class="mb-6 scroll-mt-6 last:mb-0">
                            <EnteteDeSection>{lettre.to_string()}</EnteteDeSection>
                            <Groupe>
                                {entrees
                                    .into_iter()
                                    .map(|entree| {
                                        let titre = entree.titre.clone();
                                        let hebreu = entree.hebreu.clone();
                                        let rendu = entree.rendu.clone();
                                        let nom = entree.est_un_nom;
                                        view! {
                                            <Ligne
                                                chemin=Some(
                                                    format!("/fr/lexique/{}", entree.lemme),
                                                )
                                                titre=Box::new(move || {
                                                    // La teinte suit l'**espèce**, et ce
                                                    // n'est pas une décoration : c'est le
                                                    // contrat de couleurs du corpus. L'or
                                                    // promet un intraduisible, la teinte
                                                    // du Shem promet un nom propre.
                                                    //
                                                    // C'est la seule liste du site où le
                                                    // titre garde sa couleur : ici elle
                                                    // **enseigne** au lieu de signaler
                                                    // qu'on peut toucher.
                                                    view! {
                                                        <span class=if nom {
                                                            "text-shem"
                                                        } else {
                                                            "text-accent"
                                                        }>{titre}</span>
                                                    }
                                                        .into_any()
                                                })
                                                sous_titre=Box::new(move || {
                                                    (!rendu.is_empty())
                                                        .then(|| view! { <span>{rendu}</span> })
                                                        .into_any()
                                                })
                                                // **L'hébreu est la valeur, pas une suite
                                                // du titre.** Collé au lemme il s'y
                                                // soudait — « badalבְּדַל » —, et deux
                                                // écritures de sens opposés qui se
                                                // touchent ne se lisent ni l'une ni
                                                // l'autre.
                                                valeur=Box::new(move || {
                                                    view! {
                                                        <span
                                                            aria-hidden="true"
                                                            dir="rtl"
                                                            lang="he"
                                                            class="font-hebreu text-[1.05em]"
                                                        >
                                                            {hebreu}
                                                        </span>
                                                    }
                                                        .into_any()
                                                })
                                            />
                                        }
                                    })
                                    .collect_view()}
                            </Groupe>
                        </div>
                    }
                })
                .collect_view()
        }}
    }
}

/// L'initiale d'un lemme, diacritiques écartés.
///
/// Les lemmes translittérés portent des signes que l'hébreu demande et qu'un
/// index ne connaît pas — `ʾelohim`, `ʿolam`. Les laisser produirait des
/// sections d'une seule entrée, rangées après Z.
fn initiale(lemme: &str) -> char {
    lemme
        .chars()
        .find(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .unwrap_or('—')
}
