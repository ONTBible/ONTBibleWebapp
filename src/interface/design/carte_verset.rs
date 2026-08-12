use leptos::prelude::*;

use crate::api::VersetDuJourDto;

/// La carte du verset du jour.
///
/// Sur une page déjà sombre, une carte sombre ne se détache pas toute seule.
/// Elle se pose donc **au-dessus** : une surface plus claire d'un cran, un
/// filet d'or au bord, et une ombre portée qui la décolle du fond. C'est ce
/// que l'app fait avec sa carte du Qahal, et c'est à ça qu'on la reconnaît.
///
/// Le verset est composé dans la fonte du corps — Literata, celle de l'app.
/// Il se lit ici exactement comme sur le téléphone.
#[component]
pub fn CarteVersetDuJour(verset: VersetDuJourDto) -> impl IntoView {
    view! {
        <article class="voile-aubergine relative isolate overflow-hidden rounded-carte border border-or/20 px-6 py-10 shadow-[0_24px_60px_-30px_rgba(0,0,0,0.9)]">
            <p class="mb-6 flex items-center gap-2 text-sm uppercase tracking-capitales text-accent">
                <span class="signe-montagne w-7 shrink-0" aria-hidden="true"></span>
                "Verset du jour"
            </p>

            <blockquote
                cite=verset.chemin.clone()
                class="m-0 text-lg leading-relaxed text-pretty text-encre-vive"
            >
                {verset.texte}
            </blockquote>

            // Le renvoi est un lien : il mène au passage, dans son unité, à son
            // verset. Une citation qui ne ramène pas à sa source est une
            // affirmation sans recours.
            <a
                href=verset.chemin
                class="mt-6 inline-block text-sm uppercase tracking-capitales text-accent"
            >
                {verset.renvoi}
            </a>
        </article>
    }
}
