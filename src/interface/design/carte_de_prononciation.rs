use leptos::prelude::*;
use leptos_router::components::A;

/// Le pavé qui ouvre la feuille de prononciation, en tête du lexique.
///
/// ## Pourquoi un pavé et non une ligne de plus
///
/// Le lexique est une liste de plusieurs centaines d'entrées. Une ligne de
/// plus s'y noierait, et personne ne la toucherait jamais — alors que c'est ce
/// qu'il faut lire **avant** la première fiche. L'app dit exactement ça, et
/// donne même le rapport : *« au carré, le bloc pèse comme une carte de
/// contenu et concurrence la liste ; à un tiers de sa largeur, il se lit comme
/// un en-tête et laisse la liste commencer »*. Le rapport de un à trois vient
/// de Gloire.
///
/// Il est tenu par une **hauteur minimale** et non par un `aspect-ratio` : ce
/// dernier imposerait sa forme au texte, qui grandit avec le réglage de
/// taille, et le pavé se déformerait chez qui en a le plus besoin.
///
/// ## L'aplat de marque, et il n'y en a que deux dans l'app
///
/// `brandInk` en fond, `onBrandAccent` dessus — l'aplat des boutons de
/// connexion. C'est le seul autre endroit où la marque s'affirme en aplat, et
/// c'est voulu : les deux disent « ceci n'est pas du corpus, c'est l'app qui
/// te parle ».
///
/// ## Jost, et non la fonte de lecture
///
/// Le libellé passe à deux lignes au premier cran d'agrandissement, et la
/// fonte d'affichage porte un interligne fait pour un titre d'une ligne : le
/// blanc entre les deux vaut alors le double de celui du sous-titre, et le
/// pavé se lit comme deux fragments. L'app l'a mesuré, et a écarté en chemin
/// l'hypothèse de la courbe d'échelle : l'interligne était en cause. C'est de
/// la chrome, donc les fontes de la chrome.
#[component]
pub fn CarteDePrononciation() -> impl IntoView {
    view! {
        <A
            href="/fr/lexique/prononciation"
            attr:class="mb-8 flex min-h-[4.75rem] items-center gap-4 rounded-bloc bg-marque-encre px-5 py-4 text-sur-marque-accent no-underline transition-transform duration-150 ease-out hover:-translate-y-px active:scale-[0.98] motion-reduce:transition-none"
        >
            <span class="flex-1">
                <span class="block font-titre text-base font-semibold leading-snug">
                    "Comment ça se prononce"
                </span>
                <span class="mt-1 block font-titre text-sm opacity-85">
                    "Les cinq sons que le français n'a pas"
                </span>
            </span>
            // L'onde de `waveform`, redessinée : cinq barres de hauteurs
            // inégales. Le symbole d'Apple n'est pas redistribuable hors de ses
            // plateformes, et c'est le signe qui compte, pas le tracé.
            <svg
                aria-hidden="true"
                viewBox="0 0 256 256"
                fill="currentColor"
                class="size-6 shrink-0"
            >
                <path d=crate::interface::design::symboles::trace("onde", false) />
            </svg>
        </A>
    }
}
