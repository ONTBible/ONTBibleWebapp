use leptos::prelude::*;

use crate::interface::design::Bloc;

/// Le gabarit des pages légales.
///
/// Confidentialité et conditions ont la même forme et n'ont pas à la reposer
/// chacune : un titre, une date de mise à jour, puis du texte suivi. La date
/// est en évidence parce que c'est elle qui donne sa valeur au document — une
/// politique sans date ne dit pas ce qui s'applique.
///
/// Les styles du texte suivi sont posés ici, sur les descendants : une page
/// légale est écrite en balises ordinaires — `h2`, `p`, `ul` — et il serait
/// absurde d'exiger un composant pour chaque paragraphe d'un texte juridique.
#[component]
pub fn PageLegale(
    #[prop(into)] titre: String,
    /// La date de dernière mise à jour, écrite en toutes lettres.
    #[prop(into)]
    mise_a_jour: String,
    children: Children,
) -> impl IntoView {
    view! {
        <Bloc>
            <a
                href="/fr"
                class="text-sm uppercase tracking-capitales text-encre-douce no-underline hover:text-encre"
            >
                "← Retour"
            </a>

            <h1 class="mt-6">{titre}</h1>
            <p class="mb-12 text-sm italic text-encre-douce">
                "Dernière mise à jour : " {mise_a_jour}
            </p>

            <div class="[&_a]:text-accent [&_li]:mb-2 [&_ul]:mb-6 [&_ul]:list-disc [&_ul]:ps-6">
                {children()}
            </div>
        </Bloc>
    }
}
