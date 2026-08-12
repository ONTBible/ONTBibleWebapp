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
/// cette hauteur, les bandes claires et sombres épousaient la longueur des
/// textes et se lisaient comme des rayures posées au hasard.
///
/// `min-h-dvh` et non `h-screen`, pour deux raisons distinctes :
///
/// - **minimale**, pas fixe : la comparaison ou une page légale dépassent un
///   écran, et une hauteur fixe les couperait ;
/// - **`dvh`** et non `vh` : sur un téléphone, `vh` compte la fenêtre sans la
///   barre d'adresse, qui se rétracte au défilement. Chaque bloc sauterait de
///   quelques dizaines de pixels au premier geste.
///
/// Il remplace `Section` et `Bandeau`, qui faisaient la même chose de deux
/// façons. Deux primitives de mise en page finissent toujours par diverger sur
/// un espacement, et personne ne sait plus laquelle fait foi.
#[component]
pub fn Bloc(
    /// Éclaire le bloc — surface haute et lueur du dégradé. À réserver à ce
    /// qu'on veut détacher : sur une page où tout est éclairé, plus rien ne
    /// l'est.
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
            class="flex min-h-dvh flex-col justify-center border-t border-filet/60"
            class=("voile-aubergine", eclaire)
        >
            <div
                class="mx-auto w-full max-w-mesure px-6 py-24"
                class=("max-w-mesure-large", large)
            >
                {children()}
            </div>
        </section>
    }
}
