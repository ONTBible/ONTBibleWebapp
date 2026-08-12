use leptos::prelude::*;

/// Le filet — la ligne fine du livre imprimé.
///
/// C'est la marque de la direction « un peu ancienne » : un livre classique
/// sépare par un trait, pas par une bordure de bloc ni par du vide.
///
/// Deux formes, et deux balises différentes : un séparateur nu est un `<hr>`,
/// qui porte déjà le sens ; un filet orné porte la montagne du logo, donc un
/// contenu, ce qu'un `<hr>` ne peut pas avoir. Il annonce alors son rôle et se
/// retire de la lecture assistée, puisqu'il n'ajoute rien à l'oreille.
#[component]
pub fn Filet(
    /// Pose le signe de section — la montagne — au milieu du trait.
    #[prop(optional)]
    orne: bool,
) -> impl IntoView {
    if orne {
        view! {
            <div class="filet filet--orne" role="separator" aria-hidden="true">
                <span class="filet__signe"></span>
            </div>
        }
        .into_any()
    } else {
        view! { <hr class="filet" /> }.into_any()
    }
}
