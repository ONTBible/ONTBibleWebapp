use leptos::prelude::*;

/// L'ouverture du site — on entre dans un lieu.
///
/// C'est le parti pris qu'il a choisi entre trois : ni une affiche, ni une
/// notice, mais un **espace**. La lumière tombe du haut comme dans une nef,
/// la montagne ferme le bas comme un horizon, et le texte est dedans.
///
/// Deux massifs et non un seul : le premier en aubergine, franc, donne la
/// ligne d'horizon ; le second, plus large et à peine visible en or, suggère un
/// relief plus lointain derrière. C'est ce décalage qui fait la profondeur —
/// un massif isolé aurait l'air d'un autocollant.
///
/// ## Le contenu est ancré en haut, pas centré
///
/// Le premier réflexe est de centrer verticalement, et c'est joli sur une page
/// isolée. Mais un contenu plus haut — le portrait de la page de l'auteur —
/// commence alors plus haut : le rappel s'y retrouvait cent soixante pixels
/// au-dessus de celui des autres pages, collé sous la navigation.
///
/// Le centrage est cohérent avec lui-même et incohérent d'une page à l'autre.
/// Une distance fixe sous l'en-tête pose le rappel et le titre au même endroit
/// partout, quel que soit ce qui suit. Le vide qui reste en bas n'est pas
/// perdu : c'est là que l'horizon se lit.
///
/// ## Elle occupe l'écran entier, en-tête compris
///
/// Elle remonte **sous** l'en-tête d'une marge négative, et lui rend l'espace
/// en marge intérieure. La marque et la navigation flottent donc dans le lieu
/// au lieu de le surmonter.
///
/// C'est ce qui manquait : la première chose qu'on voit doit être **une** unité
/// qui remplit l'écran. Avec une bande en haut, on en voit deux — un bandeau,
/// puis un écran — et l'effet tombe.
#[component]
pub fn Hero(
    /// Une ouverture de page intérieure : même lieu, voix plus basse.
    ///
    /// L'accueil a le droit de porter la lumière à pleine force — on y arrive
    /// sans rien savoir. Une page intérieure s'ouvre sur quelqu'un qui a déjà
    /// choisi d'y aller : elle n'a plus à convaincre, seulement à situer.
    /// D'où l'horizon plus bas et la lueur diminuée.
    ///
    /// Elle ne diffère que par la **lumière**, jamais par la hauteur. Un
    /// premier essai la fixait à 70 % de l'écran : les pages intérieures
    /// montraient alors leur ouverture *et* le début du bloc suivant, ce qui
    /// détruit précisément l'unité qu'on cherche. Et sur la page de l'auteur,
    /// dont l'ouverture porte un portrait, le contenu débordait du centrage et
    /// remontait dans la marge de l'en-tête — le rappel s'y retrouvait collé
    /// sous la navigation, alors qu'ailleurs il respirait.
    #[prop(optional)]
    sobre: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            class="relative isolate mt-[calc(var(--hauteur-entete)*-1)] flex min-h-dvh flex-col items-center overflow-hidden px-6 pt-[calc(var(--hauteur-entete)+5rem)] pb-24 text-center"
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

            <div class="flex max-w-4xl flex-col items-center gap-8">{children()}</div>
        </section>
    }
}
