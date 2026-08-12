//! Le site de La Bible ONT.
//!
//! ## Les couches, et le sens de leurs dépendances
//!
//! ```text
//!   interface  ──▶  domaine        (les pages lisent le domaine)
//!   serveur    ──▶  interface      (le serveur rend les pages)
//!   serveur    ──▶  domaine
//!   domaine    ──▶  rien
//! ```
//!
//! Le domaine ne connaît ni le serveur ni le navigateur : il compile vers
//! `wasm32` comme vers Lambda, et c'est ce qui garantit qu'un calcul donne le
//! même résultat des deux côtés.
//!
//! `serveur` n'existe que sous le drapeau `ssr` : il touche au système de
//! fichiers et au réseau, deux choses qu'un navigateur n'a pas.

pub mod domaine;
pub mod interface;

/// Le point d'entrée du navigateur.
///
/// Le serveur a déjà rendu le HTML ; cette fonction ne le reconstruit pas, elle
/// rattache les gestionnaires d'événements au DOM existant. C'est tout
/// l'intérêt du SSR : la page est lisible avant que ce code n'arrive — et elle
/// le reste s'il n'arrive jamais.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::interface::app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
