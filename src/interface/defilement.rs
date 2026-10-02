//! Remonter en haut quand on change de page.
//!
//! ## Pourquoi il faut l'écrire
//!
//! Un navigateur remonte de lui-même quand il charge un document. Un routeur en
//! SPA ne charge rien : il remplace le contenu et **laisse la page où elle
//! était**. On touche « Lexique » au bas d'un chapitre de vingt-deux mille
//! pixels, et l'on arrive au milieu du lexique, sans titre ni repère.
//!
//! ==Ce qu'un navigateur faisait gratuitement, un routeur doit le refaire à la
//! main — et son absence ne lève aucune erreur.== C'est la famille des défauts
//! du §5 : la page n'est pas cassée, elle est seulement illisible à l'endroit
//! où l'on arrive.
//!
//! ## Trois cas où il ne faut **pas** remonter
//!
//! - **le premier rendu.** Le navigateur vient de poser la page ; remonter
//!   n'ajouterait rien, et écraserait la position qu'il restaure quand on
//!   rouvre un onglet ;
//! - **une adresse qui porte une ancre.** `#installer` demande un endroit
//!   précis : remonter ferait exactement le contraire de ce que le lien dit ;
//! - **un retour en arrière.** Le navigateur rend alors la position d'où l'on
//!   venait, et c'est ce qu'un lecteur attend — remonter lui ferait perdre sa
//!   place pour la seule raison qu'il a voulu la retrouver.
//!
//! > ==Un retour n'est pas une navigation vers une page, c'est une navigation
//! > vers un **moment**.== Le défilement en fait partie.
//!
//! ## Et le saut est instantané
//!
//! `scroll-behavior: smooth` est posé sur `html` (§5), et `scrollTo` en hérite.
//! Sans `instant`, changer de page lancerait un défilement doux sur toute la
//! hauteur du document qu'on vient de quitter — jusqu'à vingt-deux mille pixels
//! pour un chapitre. C'est le piège que `porte.rs` a payé en 2026 : *la position
//! demandée doit redevenir la position obtenue.*

// `Effect` et `window_event_listener` ne servent que du côté du navigateur :
// l'import est borné comme le corps de la fonction, faute de quoi le serveur le
// compile pour rien et le signale.
#[cfg(feature = "hydrate")]
use leptos::prelude::*;

