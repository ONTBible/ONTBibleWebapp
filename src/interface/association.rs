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
//!
//! ## `HEAD` rend un corps vide, et ce n'est pas un défaut d'ici
//!
//! Relevé par la session Android en éprouvant l'accord des liens :
//!
//! ```text
//! GET  /.well-known/assetlinks.json  →  200, application/json, 261 octets
//! HEAD /.well-known/assetlinks.json  →  200, application/json, content-length: 0
//! ```
//!
//! **Rien ne casse** : iOS et Android font tous deux un `GET`, et l'association
//! d'Apple répond de la même façon depuis le premier jour sans que personne ne
//! s'en soit aperçu. C'est le comportement d'axum sur **toutes** ses routes, pas
//! une particularité de celles-ci — le corriger ici en ferait deux exceptions
//! sans rien gagner.
//!
//! Ce qu'il faut savoir, en revanche : **un outil qui sonderait en `HEAD`
//! conclurait à un fichier vide.** C'est le motif qu'on se renvoie depuis deux
//! jours — une réponse parfaitement bien formée à une question qu'on n'avait pas
//! posée. Pour vérifier ce fichier, faire un `GET`.
//!
//! ## La vérification ne part pas de l'appareil, et c'est un quatrième piège
//!
//! **Google va chercher ce fichier depuis ses propres serveurs**, pas depuis le
//! téléphone. Ce fichier peut donc être parfait, l'appareil l'atteindre en
//! douze millisecondes, et la vérification échouer quand même.
//!
//! Relevé le 28 août 2026 sur un Galaxy S20+, dans le journal du système :
//!
//! ```text
//! AppLinksIntentOperation: Verifying requested domains
//! AppLinksAsyncVerifierV2: Error performing check: jgre: UNAVAILABLE
//! ```
//!
//! La session Android avait tout contrôlé avant de conclure — `autoVerify` posé,
//! empreinte installée conforme à celle qu'on publie, domaine joignable, et une
//! autre application `verified` sur le **même** appareil, donc le mécanisme
//! fonctionnant chez lui. C'était le service de Google qui ne répondait pas.
//!
//! **Aucun relevé fait depuis le téléphone ne peut montrer ça.** Quand un lien
//! n'ouvre pas l'app, il faut donc chercher dans cet ordre : le fichier servi en
//! `GET`, l'empreinte installée, puis l'état du service de Google — et non
//! l'inverse, où l'on réécrit un fichier qui n'a jamais été en cause.

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

// ───────────────────────── l'autorisation Android ─────────────────────────────

/// Le nom du paquet Android.
///
/// **Minuscules**, là où le bundle iOS porte `ONT` en capitales. Ce n'est pas
/// une négligence : Android impose la convention du paquet Java, iOS non, et
/// les deux identifiants sont gelés depuis la première publication de chaque
/// plateforme. Relevé dans `android/app/build.gradle.kts` — `applicationId` —
/// et non recopié d'un message.
pub const PAQUET_ANDROID: &str = "com.labibleont.ont";

/// Les empreintes de signature qu'Android acceptera.
///
/// ## Il y en aura une seconde, et il faudra l'**ajouter**
///
/// Celle-ci est la clé de **téléversement**. Avec Play App Signing, Google
/// resigne l'application avec **sa** clé, et c'est celle-là que l'appareil voit
/// en production — son empreinte n'existe qu'après le premier téléversement,
/// dans la console.
///
/// Le tableau en accepte plusieurs, et c'est ce qui rend la suite sûre : le
/// jour où l'empreinte de Google arrive, elle **s'ajoute**. La remplacer ferait
/// cesser d'être reconnues toutes les installations de test, y compris celles
/// des bêta-testeurs — et la panne serait celle du §8 ci-dessus, silencieuse.
pub const EMPREINTES: &[&str] = &[
    // La clé de **téléversement** — celle des versions construites sur une
    // machine de développement et installées par `adb`.
    "AB:BB:7D:95:62:DD:68:09:6D:A3:AB:0A:18:D8:16:E1:17:04:4F:13:A9:C0:0C:03:06:62:5F:F7:54:EB:AF:EE",
    // La clé de **signature de Play** — celle des installations venues du
    // Store, ajoutée le 28 août 2026 après le premier téléversement.
    //
    // Les deux coexistent parce que ce sont **deux chemins de distribution
    // simultanés**, pas deux états d'une même chose : tant qu'on installera des
    // versions locales pour les éprouver, les deux empreintes seront en usage.
    "8C:5C:AB:E4:6E:A5:E7:8E:A8:C1:76:DD:82:3E:00:83:5F:77:CA:93:1B:AC:A9:1B:2B:AB:9F:BE:91:34:6A:1D",
];

