//! Les trois routes du compte, et le cookie qui garde la session.
//!
//! ## Pourquoi le jeton n'entre jamais dans une page
//!
//! Tout se passe côté serveur : c'est la Lambda du site qui conduit le lecteur
//! chez le fournisseur, reçoit le code au retour, et appelle le backend. La
//! session repart dans un cookie **`HttpOnly`**, qu'aucun script de la page ne
//! peut lire.
//!
//! Le backend autorise pourtant `allow_origins = ["*"]`, donc le navigateur
//! *pourrait* l'appeler directement. On s'en garde : un jeton posé dans
//! `localStorage` est lisible par n'importe quel script chargé ensuite — une
//! dépendance compromise, une extension, une injection. Celui-ci ne l'est par
//! aucun, et le prix est un aller-retour de plus vers notre propre serveur.
//!
//! ## Les trois routes
//!
//! | route | ce qu'elle fait |
//! |---|---|
//! | `/fr/compte/aller/{fournisseur}` | fabrique PKCE, pose le cookie d'aller, redirige |
//! | `/fr/compte/retour` | reçoit le code, échange, pose la session |
//! | `/fr/compte/partir` | efface les cookies |
//!
//! Elles sont posées **avant** le routeur de Leptos, comme l'association d'app :
//! ce sont des redirections et des en-têtes, pas des pages.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};

use crate::application::ports::Comptes;
use crate::domaine::compte::Fournisseur;
use crate::infrastructure::comptes::Pkce;

/// Le cookie qui porte la session, une fois le compte ouvert.
pub const COOKIE_SESSION: &str = "ont_session";
/// Le cookie qui porte le vérificateur PKCE, le temps de l'aller-retour.
const COOKIE_ALLER: &str = "ont_aller";

/// Où le fournisseur renvoie le lecteur.
///
/// **Doit être identique à l'octet** entre l'aller et l'échange : le
/// fournisseur la recompare, et un `/` final de différence donne un
/// `invalid_grant` dont le message ne dit pas lequel des deux diffère. D'où une
/// constante, plutôt que deux chaînes composées à deux endroits.
pub fn adresse_de_retour() -> String {
    format!("{}/fr/compte/retour", crate::interface::tete::ORIGINE)
}

/// L'adresse d'autorisation de chaque fournisseur.
///
/// Relevées dans leur documentation, pas devinées. Google et GitHub prennent
/// PKCE ; Apple demande en plus `response_mode=form_post` dès qu'on réclame des
/// portées — ce que nous ne faisons pas, donc `code` suffit.
fn autorisation(f: Fournisseur) -> &'static str {
    match f {
        Fournisseur::Google => "https://accounts.google.com/o/oauth2/v2/auth",
        Fournisseur::Apple => "https://appleid.apple.com/auth/authorize",
        Fournisseur::Github => "https://github.com/login/oauth/authorize",
    }
}

/// Ce que chaque fournisseur veut qu'on lui demande.
///
/// Le strict minimum : on ne veut ni le nom, ni la photo, ni les contacts. Le
/// backend n'a besoin que d'un identifiant stable pour rattacher les
/// surlignages — et une portée qu'on ne demande pas est une donnée qu'on n'aura
/// jamais à protéger.
fn portees(f: Fournisseur) -> &'static str {
    match f {
        Fournisseur::Google => "openid email",
        Fournisseur::Apple => "email",
        // GitHub n'a pas d'`openid` : sa portée vide rend déjà le profil public
        // et l'identifiant, ce qui suffit.
        Fournisseur::Github => "",
    }
}

/// L'identifiant client du **site**, par fournisseur.
///
/// ## Ce ne sont pas ceux de l'app, et c'est obligatoire
///
/// L'app iOS signe avec `com.labibleont.ONT` — un identifiant d'App ID, qu'Apple
/// n'accorde qu'à un flux natif. Le README du backend l'écrit : « Il ne
/// redeviendra nécessaire [un Services ID] que le jour où une version web
/// signera des comptes. » C'est ce jour-là.
///
/// **Un identifiant client n'est pas un secret** : il voyage en clair dans
/// l'adresse d'autorisation. Le secret, lui, ne quitte pas la Lambda du backend.
/// Le poser ici est donc correct, et le cacher n'apporterait rien qu'une
/// difficulté de plus au déploiement.
///
/// `None` signifie « pas encore déclaré chez le fournisseur ». La page de
/// compte n'affiche alors pas le bouton : mieux vaut une voie de moins qu'une
/// voie qui mène à une erreur du fournisseur, où le lecteur ne peut rien faire.
/// Le Services ID du **site**, créé le 27 août 2026.
///
/// Distinct de l'App ID `com.labibleont.ONT`, qu'Apple réserve au flux natif :
/// « The client_id used when calling the token endpoint should match the native
/// app's app id. The services ID should not be used here. » L'inverse est vrai
/// aussi — un navigateur veut le Services ID, et l'App ID y échouerait.
///
/// Il est écrit ici bien qu'inutilisé pour l'instant : le perdre coûterait de le
/// retrouver dans un portail, et **un Services ID ne se renomme pas** une fois
/// créé.
pub const SERVICES_ID_APPLE: &str = "com.labibleont.ont.webapp";

