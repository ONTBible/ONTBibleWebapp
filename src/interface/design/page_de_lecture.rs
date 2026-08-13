use leptos::prelude::*;

use crate::interface::design::{Bloc, Entete};

/// Le gabarit des pages de la liseuse et du lexique.
///
/// ## Pourquoi elles n'ont pas d'ouverture
///
/// Le reste du site s'ouvre sur un `Hero` qui remplit l'écran : on y arrive
/// sans rien savoir, et l'ouverture est ce qui dit où l'on est. Ces pages-ci
/// sont l'inverse — **on y arrive en sachant**, par un lien partagé, par un mot
/// d'or, par le sommaire. Leur poser un écran d'ouverture mettrait un obstacle
/// d'une hauteur de fenêtre entre le lecteur et le texte qu'il est venu lire.
///
/// C'est la même logique que les pages légales, qui posent leur en-tête
/// elles-mêmes. Il n'y a jamais deux `<header>` sur une page.
///
/// ## Le fil
///
/// Un retour nommé, et non un « ← Retour » générique : ces pages sont
/// **profondes** — un verset est à trois niveaux du sommaire — et quelqu'un qui
/// arrive par un lien partagé n'a pas d'historique à remonter. Le fil est sa
/// seule façon de savoir dans quel livre il se trouve.
#[component]
pub fn PageDeLecture(
    /// Le fil d'Ariane, du plus général au plus précis. La page courante n'y
    /// figure pas : c'est le titre qui la nomme.
    #[prop(optional)]
    fil: Vec<(String, String)>,
    /// La ligne au-dessus du titre — « Torah », « Intraduisible ».
    #[prop(optional, into)]
    rappel: Option<String>,
    #[prop(into)] titre: String,
    /// Ce qui se lit sous le titre, avant le corps — un renvoi, une mention.
    #[prop(optional)]
    chapeau: Option<Children>,
    children: Children,
) -> impl IntoView {
    view! {
        <Entete />
        <Bloc>
            {(!fil.is_empty())
                .then(|| {
                    view! {
                        <nav
                            aria-label="Fil d'Ariane"
                            class="mb-8 flex flex-wrap items-center gap-x-2.5 gap-y-1 text-sm uppercase tracking-capitales text-encre-douce"
                        >
                            {fil
                                .into_iter()
                                .map(|(chemin, nom)| {
                                    view! {
                                        <>
                                            <a href=chemin class="no-underline hover:text-encre">
                                                {nom}
                                            </a>
                                            // Le séparateur est décoratif : à
                                            // l'oreille, une barre oblique entre
                                            // deux liens n'est que du bruit.
                                            //
                                            // `last:hidden` retire celui du bout.
                                            // Il **sépare**, donc il n'a rien à
                                            // faire après le dernier maillon —
                                            // « Lire / Bereshit / » se lisait
                                            // comme un fil coupé.
                                            <span
                                                aria-hidden="true"
                                                class="opacity-40 last:hidden"
                                            >"/"</span>
                                        </>
                                    }
                                })
                                .collect_view()}
                        </nav>
                    }
                })}

            {rappel
                .map(|rappel| {
                    view! {
                        <p class="mb-3 text-sm uppercase tracking-capitales text-accent">{rappel}</p>
                    }
                })}

            <h1 class="mt-0 mb-4 text-balance">{titre}</h1>

            {chapeau.map(|chapeau| view! { <div class="mb-14">{chapeau()}</div> })}

            {children()}
        </Bloc>
    }
}
