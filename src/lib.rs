//! Le site de La Bible ONT.
//!
//! ## Les couches, et le sens de leurs dépendances
//!
//! ```text
//!   domaine          ──▶  rien
//!   application      ──▶  domaine                  (déclare des ports)
//!   infrastructure   ──▶  application, domaine     (les réalise)
//!   interface        ──▶  application, domaine     (les affiche)
//!   main             ──▶  tout                     (assemble)
//! ```
//!
//! Une flèche ne remonte jamais. Le domaine ignore qu'un serveur existe ;
//! l'application ignore d'où vient l'heure ; l'interface ignore que le vivier
//! est un fichier JSON. C'est ce qui rend chaque couche éprouvable seule.
//!
//! `infrastructure` n'existe que sous le drapeau `ssr` : elle touche au
//! système de fichiers et à l'horloge, deux choses qu'un navigateur n'a pas.

pub mod api;
pub mod application;
pub mod domaine;
pub mod interface;

#[cfg(feature = "ssr")]
pub mod infrastructure;

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
