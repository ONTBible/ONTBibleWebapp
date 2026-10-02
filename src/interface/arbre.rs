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

/// Sommes-nous sous un arbre — c'est-à-dire dans la liseuse, au sens large ?
///
/// **Fonction du chemin seul**, et c'est ce qui la rend sûre : la condition qui
/// la précédait lisait un compteur incrémenté au montage de la page, si bien
/// qu'elle valait zéro au moment où la racine l'interrogeait. Le serveur et le
/// client n'y répondaient pas la même chose au même instant — la définition d'un
/// désaccord d'hydratation.
///
/// ==Une condition de rendu qui dépend d'un ordre de montage n'est pas une
/// condition, c'est une course.==
pub fn dans_un_arbre() -> Signal<bool> {
    let chemin = leptos_router::hooks::use_location().pathname;
    Signal::derive(move || chemin.with(|c| Arbre::du_chemin(c).is_some()))
}

/// Sommes-nous sous cet arbre-là précisément ?
pub fn sous_l_arbre(vise: Arbre) -> Signal<bool> {
    let chemin = leptos_router::hooks::use_location().pathname;
    Signal::derive(move || chemin.with(|c| Arbre::du_chemin(c) == Some(vise)))
}

/// Sommes-nous sous l'arbre de l'**édition** — la liseuse du site ?
///
/// Lu une fois, sans s'abonner : ce qui en dépend se décide au montage, et une
/// page ne change pas d'arbre sans se remonter.
///
/// C'est la question que posent les pages pour savoir quel registre composer :
/// un rappel en capitales et un chapeau en prose sous l'édition, un grand titre
/// serré et rien d'autre sous l'app. ==Le registre se demande une fois, en haut
/// de la page ; il ne se redérive pas à chaque prop.==
pub fn sous_l_edition() -> bool {
    arbre_maintenant() == Arbre::Liseuse
}

/// Le premier maillon du fil d'Ariane — l'entrée du corpus, nommée.
///
/// ## Pourquoi un composeur et non six littéraux
///
/// Le chemin et le **nom** changent ensemble : sous l'app c'est « Bible », le mot
/// de sa barre d'onglets ; sous l'édition c'est « Lire », le mot de la navigation
/// du site — celui que le lecteur vient de toucher pour arriver là.
///
/// Les six endroits qui posent ce maillon l'écrivaient à la main, et le nom y
/// était figé. ==Deux valeurs qui changent ensemble se composent au même
/// endroit ; écrites côte à côte six fois, elles divergent à la première
/// retouche — et le fil est exactement ce que personne ne relit.==
pub fn maillon_de_la_bible() -> (String, String) {
    let ici = arbre_maintenant();
    let nom = if ici == Arbre::Liseuse { "Lire" } else { "Bible" };
    (crate::domaine::chemins::bible(ici), nom.to_string())
}
