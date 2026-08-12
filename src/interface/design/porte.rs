use leptos::prelude::*;
use leptos_router::components::A;

/// Une entrée du projet — un titre, et ce qu'on trouve derrière.
///
/// Une porte, pas une carte : une carte est un objet qu'on soulève, une porte
/// est un passage. D'où le fond du parchemin et le filet, plutôt qu'une ombre.
#[component]
pub fn Porte(
    #[prop(into)] href: String,
    #[prop(into)] titre: String,
    #[prop(into)] glose: String,
) -> impl IntoView {
    view! {
        <A href=href attr:class="group block bg-parchemin px-2 py-6 no-underline">
            <span class="block font-titre text-lg transition-colors group-hover:text-or-profond">
                {titre}
            </span>
            <span class="mt-1 block text-sm text-encre-douce">{glose}</span>
        </A>
    }
}
