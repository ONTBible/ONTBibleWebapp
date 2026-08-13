use leptos::prelude::*;

use crate::interface::design::Entete;

/// L'ouverture d'une page — on entre dans un lieu.
///
/// C'est le parti pris retenu entre trois : ni une affiche, ni une notice, mais
/// un **espace**. La lumière tombe du haut comme dans une nef, la montagne
/// ferme le bas comme un horizon, et le texte est dedans.
///
/// Deux massifs et non un seul : le premier en aubergine, franc, donne la ligne
/// d'horizon ; le second, plus large et à peine visible en or, suggère un relief
/// plus lointain. C'est le décalage entre les deux qui fait la profondeur — un
/// massif isolé aurait l'air d'un autocollant.
///
/// ## L'en-tête est **dans** l'ouverture
///
/// La première chose qu'on voit doit être **une** unité qui remplit l'écran.
/// Avec une bande de navigation posée au-dessus, on en voit deux — un bandeau,
/// puis un écran — et l'effet tombe.
///
/// Le premier montage remontait l'ouverture *sous* l'en-tête, d'une marge
/// négative égale à un jeton `--hauteur-entete`. Ça marchait sur un écran large
/// et cassait sur un téléphone : la navigation y passe sur trois lignes, donc
/// l'en-tête devient deux fois plus haut que le nombre inscrit, la bande
/// réapparaît, et le contenu déborde sous la ligne de flottaison.
///
/// L'ouverture **contient** donc l'en-tête. Plus de nombre à tenir à jour, et
/// la mise en page se règle d'elle-même quelle que soit la hauteur réelle de la
/// navigation. Les pages sans ouverture — les légales, l'erreur — posent leur
/// en-tête elles-mêmes.
///
/// ## Le contenu est ancré en haut, pas centré
///
/// Centrer verticalement est joli sur une page isolée, mais un contenu plus
/// haut — le portrait de la page de l'auteur — commence alors plus haut : le
/// rappel s'y retrouvait cent soixante pixels au-dessus de celui des autres
/// pages, collé sous la navigation. Une distance fixe sous l'en-tête pose le
/// rappel et le titre au même endroit partout.
///
/// Le vide qui reste en bas n'est pas perdu : c'est là que l'horizon se lit.
///
/// ## Les espaces se resserrent sur téléphone
///
/// Un écran de téléphone offre moins de la moitié de la hauteur d'un écran
/// d'ordinateur, et Safari lui reprend encore sa barre. Les écarts et la marge
/// haute y sont donc réduits : sans ça, la dernière ligne de l'ouverture passe
/// sous la barre, et l'unité qu'on cherchait n'en est plus une.
#[component]
pub fn Hero(
    /// Une ouverture de page intérieure : même lieu, voix plus basse.
    ///
    /// L'accueil a le droit de porter la lumière à pleine force — on y arrive
    /// sans rien savoir. Une page intérieure s'ouvre sur quelqu'un qui a déjà
    /// choisi d'y aller : elle n'a plus à convaincre, seulement à situer.
    ///
    /// Elle ne diffère que par la **lumière**, jamais par la hauteur. Un essai
    /// l'avait fixée à 70 % de l'écran : les pages intérieures montraient alors
    /// leur ouverture *et* le début du bloc suivant, ce qui détruit exactement
    /// l'unité recherchée.
    #[prop(optional)]
    sobre: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            class="relative isolate flex min-h-dvh flex-col overflow-hidden pb-14 text-center sm:pb-20"
            class=("voute", !sobre)
            class=("voute-basse", sobre)
        >
            <span
                aria-hidden="true"
                class="massif pointer-events-none absolute left-1/2 -z-10 -translate-x-1/2 text-aubergine"
                class=("-bottom-[6%]", !sobre)
                class=("w-[165%]", !sobre)
                class=("opacity-80", !sobre)
                class=("-bottom-[14%]", sobre)
                class=("w-[150%]", sobre)
                class=("opacity-45", sobre)
            ></span>
            <span
                aria-hidden="true"
                class="massif pointer-events-none absolute -bottom-[13%] left-1/2 -z-10 w-[200%] -translate-x-1/2 text-or opacity-[0.055]"
                class=("hidden", sobre)
            ></span>

            <Entete />

            // Le contenu occupe la hauteur restante et s'y centre.
            //
            // Un premier écran se lit comme une affiche : le vide au-dessus
            // vaut celui d'en dessous, et un contenu posé en haut laisse un
            // trou en bas qu'aucun regard ne comble.
            //
            // L'ancrage en haut, à distance fixe, était la règle précédente. Il
            // faisait tomber le rappel et le titre exactement à la même hauteur
            // d'une page à l'autre — ce que le centrage ne garantit plus, une
            // ouverture au contenu plus haut commençant plus haut. C'est le
            // prix, et il est assumé : toutes les ouvertures remplissent
            // l'écran, donc toutes se centrent de la même façon.
            <div class="flex flex-1 flex-col items-center justify-center gap-4 px-6 sm:gap-8">
                {children()}
            </div>
        </section>
    }
}
