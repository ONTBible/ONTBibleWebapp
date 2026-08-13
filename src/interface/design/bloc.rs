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
    /// L'ancre, pour qu'un lien de la page puisse y mener.
    #[prop(optional, into)]
    id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            id=id
            class="flex min-h-dvh flex-col justify-center border-t border-filet/50"
            class=("voute-basse", !eclaire)
            class=("voute", eclaire)
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
            <div
                class="mx-auto w-full px-6 py-24"
                class=("max-w-mesure", !large)
                class=("max-w-large", large)
            >
                {children()}
            </div>
        </section>
    }
}
