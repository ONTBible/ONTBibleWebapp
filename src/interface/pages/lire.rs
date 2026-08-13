use leptos::prelude::*;

use crate::api::sommaire;
use crate::interface::design::{PageDeLecture, Sommaire};
use crate::interface::tete::Tete;

/// `/fr/lire` — le sommaire du corpus.
///
/// C'est la porte de la liseuse, et la première page du site où l'on ne
/// **dit** pas ce qu'est l'ONT : on le montre en donnant le plan entier. Trois
/// livres en or au milieu de soixante-sept en encre atténuée disent l'état du
/// chantier plus vite qu'une phrase, et sans se périmer — les chiffres viennent
/// du pipeline.
#[component]
pub fn Lire() -> impl IntoView {
    let plan = Resource::new_blocking(|| (), |_| async { sommaire().await });

    view! {
        <Tete
            titre="Lire"
            description="Le corpus de La Bible ONT — les soixante-dix livres du Kenesset et \
                         de la Berit Hadashah, et l'état de leur restitution."
            chemin="/fr/lire"
        />

        <PageDeLecture
            rappel="Le corpus"
            titre="Lire"
            chapeau=Box::new(|| {
                view! {
                    <p class="text-encre-douce text-pretty">
                        "Le plan entier, et ce qui en est traduit. Les titres en or se lisent ; \
                         les autres attendent leur tour."
                    </p>
                }
                    .into_any()
            })
        >
            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    match plan.await {
                        Ok(ensembles) => view! { <Sommaire ensembles /> }.into_any(),
                        // Le sommaire est analysé au démarrage du serveur : s'il
                        // manque ici, c'est le contexte qui n'a pas été fourni,
                        // pas le corpus qui serait absent. La page se tait
                        // plutôt que d'annoncer un corpus vide.
                        Err(_) => ().into_any(),
                    }
                })}
            </Suspense>
        </PageDeLecture>
    }
}
