use leptos::prelude::*;

/// Une note à l'échelle de la page.
///
/// Ce qu'il faut savoir sans que ça interrompe la lecture : l'état du corpus,
/// une réserve, une date de mise à jour.
#[component]
pub fn Mention(children: Children) -> impl IntoView {
    view! {
        <p class="mt-16 text-center text-sm text-encre-douce">{children()}</p>
    }
}
