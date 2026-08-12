use leptos::prelude::*;

/// Une suite d'affirmations, séparées par des filets.
///
/// Pas de puces : une puce fait une liste de courses. Ces lignes ne sont pas
/// des éléments d'un inventaire, ce sont des phrases qui se tiennent seules —
/// le filet les sépare sans les subordonner.
#[component]
pub fn ListeAffirmations(lignes: Vec<&'static str>) -> impl IntoView {
    view! {
        <ul class="m-0 list-none p-0">
            {lignes
                .into_iter()
                .map(|ligne| {
                    view! {
                        <li class="border-t border-filet py-4 text-lg leading-snug first:border-t-0">
                            {ligne}
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}
