use leptos::prelude::*;

/// Un énoncé qui porte la thèse.
///
/// Plus grand que le corps, mais ce n'est pas un titre : ça se lit comme du
/// texte et ça doit pouvoir enchaîner sur le paragraphe suivant.
#[component]
pub fn Principe(
    /// La chute d'un enchaînement — la phrase sur laquelle il se referme.
    #[prop(optional)]
    chute: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <p class="text-lg leading-relaxed text-pretty" class=("text-or-profond", chute)>
            {children()}
        </p>
    }
}
