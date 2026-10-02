use leptos::prelude::*;

/// Un bloc de page — la primitive de mise en page du site.
///
/// Il touche les deux bords de l'écran et rétablit la **mesure** à
/// l'intérieur : le fond respire, le texte reste lisible.
///
/// ## Un écran par bloc
///
/// Chaque bloc occupe au moins la hauteur de la fenêtre, et son contenu y est
/// centré. C'est ce qui transforme l'alternance des fonds en **rythme** : sans
/// cette hauteur, les bandes épousaient la longueur des textes et se lisaient
/// comme des rayures posées au hasard.
///
/// `min-h-dvh` et non `h-screen`, pour deux raisons distinctes :
///
/// - **minimale**, pas fixe : la comparaison ou une page légale dépassent un
///   écran, et une hauteur fixe les couperait ;
/// - **`dvh`** et non `vh` : sur un téléphone, `vh` compte la fenêtre sans la
///   barre d'adresse, qui se rétracte au défilement. Chaque bloc sauterait de
///   quelques dizaines de pixels au premier geste.
///
/// ## La lumière
///
/// Tous les blocs portent la voûte, mais **assourdie** : la même lumière que
/// l'ouverture, quatre fois plus basse. `eclaire` la remonte à pleine force,
/// et c'est à réserver à deux ou trois moments — sur une page où tout est
/// éclairé, plus rien ne l'est.
#[component]
pub fn Bloc(
    /// Remonte la lumière à pleine force.
    #[prop(optional)]
    eclaire: bool,
    /// Élargit la mesure — pour ce qui n'est pas du texte courant.
    #[prop(optional)]
    large: bool,
    /// La mesure d'une **page** plutôt que celle d'une phrase.
    ///
    /// C'est `pageWidth` contre `readingWidth` chez l'app : une liste, une
    /// carte, un panneau de réglages. Ils ne se lisent pas d'un bout à
    /// l'autre — l'œil y saute d'un intitulé à sa valeur —, et les borner à la
    /// mesure d'un verset pose le compteur au milieu de la ligne au lieu du
    /// bord.
    ///
    /// **Exclusif de `large`**, et il faut que ça le soit dans le balisage :
    /// c'est exactement le piège raconté plus bas, un cran plus loin. Le
    /// calcul se fait donc en amont, en une fois.
    #[prop(optional)]
    page: bool,
    /// **Sans la voûte ni le filet** — un écran d'app, pas une section de page.
    ///
    /// La voûte est l'éclairage de l'**édition** : une lueur d'aubergine en
    /// haut, la nuit qui reprend vers le bas, de sorte que la lumière semble
    /// venir d'au-dessus du texte. Elle est dessinée avec `--color-aubergine`,
    /// qui est **la marque** et ne suit donc aucun thème.
    ///
    /// Posée sur parchemin, elle devient une brume rose derrière le texte —
    /// c'est ce que l'auteur a vu, et c'est le défaut que le §8 undecies avait
    /// déjà relevé sur le massif de l'accueil, mot pour mot : *« posés sur du
    /// parchemin, le massif devient une forme violette sur de la crème »*.
    ///
    /// Le filet part avec : il dit « nouvelle section » dans une page qui en
    /// enchaîne plusieurs. Un écran de webapp n'en a qu'une.
    #[prop(optional)]
    nu: bool,
    /// L'ancre, pour qu'un lien de la page puisse y mener.
    #[prop(optional, into)]
    id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            id=id
            // **Trois états d'éclairage, donc une classe calculée.** À
            // spécificité égale, trois `class=(…)` se départagent par l'ordre
            // de la feuille — le piège de `max-w-mesure`, et celui que ce
            // fichier raconte déjà dix lignes plus bas.
            class=if nu {
                "flex min-h-dvh flex-col justify-center"
            } else if eclaire {
                "flex min-h-dvh flex-col justify-center border-t border-filet/50 voute"
            } else {
                "flex min-h-dvh flex-col justify-center border-t border-filet/50 voute-basse"
            }
        >
            // Les deux largeurs sont **exclusives**, et il faut qu'elles le
            // soient dans le balisage, pas seulement dans l'intention.
            //
            // Ce composant portait `max-w-mesure` en dur *plus* `max-w-large`
            // en conditionnel. Les deux classes se retrouvaient sur l'élément,
            // à spécificité égale — et à spécificité égale, c'est l'ordre de la
            // **feuille de style** qui tranche, pas celui de l'attribut. Or
            // Tailwind range `max-w-mesure` après `max-w-large`. Le prop
            // `large` ne faisait donc rien, nulle part, depuis le premier jour :
            // la comparaison de l'accueil — la pièce qui porte tout le site —
            // se composait sur 38 rem au lieu de 52, et ses deux colonnes
            // tombaient à 230 et 330 px, où le texte se césurait à chaque ligne.
            //
            // Un défaut de ce genre ne se voit pas : la page ne casse pas, elle
            // est juste étroite, et rien ne dit qu'elle devrait l'être moins.
            // Trois prétendants maintenant, donc la classe se **calcule** au
            // lieu de s'empiler : à trois `class=(…)`, deux peuvent être vrais
            // à la fois, et c'est encore l'ordre de la feuille qui trancherait.
            // `large` l'emporte, étant la plus large — un appel qui demande les
            // deux demande de la place.
            <div class=if large {
                "mx-auto w-full px-6 py-24 max-w-large"
            } else if page {
                "mx-auto w-full px-6 py-24 max-w-page"
            } else {
                "mx-auto w-full px-6 py-24 max-w-mesure"
            }>
                {children()}
            </div>
        </section>
    }
}
