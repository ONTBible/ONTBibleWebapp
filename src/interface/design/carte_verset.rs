use leptos::prelude::*;

use crate::api::VersetDuJourDto;

/// La carte du verset du jour — un plan éclairé.
///
/// C'est l'un des trois ou quatre **moments** du site : une surface plus
/// claire, cernée d'or, avec un halo qui la décolle du fond et un massif dans
/// sa marge. Sur une page déjà sombre, une carte sombre ne se détache pas
/// toute seule ; c'est la lumière qui la pose, pas une bordure.
///
/// Le verset est composé **un cran** au-dessus du corps, pas trois. L'essai de
/// direction le montait à `text-3xl` : avec un corps à 21 px, ça donnait des
/// lettres de cinquante pixels, dix-sept signes par ligne, et une citation qui
/// criait au lieu de se lire. Un moment se distingue par son traitement — la
/// surface éclairée, le halo, le massif dans la marge — pas par sa taille.
#[component]
pub fn CarteVersetDuJour(verset: VersetDuJourDto) -> impl IntoView {
    view! {
        <article class="halo relative isolate overflow-hidden rounded-carte border border-or/25 bg-surface-haute px-7 py-10">
            // Le massif dans la marge de la carte : on ne le lit pas comme une
            // montagne, on sent qu'il y a un relief derrière le texte.
            <span
                aria-hidden="true"
                class="massif pointer-events-none absolute -right-[15%] -bottom-[35%] -z-10 w-[80%] text-or opacity-[0.07]"
            ></span>

            <p class="mb-7 flex items-center gap-3 text-sm uppercase tracking-capitales text-accent">
                <span class="massif w-7 shrink-0"></span>
                "Verset du jour"
            </p>

            <blockquote
                cite=verset.chemin.clone()
                class="m-0 text-lg leading-[1.6] text-pretty text-encre-vive"
            >
                {verset.texte}
            </blockquote>

            // Le renvoi mène au passage, dans son unité, à son verset. Une
            // citation qui ne ramène pas à sa source est une affirmation sans
            // recours.
            <a
                href=verset.chemin
                class="mt-8 inline-block text-sm uppercase tracking-capitales text-accent"
            >
                {verset.renvoi}
            </a>
        </article>
    }
}
