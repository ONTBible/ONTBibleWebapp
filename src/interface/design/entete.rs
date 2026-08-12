use leptos::prelude::*;
use leptos_router::components::A;

/// L'en-tête — la marque, puis la navigation.
///
/// Le wordmark est une **image et non du texte** : il porte l'hébreu מקרא הקדם
/// et un dessin de lettres qui n'est pas exactement Jost. Le recomposer en
/// texte le trahirait. Son texte de remplacement dit ce qu'il dit, pour qui ne
/// le voit pas.
///
/// La navigation est en capitales espacées. En bas de casse, elle ressemblerait
/// à une barre d'application ; ainsi, elle ressemble à un titre courant.
#[component]
pub fn Entete() -> impl IntoView {
    view! {
        <header class="flex flex-col items-center gap-6 px-6 pt-10 text-center">
            <A
                href="/fr"
                attr:class="block text-accent"
                attr:aria-label="La Bible ONT — accueil"
            >
                <img
                    src="/images/wordmark.svg"
                    alt="La Bible ONT — מקרא הקדם"
                    width="765"
                    height="307"
                    class="block h-auto w-56 max-w-[60vw]"
                />
            </A>

            <nav
                aria-label="Navigation principale"
                class="flex flex-wrap justify-center gap-x-6 gap-y-2 text-sm uppercase tracking-capitales"
            >
                <A href="/fr/le-pourquoi" attr:class=LIEN>"Le pourquoi"</A>
                <A href="/fr/ce-que-l-ont-n-est-pas" attr:class=LIEN>"Ce que l'ONT n'est pas"</A>
                <A href="/fr/l-auteur" attr:class=LIEN>"L'auteur"</A>
            </nav>
        </header>
    }
}

/// La forme d'un lien de navigation.
///
/// Écrite une fois et partagée, plutôt que recopiée sur chaque lien : trois
/// copies finiraient par diverger d'un pixel, et personne ne saurait laquelle
/// fait foi. `aria-current` marque la page en cours — c'est le routeur qui la
/// pose, donc elle reste juste sans qu'on y pense.
const LIEN: &str = "py-2 text-encre-douce no-underline transition-colors \
                    hover:text-encre hover:underline hover:underline-offset-4 \
                    aria-[current=page]:text-encre aria-[current=page]:underline \
                    aria-[current=page]:underline-offset-4";
