//! Le sens d'une navigation — empilement ou dépilement.
//!
//! ## Ce qu'iOS fait, et qu'un site ne fait pas tout seul
//!
//! Une `NavigationStack` pousse l'écran qui arrive **depuis la droite** pendant
//! que le précédent glisse un peu vers la gauche ; le retour joue l'inverse. Ce
//! n'est pas un ornement : c'est ce qui dit qu'on **descend** dans une
//! arborescence plutôt qu'on change de sujet, et c'est ce qui rend le geste de
//! retour prévisible.
//!
//! Un navigateur remplace la page. Sans rien, on saute d'un écran à l'autre —
//! ce qui se lit comme un rechargement, c'est-à-dire exactement ce que la
//! webapp n'est pas.
//!
//! ## Ce qui se reproduit, et ce qui ne se reproduit pas
//!
//! **L'écran qui arrive** glisse dans le bon sens, au ressort de l'app.
//!
//! **Celui qui part, non** : Leptos l'a déjà démonté quand le suivant se monte,
//! et rien ne le garde pour l'animer. Le faire demanderait de tenir deux arbres
//! vivants à la fois — donc deux pages de corpus en mémoire, deux hydratations,
//! et un `Suspense` qui ne sait plus lequel il suspend.
//!
//! C'est un écart assumé, et il est petit : l'œil suit ce qui **entre**. Le
//! jour où les *view transitions* seront partout, elles rendront la seconde
//! moitié sans rien de tout ça — elles photographient l'ancien état.
//!
//! ## Le sens se déduit de la profondeur
//!
//! Aucun routeur du web ne dit « ceci est un empilement » : il n'a qu'une pile
//! plate et chronologique. On compare donc les segments :
//!
//!     /fr/webapp            →  /fr/webapp/bereshit    on descend
//!     /fr/webapp/bereshit   →  /fr/webapp             on remonte
//!     /fr/webapp            →  /fr/lexique            on change d'onglet
//!
//! **Le changement d'onglet n'est ni l'un ni l'autre**, et il ne doit pas
//! l'être : iOS y fait un fondu, pas un glissement. Glisser entre deux onglets
//! dirait une hiérarchie qui n'existe pas — ils sont côte à côte, pas l'un
//! sous l'autre.

use leptos::prelude::*;

/// Ce qu'une navigation vient de faire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sens {
    /// On descend d'un cran — l'écran arrive par la droite.
    Empile,
    /// On remonte — il arrive par la gauche.
    Depile,
    /// On change d'onglet, ou l'on arrive : un fondu, sans direction.
    Fondu,
}

impl Sens {
    /// La classe qui porte son animation.
    pub fn classe(self) -> &'static str {
        match self {
            Sens::Empile => "empile",
            Sens::Depile => "depile",
            Sens::Fondu => "arrivee",
        }
    }
}

/// Le sens de la dernière navigation, lisible par la page qui arrive.
///
/// **Un signal global et non un contexte de page** : la page qui arrive ne
/// connaît pas celle qui part, et c'est justement la comparaison qui donne le
/// sens. Seul quelque chose qui survit aux deux peut la faire.
pub fn sens() -> Signal<Sens> {
    use_context::<RwSignal<Sens>>()
        .map(Into::into)
        .unwrap_or_else(|| Signal::stored(Sens::Fondu))
}

/// Installe le calcul du sens — **monté dans le `<Router>`**, pas avant.
///
/// C'est un composant et non un appel de fonction, et la raison n'est pas de
/// style : `use_location()` lit un contexte que le routeur fournit, donc il
/// **panique hors de lui**. Appelé en tête d'`App`, il tuait le serveur au
/// démarrage — *« Tried to access Location outside a `<Router>` »*.
///
/// Un composant ne peut pas se tromper de place : il est là où on l'écrit dans
/// la vue, c'est-à-dire forcément sous le routeur qui l'entoure.
#[component]
pub fn SuiviDuSens() -> impl IntoView {
    suivre_le_sens();
    view! { <></> }
}

/// Installe le calcul du sens. Voir [`SuiviDuSens`], qui le monte au bon
/// endroit.
fn suivre_le_sens() {
    let courant = RwSignal::new(Sens::Fondu);
    provide_context(courant);

    let chemin = leptos_router::hooks::use_location().pathname;
    let precedent = StoredValue::new(String::new());

    Effect::new(move |_| {
        let vers = chemin.get();
        let depuis = precedent.get_value();
        precedent.set_value(vers.clone());
        if depuis.is_empty() {
            return;
        }
        courant.set(comparer(&depuis, &vers));
    });
}

/// Le sens, des segments de l'un vers ceux de l'autre.
///
/// Deux chemins n'ayant pas la **même tête** sont deux onglets : on ne descend
/// ni ne remonte, on se déplace de côté. Sinon, c'est la profondeur qui dit.
fn comparer(depuis: &str, vers: &str) -> Sens {
    let tete = |c: &str| {
        c.trim_matches('/')
            .split('/')
            .take(2)
            .collect::<Vec<_>>()
            .join("/")
    };
    if tete(depuis) != tete(vers) {
        return Sens::Fondu;
    }
    let profondeur = |c: &str| c.trim_matches('/').split('/').count();
    match profondeur(vers).cmp(&profondeur(depuis)) {
        std::cmp::Ordering::Greater => Sens::Empile,
        std::cmp::Ordering::Less => Sens::Depile,
        // Même profondeur sous le même onglet : d'un livre à un autre, d'un
        // chapitre à son voisin. Ce n'est pas une descente — un fondu.
        std::cmp::Ordering::Equal => Sens::Fondu,
    }
}

#[cfg(test)]
mod tests {
    use super::{comparer, Sens};

    #[test]
    fn descendre_empile_et_remonter_depile() {
        assert_eq!(comparer("/fr/webapp", "/fr/webapp/bereshit"), Sens::Empile);
        assert_eq!(
            comparer("/fr/webapp/bereshit", "/fr/webapp/bereshit/bereshit-3"),
            Sens::Empile
        );
        assert_eq!(comparer("/fr/webapp/bereshit", "/fr/webapp"), Sens::Depile);
    }

    /// **Changer d'onglet n'est pas descendre.**
    ///
    /// iOS y fait un fondu, et c'est juste : deux onglets sont côte à côte, pas
    /// l'un sous l'autre. Un glissement y dirait une hiérarchie qui n'existe
    /// pas — et il dirait la mauvaise à la moitié des bascules, `lexique` étant
    /// plus court que `webapp/bereshit` sans être au-dessus de lui.
    #[test]
    fn changer_d_onglet_ne_glisse_pas() {
        assert_eq!(comparer("/fr/webapp", "/fr/lexique"), Sens::Fondu);
        assert_eq!(
            comparer("/fr/webapp/bereshit/bereshit-3", "/fr/lexique"),
            Sens::Fondu
        );
        assert_eq!(comparer("/fr/qahal", "/fr/compte"), Sens::Fondu);
    }

    /// Deux frères ne s'empilent pas.
    #[test]
    fn d_un_voisin_a_l_autre_c_est_un_fondu() {
        assert_eq!(
            comparer("/fr/webapp/bereshit", "/fr/webapp/chanokh"),
            Sens::Fondu
        );
        assert_eq!(
            comparer("/fr/lexique/adam", "/fr/lexique/bara"),
            Sens::Fondu
        );
    }
}