fn identifiant_client(f: Fournisseur) -> Option<&'static str> {
    match f {
        // Le client « Application Web » du backend, relevé dans
        // `ONTBibleApp/app/project.yml`. Il accepte plusieurs adresses de
        // retour : il suffit d'y ajouter celle du site.
        Fournisseur::Google => {
            Some("154337904456-de9o2u3res51203irei6o0ggk1lvlkq5.apps.googleusercontent.com")
        }
        // Le Services ID, créé dans le portail Apple le 27 août 2026 —
        // `com.labibleont.ont.webapp`, avec `ontbible.com` en domaine et notre
        // adresse de retour.
        //
        // **Allumé le 30 août 2026, après sonde.** Le backend distingue les deux
        // flux depuis longtemps — il choisit le Services ID pour l'origine
        // `webapp`, signe le secret client avec cette identité, et n'envoie
        // `redirect_uri` que dans ce cas. Ce qui manquait n'était pas son code
        // mais la **valeur** dans sa configuration déployée, absente de
        // `oauth.env`, et dont le script de déploiement tolérait l'absence par
        // un `${APPLE_SERVICES_ID:-}` : elle traversait tout en silence et
        // arrivait vide sur la Lambda.
        //
        // Ce n'est pas allumé sur la parole de qui l'a posée, mais sur
        // `./scripts/sonder-les-fournisseurs.py`, qui a rendu :
        //
        //     apple 401 servi · le site le dit éteint → il peut être rallumé
        //
        // Le `401` dit que la requête est allée jusqu'à Apple, qui a refusé un
        // code bidon. Le `503` d'avant disait qu'elle n'était jamais partie.
        Fournisseur::Apple => Some(SERVICES_ID_APPLE),
        // L'application `La Bible ONT` existante, à laquelle
        // `https://ontbible.com/fr/compte/retour` a été ajoutée : GitHub accepte
        // plusieurs adresses de retour, contrairement à ce qu'on avait cru.
        // L'identifiant n'est pas un secret ; le secret reste au backend.
        //
        // **Éteint le 30 août 2026, et il était allumé en production.**
        //
        // Le backend distingue bien les deux origines — mais pour GitHub il
        // attend deux **applications** distinctes, `github` et `github_web`, et
        // la seconde n'est pas configurée sur la Lambda déployée. Mesuré, même
        // requête et même code bidon, seule l'origine changeant :
        //
        //     origine app      401  « connexion refusée »        configuré
        //     origine webapp   503  « fournisseur non configuré »
        //
        // Un lecteur qui cliquait partait donc chez GitHub, autorisait, revenait
        // — et tombait sur une erreur où il ne pouvait rien faire. C'est le
        // défaut du badge App Store, en production cette fois.
        //
        // Il se rallume en une ligne le jour où le backend porte
        // `GITHUB_WEB_CLIENT_ID` et son secret. Ne pas le rallumer sans avoir
        // **resondé** : le code du backend sait déjà le faire, c'est sa
        // configuration qui décide, et aucune des deux ne se voit d'ici.
        Fournisseur::Github => None,
    }
}

/// Le fait et la mécanique doivent s'accorder, et un test le tient.
///
/// `compte_public::disponible` décide ce que la page affiche ; `identifiant_client`
/// décide ce que la route sait faire. S'ils divergent, on obtient soit un bouton
/// qui mène à une erreur, soit une route utilisable que personne ne voit.

/// Un cookie de session, avec les quatre attributs qui comptent.
///
/// **`HttpOnly`** — aucun script ne le lit, c'est tout l'intérêt du montage.
/// **`Secure`** — jamais en clair sur le réseau.
/// **`SameSite=Lax`** — il voyage sur une navigation venue d'ailleurs, ce qu'il
/// faut pour le retour du fournisseur, mais pas sur une requête de fond ; c'est
/// ce qui coupe la falsification de requête.
/// **`Path=/`** — la session vaut pour tout le site, pas seulement pour la page
/// qui l'a posée.
fn cookie(nom: &str, valeur: &str, duree_s: i64) -> String {
    format!("{nom}={valeur}; Max-Age={duree_s}; Path=/; HttpOnly; Secure; SameSite=Lax")
}

