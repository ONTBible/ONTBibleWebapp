//! Où l'on est — l'arbre courant, lu dans l'adresse.
//!
//! ## Pourquoi un contexte et non un prop
//!
//! Tout ce qui pose un lien a besoin de savoir dans quel arbre il est : un verset
//! cliqué depuis l'édition doit mener à l'édition. Faire descendre l'information
//! en prop demanderait de traverser une quinzaine de composants dont la plupart
//! n'en ont que faire — et la moitié d'entre eux sont des feuilles rendues dans
//! une boucle, où un prop de plus est un prop à oublier.
//!
//! **Et il ne s'agit pas d'un signal reçu d'ailleurs.** Ce dépôt a payé une
//! navigation figée pour l'avoir fait : `#[prop(into)]` crée un signal possédé par
//! l'appelant, donc détruit avec lui, et une barre qui survit aux pages ne peut
//! pas lire un signal qui meurt avec elles. Ici chaque lecteur interroge
//! `use_location()` lui-même — le mémo du routeur, qui survit à tout.
//!
//! ## Le défaut hors des arbres
//!
//! L'accueil, « Le pourquoi », « L'app » n'appartiennent à aucun arbre et posent
//! pourtant des liens vers le corpus. Ils reçoivent [`Arbre::CANONIQUE`] : c'est
//! l'adresse officielle du texte, celle qu'on veut voir partagée depuis une page
//! publique. La préférence du lecteur corrige ensuite, au chargement de la page
//! visée — jamais depuis celle qui pose le lien, qui ne sait rien de lui.

use leptos::prelude::*;

use crate::domaine::lecture::Arbre;

/// L'arbre courant, porté par le contexte plutôt que lu partout.
///
/// **Fourni par `App`**, qui est le seul endroit garanti d'être sous le routeur.
/// Les feuilles le lisent avec un repli : un composant rendu hors routeur — une
/// épreuve qui monte un bloc isolé, un banc — doit rendre quelque chose de juste
/// plutôt que de paniquer.
///
/// ==Une dépendance au routeur qui traverse trente composants est trente
/// endroits où un rendu isolé s'arrête.== Le contexte la ramène à un.
#[derive(Clone, Copy)]
struct ArbreCourant(Signal<Arbre>);

/// Pose l'arbre dans le contexte. Appelé une fois, par `App`.
pub fn fournir_l_arbre() {
    let chemin = leptos_router::hooks::use_location().pathname;
    let courant = Signal::derive(move || {
        chemin.with(|chemin| Arbre::du_chemin(chemin).unwrap_or(Arbre::CANONIQUE))
    });
    provide_context(ArbreCourant(courant));
}

/// L'arbre de l'adresse courante, réactif.
///
/// Une navigation d'un arbre à l'autre le fait changer, et tout ce qui en dépend
/// se recompose — les liens de la barre, le fil d'Ariane, le chrome.
pub fn arbre() -> Signal<Arbre> {
    match use_context::<ArbreCourant>() {
        Some(ArbreCourant(courant)) => courant,
        None => Signal::derive(|| Arbre::CANONIQUE),
    }
}

/// L'arbre courant, lu une fois, sans s'abonner.
///
/// Pour ce qui se décide **au montage** et ne se recompose pas : l'adresse d'un
/// lien posé dans une boucle, le titre d'une page. S'abonner là où rien ne doit
/// rebouger coûte un recalcul à chaque navigation, et n'en rend aucun.
pub fn arbre_maintenant() -> Arbre {
    match use_context::<ArbreCourant>() {
        Some(ArbreCourant(courant)) => courant.get_untracked(),
        None => Arbre::CANONIQUE,
    }
}

/// Le chemin courant, dans l'autre arbre.
///
/// C'est ce que pose le sélecteur de « Vous » : basculer d'habillage ne renvoie
/// pas à l'accueil de l'autre arbre, il rend **la même page**.
///
/// > Un réglage qui fait perdre sa place n'est pas un réglage, c'est une sortie.
pub fn ici_dans(vise: Arbre) -> String {
    let ici = leptos_router::hooks::use_location()
        .pathname
        .get_untracked();
    crate::domaine::chemins::dans(vise, &ici)
}
