use leptos::prelude::*;

/// Ce qui sort de la colonne de lecture.
///
/// Un `Bloc` borne son contenu à la **mesure** : au-delà de ~70 signes par
/// ligne, l'œil perd le début de la ligne suivante. C'est une contrainte de
/// lecture, et elle ne se négocie pas.
///
/// Mais une comparaison en deux colonnes, une rangée de chiffres ou un
/// portrait à côté de sa légende ne sont pas du texte suivi. Les enfermer dans
/// la mesure donne, sur grand écran, un site en ruban — c'était le défaut du
/// premier essai, où tout avait la même largeur.
///
/// Ce composant laisse un enfant sortir de la colonne **sans quitter son
/// centre** : la marge négative vaut la moitié de l'écart entre les deux
/// largeurs. Sur téléphone, il ne se passe rien — la largeur demandée dépasse
/// déjà l'écran, et tout retombe sur une colonne.
#[component]
pub fn Deborde(
    /// Jusqu'à la borne extérieure de la page, et non seulement d'un cran.
    #[prop(optional)]
    pleine: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="deborde-large" class=("deborde-page", pleine)>
            {children()}
        </div>
    }
}
