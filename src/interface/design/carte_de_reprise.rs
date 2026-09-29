use leptos::prelude::*;
use leptos_router::components::A;

use crate::domaine::surlignage::Position;

/// « Reprendre » — le retour à l'endroit qu'on avait quitté.
///
/// ## Ce n'est pas une destination de plus
///
/// L'app le dit dans sa barre latérale, et la distinction porte tout le
/// dessin : *« les trois suivantes sont des lieux ; celle-ci est un signet »*.
/// D'où sa place — **avant** le corpus, détachée de lui — et sa forme : une
/// carte de bord à bord, et non une ligne de liste de plus.
///
/// ## Elle ne paraît pas sans compte, et ce n'est pas une privation
///
/// La position vit chez le backend de l'app, sous le compte. C'est ce qui fait
/// sa valeur : celui qui a lu sur son téléphone ce matin retrouve ici l'endroit
/// exact, parce que c'est le même serveur qui a délivré les deux sessions.
///
/// Sans compte il n'y a pas de position — pas d'erreur, pas d'invitation à
/// s'inscrire : lire sans compte est le cas normal du site, et une carte qui
/// dirait « connectez-vous pour reprendre » ferait de la lecture une chose
/// qu'on mérite.
///
/// ## Ce qu'elle promet, et ce qu'elle tient
///
/// Le verset, explicitement — l'app l'écrit ainsi : *« la promesse est de
/// rendre l'endroit, pas le chapitre »*. Le site retient la position à
/// l'ouverture d'une unité et non au défilement, donc le verset vaut le plus
/// souvent 1 : le lien le porte quand même, et il vaudra juste le jour où le
/// suivi s'affinera, sans que cette carte bouge.
#[component]
pub fn CarteDeReprise(position: Position) -> impl IntoView {
    let chemin = format!(
        "/fr/lire/{}/{}?v={}",
        position.book_id, position.chapter_id, position.verse
    );
    let ou = format!("{}:{}", position.chapter_title, position.verse);

    view! {
        <A
            href=chemin
            attr:class="carte-de-liste mb-8 flex items-center gap-4 rounded-bloc bg-surface px-5 py-4 no-underline transition-transform duration-150 ease-out hover:-translate-y-px active:scale-[0.99] motion-reduce:transition-none"
        >
            <span class="flex-1">
                <span class="block font-titre text-base font-medium text-encre">"Reprendre"</span>
                <span class="chiffres-tableau mt-0.5 block text-sm text-encre-douce">{ou}</span>
            </span>
            // La flèche qui repart, et non un chevron : un chevron dit « ici
            // dedans », celle-ci dit « là où tu étais ». C'est
            // `arrow.turn.down.right` chez l'app, et elle est en accent —
            // le seul signe coloré de l'écran, parce que c'est la seule ligne
            // qui ne soit pas un lieu.
            <svg
                aria-hidden="true"
                viewBox="0 0 24 24"
                class="size-5 shrink-0 text-accent"
                fill="none"
                stroke="currentColor"
                stroke-width="1.8"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="M4 5v7a3 3 0 0 0 3 3h13" />
                <path d="m16 11 4 4-4 4" />
            </svg>
        </A>
    }
}
