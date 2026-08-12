use leptos::prelude::*;

/// Une section de page — les larges marges du livre imprimé.
///
/// Elle borne la mesure du texte plutôt que la largeur de l'écran : au-delà de
/// soixante-dix signes par ligne, l'œil perd le début de la ligne suivante.
/// C'est la raison des marges d'un livre, et elle vaut aussi à l'écran.
#[component]
pub fn Section(
    /// Élargit la mesure — pour ce qui n'est pas du texte courant.
    #[prop(optional)]
    large: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="section" class:section--large=large>
            {children()}
        </section>
    }
}
