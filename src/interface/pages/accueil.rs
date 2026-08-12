use leptos::prelude::*;

use crate::interface::design::{Filet, Section};

/// L'accueil.
///
/// Provisoire : elle ne porte encore que le principe, le temps de vérifier que
/// le rendu côté serveur tourne. Le verset du jour et la suite viendront une
/// fois le design system posé.
#[component]
pub fn Accueil() -> impl IntoView {
    view! {
        <Section>
            <h1>"La Bible ONT"</h1>
            <Filet orne=true />
            <p>
                "Le cosmos hébreu n'est pas une usine. C'est un Temple."
            </p>
        </Section>
    }
}
