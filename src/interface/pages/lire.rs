use leptos::prelude::*;

use crate::api::sommaire;
use crate::interface::design::{fournir_preferences, PageDeLecture, Sommaire};
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
    // Le sommaire nomme les unités selon le registre choisi — « Chapitre 3 »
    // ou « Parashah 3 » —, donc il **lit** les préférences. Sans ce
    // fournisseur, `preferences()` retombait sur un signal constant : le
    // réglage n'avait aucun effet sur cette page, et rien ne le disait.
    //
    // La correction du 25 août avait couvert `livre.rs` et `fiche.rs` en
    // relevant qui appelait `preferences()` **directement**. Celle-ci consomme
    // par composant interposé : `Lire` ne cite ni `preferences` ni
    // `nom_d_unite`, c'est `Sommaire` qui les appelle. Un relevé par appel
    // direct ne pouvait pas la voir.
    //
    // C'est le `debug_assert!` posé le même jour qui l'a trouvée, en faisant
    // paniquer la page en développement. En `--release` il ne s'arme pas : la
    // page s'affichait, se lisait, et le réglage restait mort.
    let _preferences = fournir_preferences();

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
                        "Le plan entier, et ce qui en est traduit. Les titres en or se lisent ; \
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
