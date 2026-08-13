//! `/.well-known/apple-app-site-association` — l'autorisation donnée à l'app.
//!
//! Ce n'est pas une page : personne ne la lit. C'est le fichier que iOS va
//! chercher pour savoir si l'app a le droit d'ouvrir les liens `ontbible.com`.
//! Il est ici parce qu'il est servi, comme une page l'est — c'est le bord
//! extérieur du site, pas sa mise en forme.
//!
//! ## Pourquoi ce fichier est dangereux
//!
//! Il est aujourd'hui rendu par la **Lambda de l'API**, qui répond encore à la
//! racine du domaine. Le jour où le site prend cette racine, il doit le rendre
//! à sa place — sinon les liens partagés cessent d'ouvrir l'app, et rien ne le
//! signale.
//!
//! Trois pièges, tous silencieux :
//!
//! * il doit sortir en **`application/json`** — servi en `text/plain`, iOS
//!   l'ignore sans un mot ;
//! * **aucune redirection** n'est tolérée sur ce chemin, pas même un `/` final
//!   normalisé — d'où une route posée avant le routeur de Leptos ;
//! * il n'est relu **qu'à l'installation ou à la mise à jour de l'app**, et
//!   Apple le met en cache derrière son propre CDN. Une erreur ici ne se voit
//!   pas le jour où on la commet ; elle se voit des semaines plus tard, chez
//!   quelqu'un qui vient d'installer l'app.
//!
//! C'est cette invisibilité qui justifie le test plus bas. Un fichier qu'on ne
//! consulte jamais et dont la casse est différée doit être tenu par une
//! machine.

/// L'identifiant d'équipe et le bundle, tels qu'Apple les attend.
///
/// Repris **tel quel** de `ONTBibleApp/backend/src/interface/web.rs`. Le bundle
/// porte le nom français (`labibleont`) alors que le domaine nomme le projet
/// (`ontbible.com`) : c'est délibéré et gelé. Un identifiant d'app se fixe à la
/// première publication ; le changer coûterait de refaire toute la
/// configuration Sign in with Apple.
pub const APP_ID: &str = "N49VNC2G57.com.labibleont.ONT";

/// Le chemin que l'app a le droit d'ouvrir.
///
/// `/fr/lire/*` **seulement**. Le reste du domaine — l'accueil, le pourquoi,
/// les pages légales — doit rester consultable dans un navigateur : quelqu'un
/// qui a l'app installée et qui clique sur un lien vers la page d'accueil veut
/// voir le site, pas se faire enlever vers l'app.
pub const CHEMINS: &str = "/fr/lire/*";

/// Le corps du fichier.
///
/// Écrit à la main plutôt que sérialisé : c'est un document figé de trois
/// lignes, dont la forme exacte est imposée par Apple. Une structure `serde`
/// ajouterait un étage d'indirection sans rien garantir de plus — le test, lui,
/// garantit quelque chose.
pub fn corps() -> String {
    format!(
        r#"{{"applinks":{{"details":[{{"appIDs":["{APP_ID}"],"components":[{{"/":"{CHEMINS}"}}]}}]}}}}"#
    )
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    /// Le corps est du JSON valide, et il porte ce qu'Apple attend là où Apple
    /// l'attend.
    ///
    /// On navigue la structure au lieu de comparer deux chaînes : une
    /// comparaison littérale casserait au premier espace ajouté, ce qui
    /// donnerait un test qui crie sans qu'il y ait de faute.
    #[test]
    fn le_corps_est_du_json_conforme() {
        let valeur: serde_json::Value =
            serde_json::from_str(&corps()).expect("le fichier d'association doit être du JSON");

        let details = &valeur["applinks"]["details"][0];
        assert_eq!(details["appIDs"][0], APP_ID);
        assert_eq!(details["components"][0]["/"], CHEMINS);
    }

    /// Le fichier doit rester d'accord avec le backend de l'app, qui le sert
    /// aujourd'hui. Les deux diverger, c'est exactement la panne qu'on redoute.
    ///
    /// Le test lit le dépôt voisin. S'il n'est pas là — clone partiel, machine
    /// de compilation — il passe : un test d'accord ne peut pas exiger la
    /// présence de ce avec quoi il accorde.
    #[test]
    fn l_identifiant_s_accorde_avec_le_backend() {
        let backend = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../ONTBibleApp/backend/src/interface/web.rs"
        );
        let Ok(source) = std::fs::read_to_string(backend) else {
            return;
        };
        assert!(
            source.contains(APP_ID),
            "l'identifiant d'app a changé dans le backend sans changer ici"
        );
        assert!(
            source.contains(CHEMINS),
            "les chemins associés ont changé dans le backend sans changer ici"
        );
    }
}
