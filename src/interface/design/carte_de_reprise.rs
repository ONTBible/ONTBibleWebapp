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
    let chemin = crate::domaine::chemins::unite_au_verset(
        crate::interface::arbre::arbre_maintenant(),
        &position.book_id,
        &position.chapter_id,
        &position.verse.to_string(),
    );
    let ou = format!("{}:{}", position.chapter_title, position.verse);

    // **La DA du hero, mais sous la webapp seulement.**
    //
    // `ONTHero` est le pavé d'appel en tête d'onglet, et son commentaire porte
    // la décision de l'auteur du 13 septembre 2026 : *« la DA du hero de
    // prononciation vaut pour les deux »*. Les deux, ce sont la feuille de
    // prononciation dans le Lexique **et la reprise de lecture dans la Bible** —
    // *un onglet a un hero ou n'en a pas ; il n'en a jamais deux.*
    //
    // Son argument tient à l'arrivée : un hero *doit se voir d'un coup d'œil en
    // arrivant*, donc il est doré et plein. C'est vrai d'un écran d'app, où l'on
    // atterrit sur un onglet et où l'œil cherche la porte principale.
    //
    // **L'édition n'a pas cette grammaire.** On y arrive par un en-tête, un
    // rappel en capitales et un titre ; un aplat de marque y pèserait plus que
    // le titre de la page, et le site n'emploie l'aplat qu'à deux endroits — le
    // bouton de connexion et la feuille de prononciation —, tous deux pour dire
    // *« ceci n'est pas du corpus, c'est l'app qui te parle »*.
    //
    // ==Une DA se porte avec la grammaire qui la justifie, pas toute seule.==
    // Arbitré par l'auteur le 2 octobre 2026 : *« sur la webapp récupère la DA
    // du hero de vocalisation, mais sur la liseuse laisse comme elle est »*.
    let hero = !crate::interface::arbre::sous_l_edition();

    view! {
        <A
            href=chemin
            attr:class=if hero {
                "presse survol survol--souleve mb-8 flex min-h-[4.75rem] items-center gap-4 \
                 rounded-bloc bg-marque-encre px-5 py-4 text-sur-marque-accent no-underline"
            } else {
                "presse survol survol--souleve carte-de-liste mb-8 flex items-center gap-4 \
                 rounded-bloc bg-surface px-5 py-4 no-underline"
            }
        >
            <span class="flex-1">
                <span
                    class="block font-titre text-base leading-snug"
                    class=("font-semibold", hero)
                    class=("font-medium", !hero)
                    class=("text-encre", !hero)
                >
                    "Reprendre"
                </span>
                // Sous le hero, l'opacité plutôt qu'une encre atténuée : sur un
                // aplat doré, `text-encre-douce` part du fond de l'**écran** et
                // rend un gris qui n'a rien à voir avec l'or. C'est la mesure de
                // l'app, et elle vaut ici au mot près — *une opacité de la même
                // encre garde le rapport voulu sur les quatre thèmes.*
                <span
                    class="chiffres-tableau block text-sm"
                    class=("mt-1", hero)
                    class=("opacity-85", hero)
                    class=("mt-0.5", !hero)
                    class=("text-encre-douce", !hero)
                >
                    {ou}
                </span>
            </span>
            // La flèche qui repart, et non un chevron : un chevron dit « ici
            // dedans », celle-ci dit « là où tu étais ». C'est
            // `arrow.turn.down.right` chez l'app, et elle est en accent —
            // le seul signe coloré de l'écran, parce que c'est la seule ligne
            // qui ne soit pas un lieu.
            <svg
                aria-hidden="true"
                viewBox="0 0 24 24"
                // Sur l'aplat, la flèche prend l'encre du pavé : l'or sur de
                // l'or ne se voit pas, et le hero n'a qu'une couleur.
                class="size-5 shrink-0"
                class=("text-accent", !hero)
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
