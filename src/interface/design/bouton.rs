use leptos::prelude::*;
use leptos_router::components::A;

/// Une action.
///
/// Deux formes seulement, et c'est délibéré : la **pleine** dit ce qu'on
/// attend du lecteur, la **cerclée** propose une seconde voie. Une troisième
/// forme rendrait le choix illisible — s'il y a trois chemins d'égale
/// importance, c'est qu'il n'y en a aucun.
///
/// ## Trois destinations, trois balises
///
/// La même règle que `Lien`, et il a fallu la lui recopier : elle n'était ici
/// qu'à moitié. Un chemin du site passe par le routeur — sinon la page entière
/// se recharge pour un lien interne. Une adresse hors du site prend `noopener`.
/// Et **une ancre reste une ancre** : la donner au routeur lui fait pousser une
/// navigation pour un déplacement qui ne quitte pas la page.
///
/// Ça marchait — le routeur retombe sur `scrollIntoView` — mais par accident,
/// et ça empêchait d'agir sur le clic : le gestionnaire du routeur est délégué
/// au document, donc il passe après. Le seul bouton du site qui pointe une
/// ancre est « Entrer », celui qui ouvre le seuil.
#[component]
pub fn Bouton(
    #[prop(into)] href: String,
    /// La forme pleine — une seule par écran.
    #[prop(optional)]
    principal: bool,
    /// Ce qu'il faut faire du clic en plus de suivre le lien.
    ///
    /// Un seul usage aujourd'hui, et il est de la bonne sorte : franchir le
    /// seuil à sa vitesse plutôt qu'à celle, fixe, du saut d'ancre natif. Le
    /// gestionnaire décide lui-même s'il empêche le comportement par défaut ;
    /// sans lui, le lien reste un lien.
    #[prop(optional)]
    au_clic: Option<Callback<leptos::ev::MouseEvent>>,
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
    let clic = move |evenement: leptos::ev::MouseEvent| {
        if let Some(au_clic) = au_clic {
            au_clic.run(evenement);
        }
    };

    if href.starts_with('#') {
        view! { <a href=href class=classe on:click=clic>{children()}</a> }.into_any()
    } else if href.starts_with("http") {
        view! { <a href=href rel="noopener" class=classe on:click=clic>{children()}</a> }.into_any()
    } else {
        view! { <A href=href attr:class=classe on:click=clic>{children()}</A> }.into_any()
    }
}
