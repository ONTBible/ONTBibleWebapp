use leptos::prelude::*;

use crate::api::VersetDuJourDto;

/// La carte du verset du jour — aubergine et or.
///
/// Les couleurs sont **figées** et ne suivent pas le thème de la page : c'est
/// la même carte que sur l'écran d'accueil d'un iPhone, et c'est à ça qu'on la
/// reconnaît. Une carte qui prend la couleur de son hôte ressemble à tout le
/// reste.
///
/// Le dégradé et le filigrane la distinguent d'un simple aplat : posée sur le
/// parchemin, elle doit avoir l'air d'un objet ancien qu'on a déposé là, pas
/// d'un rectangle coloré.
///
/// Le texte est composé en Literata — la fonte du corps de l'app. Un verset
/// doit se lire ici comme sur le téléphone, sinon le lecteur voit deux
/// traductions là où il n'y en a qu'une.
#[component]
pub fn CarteVersetDuJour(verset: VersetDuJourDto) -> impl IntoView {
    view! {
        <article class="voile-aubergine relative isolate overflow-hidden rounded-carte px-6 py-10 text-or shadow-[0_18px_40px_-24px_rgba(42,16,24,0.55)]">
            <span
                aria-hidden="true"
                class="filigrane-montagne pointer-events-none absolute -right-[18%] -bottom-[30%] \
                       -z-10 w-[85%] opacity-8"
            ></span>

            <p class="mb-6 flex items-center gap-2 text-sm uppercase tracking-capitales">
                <span class="signe-montagne w-7 shrink-0" aria-hidden="true"></span>
                "Verset du jour"
            </p>

            <blockquote
                cite=verset.chemin.clone()
                class="m-0 font-citation text-lg leading-relaxed text-pretty"
            >
                {verset.texte}
            </blockquote>

            // Le renvoi est un lien : il mène au passage, dans son unité, à son
            // verset. Une citation qui ne ramène pas à sa source est une
            // affirmation sans recours.
            <a
                href=verset.chemin
                class="mt-6 inline-block text-sm uppercase tracking-capitales decoration-or/50 hover:decoration-or"
            >
                {verset.renvoi}
            </a>
        </article>
    }
}