/// Le corps de `/.well-known/assetlinks.json`.
///
/// L'équivalent Android du fichier d'Apple, et il porte les **mêmes trois
/// pièges** — `application/json`, aucune redirection, mise en cache par le
/// système. Deux différences comptent :
///
/// * **Android est plus sévère sur la redirection.** Depuis Android 12, un lien
///   web non vérifié ne propose même plus de sélecteur : il part droit au
///   navigateur, sans que rien ne dise pourquoi.
/// * **Il n'y a pas de champ de chemins.** `handle_all_urls` accorde le domaine
///   entier, là où Apple laisse restreindre à `/fr/lire/*`. C'est le filtre
///   d'intention de l'app qui borne, côté Android — donc la borne vit là-bas,
///   et ce fichier ne peut pas la reproduire.
pub fn assetlinks() -> String {
    let empreintes = EMPREINTES
        .iter()
        .map(|e| format!(r#""{e}""#))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        r#"[{{"relation":["delegate_permission/common.handle_all_urls"],"target":{{"namespace":"android_app","package_name":"{PAQUET_ANDROID}","sha256_cert_fingerprints":[{empreintes}]}}}}]"#
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

    /// Le corps Android est du JSON valide, et conforme à ce qu'Android attend.
    ///
    /// La racine est un **tableau**, et ça n'a rien d'accessoire : Android
    /// accepte plusieurs déclarations pour un même domaine — plusieurs apps,
    /// plusieurs paquets. Un objet seul, à la place, est rejeté sans message.
    #[test]
    fn l_autorisation_android_est_du_json_conforme() {
        let valeur: serde_json::Value =
            serde_json::from_str(&assetlinks()).expect("assetlinks.json doit être du JSON");

        let premier = &valeur[0];
        assert_eq!(
            premier["relation"][0], "delegate_permission/common.handle_all_urls",
            "sans cette relation exacte, Android ignore la déclaration"
        );
        assert_eq!(premier["target"]["namespace"], "android_app");
        assert_eq!(premier["target"]["package_name"], PAQUET_ANDROID);

        let empreintes = premier["target"]["sha256_cert_fingerprints"]
            .as_array()
            .expect("les empreintes doivent être un tableau");
        assert_eq!(empreintes.len(), EMPREINTES.len());
        assert!(
            !empreintes.is_empty(),
            "une déclaration sans empreinte n'accorde rien, et ne dit pas qu'elle n'accorde rien"
        );
    }

    /// Chaque empreinte a la forme qu'Android exige.
    ///
    /// Trente-deux octets en hexadécimal majuscule, séparés par des
    /// deux-points. Une empreinte en minuscules, ou copiée sans les
    /// séparateurs, est **acceptée à la lecture et refusée à la
    /// vérification** — le fichier reste du JSON valide, la déclaration reste
    /// bien formée, et le lien n'ouvre simplement jamais l'app.
    ///
    /// C'est la faute la plus probable ici : une empreinte se colle depuis un
    /// terminal, et `keytool` la rend dans un format, la console Play dans un
    /// autre.
    #[test]
    fn les_empreintes_ont_la_forme_attendue() {
        for empreinte in EMPREINTES {
            let octets: Vec<&str> = empreinte.split(':').collect();
            assert_eq!(
                octets.len(),
                32,
                "une empreinte SHA-256 fait 32 octets, celle-ci en a {} : {empreinte}",
                octets.len()
            );
            for octet in octets {
                assert!(
                    octet.len() == 2
                        && octet
                            .chars()
                            .all(|c| c.is_ascii_digit() || c.is_ascii_uppercase()),
                    "« {octet} » n'est pas un octet hexadécimal majuscule dans {empreinte}"
                );
            }
        }
    }

    /// Les deux chemins de distribution restent accordés.
    ///
    /// Une empreinte **retirée** ne casse rien de visible : le fichier reste du
    /// JSON valide, la déclaration reste bien formée, et l'app continue d'ouvrir
    /// les liens — sur la moitié du parc qui porte l'autre clé. L'autre moitié
    /// cesse simplement, sans qu'aucune erreur ne le dise.
    ///
    /// C'est le piège que le commentaire de `EMPREINTES` annonce, et la seule
    /// façon de le tenir est de compter.
    #[test]
    fn les_deux_chemins_de_distribution_restent_declares() {
        assert_eq!(
            EMPREINTES.len(),
            2,
            "il faut **deux** empreintes : la clé de téléversement pour les \
             versions construites à la main, celle de Play pour les \
             installations venues du Store. En retirer une coupe silencieusement \
             les liens sur toute une moitié du parc."
        );
        assert!(
            EMPREINTES
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len()
                == 2,
            "les deux empreintes sont identiques — l'une des deux a été recopiée \
             sur l'autre au lieu d'être ajoutée"
        );
    }

    /// Le paquet reste d'accord avec le dépôt Android.
    ///
    /// Même raison que pour le backend : deux identifiants qui divergent font
    /// une panne muette. Le test lit le dépôt voisin et se tait s'il n'est pas
    /// là — un test d'accord ne peut pas exiger la présence de ce avec quoi il
    /// accorde.
    #[test]
    fn le_paquet_s_accorde_avec_le_depot_android() {
        let gradle = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../ONTBibleApp/android/app/build.gradle.kts"
        );
        let Ok(source) = std::fs::read_to_string(gradle) else {
            return;
        };
        assert!(
            source.contains(&format!("applicationId = \"{PAQUET_ANDROID}\"")),
            "`applicationId` a changé dans le dépôt Android sans changer ici — \
             les liens cesseront d'ouvrir l'app, et rien ne le dira"
        );
    }
}
