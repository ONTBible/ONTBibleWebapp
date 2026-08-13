use leptos::prelude::*;
use leptos_router::components::A;

/// Une action.
///
/// Deux formes seulement, et c'est délibéré : la **pleine** dit ce qu'on
/// attend du lecteur, la **cerclée** propose une seconde voie. Une troisième
/// forme rendrait le choix illisible — s'il y a trois chemins d'égale
/// importance, c'est qu'il n'y en a aucun.
///
/// Un lien interne passe par le routeur, un lien externe par une balise
/// ordinaire : sans cette distinction, une adresse hors du site serait
/// interceptée par le routeur, qui chercherait une page inexistante.
#[component]
pub fn Bouton(
    #[prop(into)] href: String,
    /// La forme pleine — une seule par écran.
    #[prop(optional)]
    principal: bool,
    children: Children,
) -> impl IntoView {
    let forme = if principal {
        "bg-or text-nuit hover:bg-encre-vive"
    } else {
        "border border-or/45 text-accent hover:border-or hover:bg-or/8"
    };
    let classe = format!(
        "inline-block rounded-full px-6 py-3 text-sm uppercase tracking-capitales \
         no-underline transition-colors {forme}"
    );

    if href.starts_with("http") {
        view! { <a href=href rel="noopener" class=classe>{children()}</a> }.into_any()
    } else {
        view! { <A href=href attr:class=classe>{children()}</A> }.into_any()
    }
}