/// Le même cookie, vidé — c'est ainsi qu'on en efface un.
fn cookie_efface(nom: &str) -> String {
    format!("{nom}=; Max-Age=0; Path=/; HttpOnly; Secure; SameSite=Lax")
}

/// Lit un cookie dans l'en-tête, ou rien.
pub fn lire_cookie(entetes: &axum::http::HeaderMap, nom: &str) -> Option<String> {
    entetes
        .get(header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .filter_map(|morceau| morceau.trim().split_once('='))
        .find(|(cle, _)| *cle == nom)
        .map(|(_, valeur)| valeur.to_string())
}

/// `/fr/compte/aller/{fournisseur}` — on part chez le fournisseur.
pub async fn aller(Path(cle): Path<String>) -> Response {
    let Some(fournisseur) = Fournisseur::depuis_cle(&cle) else {
        return redirige("/fr/compte?erreur=fournisseur", None);
    };
    let Some(client) = identifiant_client(fournisseur) else {
        return redirige("/fr/compte?erreur=indisponible", None);
    };

    let pkce = Pkce::neuf();
    let retour = adresse_de_retour();

    // Le fournisseur est mis dans le cookie d'aller plutôt que dans l'adresse
    // de retour : celle-ci doit rester **identique à l'octet** entre l'aller et
    // l'échange, donc on ne peut rien y ajouter. Et un paramètre `state` que
    // l'on relirait sans le comparer ne protégerait de rien.
    let aller = format!("{}|{}", fournisseur.cle(), pkce.verificateur);

    let mut adresse = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code",
        autorisation(fournisseur),
        encoder(client),
        encoder(&retour),
    );
    let p = portees(fournisseur);
    if !p.is_empty() {
        adresse.push_str(&format!("&scope={}", encoder(p)));
    }
    if fournisseur.exige_pkce() {
        adresse.push_str(&format!(
            "&code_challenge={}&code_challenge_method=S256",
            encoder(&pkce.defi)
        ));
    }

    // Dix minutes : le temps de se connecter chez le fournisseur, pas celui
    // d'oublier l'onglet ouvert. Au-delà, le code serait de toute façon périmé
    // chez lui.
    redirige(&adresse, Some(cookie(COOKIE_ALLER, &aller, 600)))
}

/// `/fr/compte/retour` — le fournisseur nous renvoie le lecteur.
pub async fn retour(
    State(comptes): State<Arc<dyn Comptes>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
    entetes: axum::http::HeaderMap,
) -> Response {
    // Un refus n'est pas une panne : le lecteur a pu changer d'avis sur l'écran
    // du fournisseur. On revient sans rien dire de plus.
    if params.contains_key("error") {
        return redirige("/fr/compte?erreur=refus", Some(cookie_efface(COOKIE_ALLER)));
    }
    let Some(code) = params.get("code") else {
        return redirige(
            "/fr/compte?erreur=reponse",
            Some(cookie_efface(COOKIE_ALLER)),
        );
    };

    // Sans le cookie d'aller, ce retour ne vient pas d'un départ que nous avons
    // conduit : on refuse. C'est ce qui empêche qu'un lien fabriqué ouvre une
    // session chez quelqu'un qui l'a simplement cliqué.
    let Some(aller) = lire_cookie(&entetes, COOKIE_ALLER) else {
        return redirige("/fr/compte?erreur=expire", None);
    };
    let Some((cle, verificateur)) = aller.split_once('|') else {
        return redirige(
            "/fr/compte?erreur=expire",
            Some(cookie_efface(COOKIE_ALLER)),
        );
    };
    let Some(fournisseur) = Fournisseur::depuis_cle(cle) else {
        return redirige(
            "/fr/compte?erreur=fournisseur",
            Some(cookie_efface(COOKIE_ALLER)),
        );
    };

    let verificateur = fournisseur.exige_pkce().then_some(verificateur);
    match comptes
        .ouvrir(fournisseur, code, &adresse_de_retour(), verificateur)
        .await
    {
        Ok(session) => {
            let Ok(serialisee) = serde_json::to_string(&session) else {
                return redirige(
                    "/fr/compte?erreur=interne",
                    Some(cookie_efface(COOKIE_ALLER)),
                );
            };
            // Le cookie vit soixante jours — la durée du jeton de
            // rafraîchissement, pas celle du jeton d'accès. Le second se
            // renouvelle tout seul tant que le premier vaut ; le caler sur une
            // heure déconnecterait le lecteur chaque heure.
            let session_cookie = cookie(COOKIE_SESSION, &encoder(&serialisee), 60 * 24 * 3600);
            (
                StatusCode::FOUND,
                [
                    (header::LOCATION, "/fr/compte".to_string()),
                    (header::SET_COOKIE, cookie_efface(COOKIE_ALLER)),
                    (header::SET_COOKIE, session_cookie),
                ],
            )
                .into_response()
        }
        Err(erreur) => {
            let quoi = match erreur {
                crate::application::ports::ErreurDeCompte::Refuse => "refus",
                crate::application::ports::ErreurDeCompte::Indisponible => "indisponible",
                crate::application::ports::ErreurDeCompte::ContratRompu(_) => "interne",
            };
            redirige(
                &format!("/fr/compte?erreur={quoi}"),
                Some(cookie_efface(COOKIE_ALLER)),
            )
        }
    }
}

