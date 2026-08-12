use leptos::prelude::*;

/// La grille des entrées du projet.
///
/// Les traits qui séparent les portes sont le **fond** de la grille, révélé
/// par un écart d'un pixel : aucune bordure à gérer, donc aucun cas
/// particulier pour la dernière porte.
#[component]
pub fn Portes(children: Children) -> impl IntoView {
    view! {
        <nav
            aria-label="Entrer dans le projet"
            class="grid gap-px border-y border-filet bg-filet"
        >
            {children()}
        </nav>
    }
}
