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
        // Les deux moitiés du trait sont des pseudo-éléments souples : la
        // marque reste centrée quelle que soit la mesure, sans calcul de
        // largeur ni second élément.
        view! {
            <div
                role="separator"
                aria-hidden="true"
                class="my-16 flex items-center gap-6 text-accent \
                       before:h-px before:flex-1 before:bg-filet before:content-[''] \
                       after:h-px after:flex-1 after:bg-filet after:content-['']"
            >
                <span class="signe-montagne w-10"></span>
            </div>
        }
        .into_any()
    } else {
        view! { <hr class="my-10 h-px border-0 bg-filet" /> }.into_any()
    }
}
