use leptos::prelude::*;

/// Une phrase mise en exergue.
///
/// Le livre imprimé détache une phrase par la taille et les marges, pas par un
/// encadré ni par une couleur de fond. Un bloc coloré au milieu d'une page de
/// texte est une convention d'écran, et elle jure avec le reste.
#[component]
pub fn Exergue(children: Children) -> impl IntoView {
    view! {
        <p class="my-16 px-6 text-center text-xl leading-snug text-balance">
            {children()}
        </p>
    }
}
