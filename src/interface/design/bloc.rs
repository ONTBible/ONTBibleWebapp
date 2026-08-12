use leptos::prelude::*;

/// Un bloc de page — la primitive de mise en page du site.
///
/// Il touche les deux bords de l'écran et rétablit la **mesure** à
/// l'intérieur : le fond respire, le texte reste lisible. C'est ce qui donne
/// au site son rythme — une suite de bandes qui alternent, plutôt qu'une
/// colonne unique posée au milieu du vide.
///
/// Il remplace `Section` et `Bandeau`, qui faisaient la même chose de deux
/// façons. Deux primitives de mise en page finissent toujours par diverger sur
/// un espacement, et personne ne sait plus laquelle fait foi.
#[component]
pub fn Bloc(
    /// Éclaire le bloc — surface haute et lueur du dégradé. À réserver à ce
    /// qu'on veut détacher : sur une page où tout est éclairé, plus rien ne
    /// l'est.
    #[prop(optional)]
    eclaire: bool,
    /// Élargit la mesure — pour ce qui n'est pas du texte courant.
    #[prop(optional)]
    large: bool,
    /// Resserre le rythme vertical, pour un bloc qui prolonge le précédent.
    #[prop(optional)]
    serre: bool,
    /// L'ancre, pour qu'un lien de la page puisse y mener.
    #[prop(optional, into)]
    id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            id=id
            class="border-t border-filet/60"
            class=("voile-aubergine", eclaire)
        >
            <div
                class="mx-auto max-w-mesure px-6 py-24"
                class=("max-w-mesure-large", large)
                class=("py-14", serre)
            >
                {children()}
            </div>
        </section>
    }
}
