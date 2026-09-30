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

/// L'arbre de l'adresse courante.
///
/// Réactif : une navigation d'un arbre à l'autre le fait changer, et tout ce qui
/// en dépend se recompose — les liens de la barre, le fil d'Ariane, le chrome.
pub fn arbre() -> Signal<Arbre> {
    let chemin = leptos_router::hooks::use_location().pathname;
    Signal::derive(move || {
        chemin.with(|chemin| Arbre::du_chemin(chemin).unwrap_or(Arbre::CANONIQUE))
    })
}

/// L'arbre courant, lu une fois, sans s'abonner.
///
/// Pour ce qui se décide **au montage** et ne se recompose pas : le titre d'une
/// page, l'adresse canonique, la classe d'un conteneur. S'abonner là où rien ne
/// doit rebouger coûte un recalcul à chaque navigation, et n'en rend aucun.
pub fn arbre_maintenant() -> Arbre {
    let chemin = leptos_router::hooks::use_location().pathname;
    chemin.with_untracked(|chemin| Arbre::du_chemin(chemin).unwrap_or(Arbre::CANONIQUE))
}

/// Le chemin courant, dans l'autre arbre.
///
/// C'est ce que pose le sélecteur de « Vous » : basculer d'habillage ne renvoie
/// pas à l'accueil de l'autre arbre, il rend **la même page**. Un réglage qui
/// fait perdre sa place n'est pas un réglage, c'est une sortie.
pub fn ici_dans(vise: Arbre) -> String {
    let chemin = leptos_router::hooks::use_location().pathname;
    chemin.with_untracked(|chemin| crate::domaine::chemins::dans(vise, chemin))
}
