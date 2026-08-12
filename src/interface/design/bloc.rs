use leptos::prelude::*;

/// Un bloc de page — la primitive de mise en page du site.
///
/// Il touche les deux bords de l'écran et rétablit la **mesure** à
/// l'intérieur : le fond respire, le texte reste lisible. C'est ce qui donne
/// au site son rythme — une suite de bandes qui alternent, plutôt qu'une
/// colonne unique posée au milieu du vide.
///
/// Il borne toujours son contenu à la **mesure** : c'est une contrainte de
/// lecture, pas un choix de mise en page. Ce qui n'est pas du texte suivi en
/// sort explicitement, avec `Deborde` — l'inverse ferait de chaque bloc une
/// décision à reprendre.
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
                class=("py-14", serre)
            >
                {children()}
            </div>
        </section>
    }
}