/// Remonte en haut à chaque changement de chemin, sauf aux trois cas ci-dessus.
///
/// ## Une fonction, et pas un composant
///
/// Écrit en composant rendant `view! { <></> }`, il **a tué l'hydratation** :
///
/// ```text
/// the framework expected a marker node, but found this instead:
/// [object HTMLElement]
/// panicked at tachys/src/hydration.rs:216
/// ```
///
/// Le WASM meurt au démarrage, la page reste belle, et plus rien ne répond — le
/// symptôme du §7 bis, pour la cinquième fois dans ce dépôt. Le banc l'a nommé
/// en une passe : dix clics, et l'URL ne bougeait plus de `/fr/liseuse/bible`.
///
/// Un composant qui ne rend rien **n'est pas gratuit** : il occupe une place
/// dans le comptage des marqueurs, et Leptos n'en pose pas les mêmes des deux
/// côtés quand son corps est vide. `fournir_l_arbre()` est appelée dans un bloc
/// `{ }` pour cette raison exacte, et c'est la forme à reprendre.
///
/// ==Ce qui ne rend rien ne doit pas se rendre.== Un effet de bord s'appelle,
/// il ne se monte pas.
pub fn remonter_en_haut() {
    #[cfg(feature = "hydrate")]
    {
        use std::cell::Cell;
        use std::rc::Rc;

        let chemin = leptos_router::hooks::use_location().pathname;

        // **Vrai pendant qu'un retour d'historique se traite.** `popstate` est
        // émis avant que le routeur ne mette le chemin à jour, donc le drapeau
        // est déjà posé quand l'effet s'exécute. Il se consomme aussitôt : deux
        // navigations ordinaires ne doivent pas hériter d'un retour.
        let retour = Rc::new(Cell::new(false));
        {
            let retour = retour.clone();
            let _ = window_event_listener(leptos::ev::popstate, move |_| {
                retour.set(true);
            });
        }

        // Le premier passage de l'effet est le rendu initial : on ne remonte
        // pas une page que le navigateur vient de poser.
        let premier = Cell::new(true);

        Effect::new(move |_| {
            // Lu pour s'abonner, même quand on ne s'en sert pas : un effet qui
            // sort avant sa lecture ne se réveille plus.
            let _ = chemin.get();

            if premier.replace(false) {
                return;
            }
            if retour.replace(false) {
                return;
            }

            let Some(fenetre) = web_sys::window() else {
                return;
            };
            if fenetre
                .location()
                .hash()
                .is_ok_and(|hash| !hash.is_empty() && hash != "#")
            {
                return;
            }

            let options = web_sys::ScrollToOptions::new();
            options.set_top(0.0);
            options.set_left(0.0);
            options.set_behavior(web_sys::ScrollBehavior::Instant);
            fenetre.scroll_to_with_scroll_to_options(&options);
        });

        // ── Toucher la page où l'on est déjà remonte aussi ────────────────
        //
        // C'est le geste d'iOS, et l'app le fait : on touche l'onglet courant,
        // la vue revient en haut. Sur le site, le chemin ne change pas — donc
        // l'effet ci-dessus ne se réveille jamais, et le lien ne fait
        // visiblement **rien**. Un lien qui ne fait rien se lit comme un lien
        // cassé, pas comme un lien sans objet.
        //
        // ==Le seul geste qui n'a pas d'effet par construction est celui qui
        // demande là où l'on est ; c'est précisément celui à qui il faut en
        // donner un.==
        //
        // ## Trois conditions, et chacune écarte un faux positif
        //
        // - **même hôte**, sinon on remonterait en partant du site ;
        // - **même chemin *et* même chaîne de requête.** `…/bereshit-1?v=5`
        //   depuis `…/bereshit-1` est un autre endroit, et le routeur ne change
        //   pourtant pas le `pathname` : comparer le seul chemin ferait remonter
        //   un lecteur qu'on envoie au verset cinq ;
        // - **pas d'ancre**, qui demande explicitement un autre endroit.
        //
        // ## Et celui-ci est **doux**, à l'inverse du précédent
        //
        // On ne quitte pas la page : on la voit revenir, et ce mouvement est ce
        // qui dit que le geste a été pris. La feuille porte déjà
        // `scroll-behavior: smooth` sur `html` et le repasse à `auto` sous
        // `prefers-reduced-motion` (§5) — il suffit donc de **ne rien forcer**,
        // et le réglage du lecteur est respecté sans qu'on ait à le relire.
        // ## Il a fallu deux fausses pistes pour arriver à cette ligne
        //
        // Le banc rendait `y=2000` après le clic, sans erreur. On a d'abord
        // soupçonné **le délai** — le mouvement est doux, on l'a cru inachevé —,
        // puis la **propagation** : le routeur traite le clic d'un `<A>`, il
        // pouvait l'arrêter avant `window`. L'écouteur est donc passé en phase
        // de capture, avec un commentaire qui expliquait très bien pourquoi.
        //
        // Ni l'un ni l'autre. **C'était le banc** : il descendait à 2000 px par
        // un `scrollTo` qui hérite du `scroll-behavior: smooth` de la feuille,
        // donc sa descente était encore en cours quand le clic partait. Les deux
        // animations se battaient, et le relevé final rendait la position que
        // l'instrument lui-même défendait — le piège de `porte.rs`, rejoué par
        // l'outil au lieu du produit.
        //
        // La capture a ensuite été **retirée et mesurée à nouveau** : la bulle
        // suffit. ==Une correction qui accompagne un succès n'est pas une
        // correction prouvée ; il faut la retirer pour savoir.== Sans ce
        // retour en arrière, le dépôt aurait gardé une complexité et une raison
        // fausse — et la raison aurait été relue comme une mesure.
        let _ = window_event_listener(leptos::ev::click, move |evenement| {
            use wasm_bindgen::JsCast;

            let Some(ancre) = evenement
                .target()
                .and_then(|cible| cible.dyn_into::<web_sys::Element>().ok())
                .and_then(|element| element.closest("a").ok().flatten())
                .and_then(|element| element.dyn_into::<web_sys::HtmlAnchorElement>().ok())
            else {
                return;
            };

            let Some(fenetre) = web_sys::window() else {
                return;
            };
            let ici = fenetre.location();

            let meme_hote = ancre.host() == ici.host().unwrap_or_default();
            let meme_chemin = ancre.pathname() == ici.pathname().unwrap_or_default();
            let meme_requete = ancre.search() == ici.search().unwrap_or_default();
            let sans_ancre = ancre.hash().is_empty() || ancre.hash() == "#";

            if meme_hote && meme_chemin && meme_requete && sans_ancre {
                fenetre.scroll_to_with_x_and_y(0.0, 0.0);
            }
        });
    }
}