/// `/fr/compte/partir` — on se déconnecte.
///
/// Le cookie est effacé, et rien n'est dit au backend : le jeton d'accès est
/// **irrévocable** par construction — c'est pour ça qu'il ne vit qu'une heure.
/// Prétendre le révoquer donnerait une fausse assurance.
pub async fn partir() -> Response {
    redirige("/fr", Some(cookie_efface(COOKIE_SESSION)))
}

fn redirige(vers: &str, cookie: Option<String>) -> Response {
    let mut reponse = (StatusCode::FOUND, [(header::LOCATION, vers.to_string())]).into_response();
    if let Some(c) = cookie {
        if let Ok(valeur) = c.parse() {
            reponse.headers_mut().append(header::SET_COOKIE, valeur);
        }
    }
    reponse
}

/// Encodage d'un composant d'adresse.
///
/// Repris du backend, à la lettre, plutôt qu'une dépendance de plus : la liste
/// des signes sûrs est celle de la RFC 3986, et elle ne bouge pas.
fn encoder(valeur: &str) -> String {
    valeur
        .bytes()
        .map(|octet| match octet {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (octet as char).to_string()
            }
            _ => format!("%{octet:02X}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ce que la page propose est exactement ce que la route sait faire.
    ///
    /// Les deux listes vivent dans deux modules — l'une doit voyager jusqu'au
    /// navigateur, l'autre porte des identifiants et reste au serveur. Deux
    /// listes finissent toujours par diverger ; celle-ci ne peut plus.
    #[test]
    fn la_page_et_la_route_s_accordent_sur_les_fournisseurs() {
        for f in Fournisseur::tous() {
            assert_eq!(
                crate::interface::compte_public::disponible(f),
                identifiant_client(f).is_some(),
                "{} : la page et la route ne disent pas la même chose",
                f.nom()
            );
        }
    }

    #[test]
    fn le_cookie_porte_les_quatre_attributs_qui_comptent() {
        let c = cookie("x", "y", 60);
        for attendu in ["HttpOnly", "Secure", "SameSite=Lax", "Path=/"] {
            assert!(c.contains(attendu), "« {attendu} » manque à {c}");
        }
    }

    /// Effacer, c'est reposer le même cookie vide et périmé.
    ///
    /// Les attributs doivent être **les mêmes** qu'à la pose : un navigateur
    /// qui ne les reconnaît pas laisse l'ancien en place, et la déconnexion
    /// n'a lieu qu'en apparence.
    #[test]
    fn effacer_repose_les_memes_attributs() {
        let e = cookie_efface(COOKIE_SESSION);
        assert!(e.contains("Max-Age=0"));
        for attendu in ["HttpOnly", "Secure", "SameSite=Lax", "Path=/"] {
            assert!(e.contains(attendu), "« {attendu} » manque à {e}");
        }
    }

    #[test]
    fn on_relit_le_cookie_qu_on_cherche() {
        let mut entetes = axum::http::HeaderMap::new();
        entetes.insert(
            header::COOKIE,
            "autre=1; ont_session=abc; encore=2".parse().unwrap(),
        );
        assert_eq!(
            lire_cookie(&entetes, COOKIE_SESSION).as_deref(),
            Some("abc")
        );
        assert_eq!(lire_cookie(&entetes, "absent"), None);
    }

    /// L'adresse de retour est absolue et unique.
    ///
    /// Le fournisseur la recompare à l'octet : deux façons de la composer
    /// donneraient un `invalid_grant` dont le message ne dit pas laquelle des
    /// deux diffère.
    #[test]
    fn l_adresse_de_retour_est_absolue() {
        let r = adresse_de_retour();
        assert!(r.starts_with("https://"), "{r}");
        assert!(r.ends_with("/fr/compte/retour"), "{r}");
        assert!(!r.contains("//fr"), "double barre : {r}");
    }

    #[test]
    fn l_encodage_protege_ce_qui_casse_une_adresse() {
        assert_eq!(encoder("a b"), "a%20b");
        assert_eq!(encoder("https://x/y"), "https%3A%2F%2Fx%2Fy");
        assert_eq!(encoder("openid email"), "openid%20email");
        assert_eq!(
            encoder("Aa0-_.~"),
            "Aa0-_.~",
            "les signes sûrs passent tels quels"
        );
    }
}
