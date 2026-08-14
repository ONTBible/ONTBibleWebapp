use leptos::prelude::*;
use leptos_router::components::A;

use crate::interface::design::image;

/// L'en-tête — la marque, puis la navigation.
///
/// Le wordmark est une **image et non du texte** : il porte l'hébreu מקרא הקדם
/// et un dessin de lettres qui n'est pas exactement Jost. Le recomposer en
/// texte le trahirait. Son texte de remplacement dit ce qu'il dit, pour qui ne
/// le voit pas.
///
/// La navigation est en capitales espacées. En bas de casse, elle ressemblerait
/// à une barre d'application ; ainsi, elle ressemble à un titre courant.
///
/// ## La marque se mesure en part d'écran, comme le texte
///
/// Elle était posée à 224 px en dur — 17 % de la largeur sur un ordinateur,
/// 56 % sur un téléphone. Le rapport à ce qui l'entoure n'était donc pas le
/// même d'un écran à l'autre, et sur téléphone l'en-tête pesait plus lourd que
/// le titre qui porte le propos. Le jeton `--container-marque` l'interpole
/// comme l'échelle typographique interpole le texte : le grand écran ne bouge
/// pas, le téléphone se resserre.
///
/// Le `max-w-[60vw]` qui la bornait est parti avec : une borne qui ne mord
/// jamais est une borne qu'on croit active. Le `clamp` la contient déjà.
///
/// ## Elle n'a pas de fond, et elle vit dans l'ouverture
///
/// `Hero` la contient : la marque et la navigation flottent dans le lieu au
/// lieu de le surmonter. C'est ce qui fait qu'on arrive sur une seule unité qui
/// remplit l'écran, et non sur une bande suivie d'un écran. D'où le `z-20` et
/// l'absence de fond — le dégradé de l'ouverture passe derrière.
///
/// Les capitales espacées se resserrent en dessous du seuil : à 16 px avec un
/// interlettrage de 0,16 em, les trois entrées passaient sur **trois lignes**
/// sur un téléphone, ce qui poussait tout le contenu sous la ligne de
/// flottaison.
///
/// La contrainte est donc à surveiller à chaque entrée qu'on ajoute : le seuil
/// n'est pas le nombre d'entrées, c'est la hauteur qu'elles prennent sur un
/// écran de téléphone. Elle se vérifie au simulateur (`scripts/sim.sh`), jamais
/// à l'œil sur un grand écran.
#[component]
pub fn Entete() -> impl IntoView {
    view! {
        <header class="relative z-20 flex flex-col items-center gap-5 px-6 pt-8 text-center sm:gap-6 sm:pt-10">
            <A
                href="/fr"
                attr:class="block text-accent"
                attr:aria-label="La Bible ONT — accueil"
            >
                <img
                    src=image("wordmark.svg")
                    alt="La Bible ONT — מקרא הקדם"
                    width="765"
                    height="307"
                    class="block h-auto w-marque"
                />
            </A>

            <nav
                aria-label="Navigation principale"
                class="flex flex-wrap justify-center gap-x-4 gap-y-1 text-[0.8rem] uppercase tracking-[0.09em] sm:gap-x-6 sm:gap-y-2 sm:text-sm sm:tracking-capitales"
            >
                // « Lire » et « Lexique » en tête, et ce n'est pas un détail
                // d'ordre : ce sont le corpus, le reste est le discours sur le
                // corpus. Tant qu'ils manquaient, la liseuse existait sans
                // qu'aucun lien n'y mène — on y arrivait en tapant une adresse,
                // ou par le renvoi du verset du jour. Ce n'était pas un chemin.
                <A href="/fr/lire" attr:class=LIEN>"Lire"</A>
                <A href="/fr/lexique" attr:class=LIEN>"Lexique"</A>
                <A href="/fr/le-pourquoi" attr:class=LIEN>"Le pourquoi"</A>
                <A href="/fr/ce-que-l-ont-n-est-pas" attr:class=LIEN>"Ce que l'ONT n'est pas"</A>
                <A href="/fr/l-app" attr:class=LIEN>"L'app"</A>
                // « L'auteur » revient quand sa page aura été relue — voir `app.rs`.
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
