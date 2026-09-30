use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::api::sommaire;
use crate::domaine::corpus::{Ensemble, Section};
use crate::interface::design::{fournir_preferences, PageDeLecture, SommaireDUnePartie};
use crate::interface::tete::Tete;

/// `/fr/webapp/partie/{id}` — les livres d'une section.
///
/// ## L'étage que le site n'avait pas
///
/// L'app navigue en **quatre temps** : la Bible, une section, un livre, une
/// unité. Le site en avait trois — il posait les soixante-dix livres d'un coup
/// sur la première page, groupés par intertitres.
///
/// La différence n'est pas de mise en page, c'est de **nature** : l'app
/// présente les sections comme des destinations, avec leur avancement en
/// regard — « Torah 1/6 », « Nevi'im 0/20 ». On choisit un rayon, puis un
/// livre. Le site demandait de balayer soixante-dix entrées pour trouver les
/// six qui ont du texte.
///
/// L'auteur a demandé « un rendu UI comme si c'était un fork ». Cet étage en
/// faisait partie, et il ne se voyait pas dans une comparaison de couleurs :
/// il fallait mettre les deux écrans côte à côte.
///
/// ## Pourquoi `partie` et pas `section`
///
/// Le segment doit être un mot qu'aucun livre ne porte, et il en portera
/// soixante-dix. Les identifiants de livres sont des translittérations de
/// l'hébreu — `bereshit`, `sefar-gibbaraya` —, donc un mot français est sûr.
/// « Section » est le terme technique du domaine ; « partie » est celui qu'un
/// lecteur emploierait, et c'est lui qui va dans une adresse partagée.
#[component]
pub fn Partie() -> impl IntoView {
    let _preferences = fournir_preferences();
    let parametres = use_params_map();
    let plan = Resource::new_blocking(|| (), |_| async { sommaire().await });

    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                let demandee = parametres.get().get("partie").unwrap_or_default();
                let Ok(ensembles) = plan.await else {
                    // Le sommaire est analysé au démarrage : s'il manque, c'est
                    // le contexte qui n'a pas été fourni, pas le corpus qui
                    // serait absent.
                    return ().into_any();
                };
                match trouver(&ensembles, &demandee) {
                    Some((ensemble, section)) => view! { <Vue ensemble section /> }.into_any(),
                    // Une partie inventée, ou un identifiant d'avant un
                    // remaniement du plan. Les deux méritent la même réponse :
                    // on y arrive par une adresse tapée ou un lien ancien,
                    // jamais par le sommaire.
                    None => view! { <Introuvable /> }.into_any(),
                }
            })}
        </Suspense>
    }
}

/// La section demandée, et l'ensemble d'où elle vient.
///
/// L'ensemble suit parce que le titre de la page en a besoin : « Torah » seul
/// ne dit pas de quel testament il s'agit, et les deux ensembles portent des
/// sections de même nom — *Nevi'im* et *Ketouvim* existent des deux côtés.
fn trouver(ensembles: &[Ensemble], id: &str) -> Option<(Ensemble, Section)> {
    ensembles.iter().find_map(|ensemble| {
        ensemble
            .sections
            .iter()
            .find(|section| section.id == id)
            .map(|section| (ensemble.clone(), section.clone()))
    })
}

#[component]
fn Vue(ensemble: Ensemble, section: Section) -> impl IntoView {
    // **Fournis ici aussi, et la garde a eu raison d'insister.**
    //
    // `Partie` les fournit déjà, et ce composant est son enfant : au moment de
    // l'exécution, le contexte est là. La garde raisonne pourtant par
    // composant — « celui-ci compose un lecteur de réglages et ne les fournit
    // pas » — et c'est la bonne règle.
    //
    // Un enfant qui s'appuie sur le contexte de son parent est juste tant que
    // ce parent l'enveloppe. Le jour où `Vue` est rendue ailleurs, le réglage
    // meurt en silence : la page s'affiche, rend `200`, et aucune bascule
    // n'agit. `fournir_preferences` est idempotente — le coût de ne pas
    // dépendre de la forme de l'arbre est nul.
    let _preferences = fournir_preferences();
    let titre = section.titre.clone();
    let francais = section.francais.clone();
    let glose = section.glose.clone();
    let nom_ensemble = ensemble.titre.clone();
    let ecrits = section.livres.iter().filter(|l| l.ecrit).count();
    let total = section.livres.len();

    view! {
        <Tete
            titre=format!("{titre} — {nom_ensemble}")
            description=format!(
                "{francais} — les {total} livres de cette partie du corpus de La Bible ONT, \
                 et l'état de leur restitution.",
            )
            chemin=crate::domaine::chemins::partie(crate::interface::arbre::arbre_maintenant(), &section.id)
        />

        <PageDeLecture
            liste=true
            fil=vec![(crate::domaine::chemins::bible(crate::interface::arbre::arbre_maintenant()), "Bible".to_string())]
            titre=titre.clone()
            chapeau=Box::new(move || {
                view! {
                    <p class="m-0 flex flex-wrap items-baseline gap-x-3 gap-y-1">
                        <span class="text-[0.95em] text-encre-douce">
                            {glose.clone().unwrap_or_else(|| francais.clone())}
                        </span>
                        <span class="chiffres-tableau text-sm text-encre-douce">
                            {ecrits} " sur " {total} " traduits"
                        </span>
                    </p>
                }
                    .into_any()
            })
        >
            <SommaireDUnePartie section />
        </PageDeLecture>
    }
}

/// Une partie qui n'existe pas.
///
/// Elle ne dit pas « erreur » : le plan du corpus se remanie, et un lien
/// partagé d'il y a six mois peut nommer une partie qui a changé de nom.
#[component]
fn Introuvable() -> impl IntoView {
    view! {
        <Tete
            titre="Partie introuvable"
            description="Cette partie du corpus n'existe pas."
            chemin=crate::domaine::chemins::bible(crate::interface::arbre::arbre_maintenant())
        />
        <leptos_meta::Meta name="robots" content="noindex, follow" />

        <PageDeLecture
            liste=true
            fil=vec![(crate::domaine::chemins::bible(crate::interface::arbre::arbre_maintenant()), "Bible".to_string())]
            titre="Cette partie n'existe pas"
        >
            <p class="text-encre-douce text-pretty">
                "Le plan du corpus se remanie : une adresse d'hier peut nommer une partie \
                 qui a changé de nom. Le sommaire dit celles d'aujourd'hui."
            </p>
        </PageDeLecture>
    }
}
