use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::api::livre;
use crate::interface::design::{fournir_preferences, ListeDUnites, PageDeLecture};
use crate::interface::tete::Tete;

/// `/fr/lire/{livre}` — les unités d'un livre.
///
/// L'étage qui manquait entre le sommaire et le texte. Sans lui, la liseuse
/// n'aurait que deux états : le plan des soixante-dix livres, et un chapitre
/// isolé — et rien pour passer de l'un à l'autre autrement qu'en devinant une
/// adresse.
#[component]
pub fn Livre() -> impl IntoView {
    let parametres = use_params_map();
    let identifiant = move || parametres.read().get("livre").unwrap_or_default();

    let ouvrage = Resource::new_blocking(identifiant, |id| async move { livre(id).await });

    // **Les réglages sont installés ici aussi**, bien que cette page n'offre
    // pas de panneau pour les changer.
    //
    // Ce qu'elle compose lit les réglages retenus — un lecteur qui a éteint les
    // gloses les veut éteintes ici aussi. Sans ce contexte, tout ce qui lit les
    // préférences reçoit un signal *constant* : la page s'affiche, le texte est
    // juste, et le réglage n'a jamais d'effet. La panne ressemble alors
    // exactement à un fonctionnement.
    let _preferences = fournir_preferences();

    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                match ouvrage.await {
                    Ok(Some(livre)) => {
                        let description = format!(
                            "{} — {}. {} unités traduites, {} versets, dans La Bible ONT.",
                            livre.titre,
                            livre.francais,
                            livre.unites.len(),
                            livre.versets,
                        );
                        let hebreu = livre.hebreu.clone();
                        let francais = livre.francais.clone();
                        let unites = livre.unites.len();
                        let versets = livre.versets;
                        view! {
                            <Tete
                                titre=livre.titre.clone()
                                description=description
                                chemin=format!("/fr/lire/{}", livre.id)
                            />

                            <PageDeLecture
                                fil=vec![("/fr/lire".to_string(), "Lire".to_string())]
                                rappel=francais
                                titre=livre.titre.clone()
                                chapeau=Box::new(move || {
                                    view! {
                                        // Même raison que sur une fiche : en
                                        // ligne, pour que le nom hébreu reste
                                        // sous le titre au lieu de partir au
                                        // bord de l'écran.
                                        <p class="mb-4">
                                            <span
                                                dir="rtl"
                                                lang="he"
                                                class="font-hebreu text-2xl text-encre-douce"
                                            >
                                                {hebreu}
                                            </span>
                                        </p>
                                        <p class="chiffres-tableau text-sm text-encre-douce">
                                            {unites} " unités · " {versets} " versets"
                                        </p>
                                    }
                                        .into_any()
                                })
                            >
                                <ListeDUnites livre=livre.id unites=livre.unites />
                            </PageDeLecture>
                        }
                            .into_any()
                    }
                    // Un livre du plan qui n'a pas encore de texte, ou un
                    // identifiant inventé. Les deux méritent la même réponse :
                    // le sommaire ne mène jamais ici, donc on y arrive par une
                    // adresse tapée ou un lien ancien.
                    _ => view! { <Absent /> }.into_any(),
                }
            })}
        </Suspense>
    }
}

/// Ce que voit quelqu'un dont le livre n'existe pas — ou pas encore.
///
/// Elle ne dit pas « erreur » : dans un corpus dont soixante-sept livres
/// restent à traduire, « ce livre n'est pas encore là » est la réponse vraie
/// dans la grande majorité des cas.
#[component]
fn Absent() -> impl IntoView {
    view! {
        <Tete
            titre="Livre introuvable"
            description="Ce livre n'a pas encore été restitué."
            chemin="/fr/lire"
        />
        <leptos_meta::Meta name="robots" content="noindex, follow" />

        <PageDeLecture
            fil=vec![("/fr/lire".to_string(), "Lire".to_string())]
            rappel="Le corpus"
            titre="Ce livre n'est pas encore là"
        >
            <p class="text-encre-douce text-pretty">
                "Soixante-sept des soixante-dix livres attendent leur restitution. \
                 Le sommaire dit lesquels se lisent aujourd'hui."
            </p>
        </PageDeLecture>
    }
}
