//! Le compte, réalisé — PKCE d'un côté, le backend de l'app de l'autre.

use base64::Engine;
use sha2::{Digest, Sha256};

use crate::application::ports::{Comptes, ErreurDeCompte};
use crate::domaine::compte::{Fournisseur, Session};

/// Un vérificateur PKCE et le défi qui en dérive.
///
/// ## Ce que PKCE empêche, et pourquoi le site en a besoin
///
/// Le code d'autorisation traverse le **navigateur du lecteur** : il apparaît
/// dans une barre d'adresse, dans un historique, dans les journaux d'un proxy.
/// Sans PKCE, quiconque l'intercepte avant nous peut l'échanger.
///
/// Avec, le code ne vaut que présenté avec le vérificateur — un secret que seul
/// celui qui a **commencé** le flux détient, et qui n'a jamais quitté le
/// serveur.
///
/// ## Trois détails de forme, et chacun casse l'échange
///
/// **`S256`, pas `plain`.** On envoie l'empreinte à l'aller, le secret au
/// retour. Envoyer le secret des deux côtés annulerait tout l'exercice.
///
/// **base64url, pas base64.** `+` et `/` deviennent `-` et `_` : les deux
/// premiers ont un sens dans une URL, et un `+` y devient une espace.
///
/// **Sans remplissage.** La spécification l'interdit, et un `=` laissé en place
/// fait refuser l'échange par un message qui parle de code invalide — donc qui
/// désigne le mauvais coupable.
#[derive(Debug, Clone)]
pub struct Pkce {
    /// Le secret, gardé côté serveur jusqu'au retour.
    pub verificateur: String,
    /// Son empreinte, envoyée au fournisseur à l'aller.
    pub defi: String,
}

impl Pkce {
    /// Un vérificateur neuf, tiré au hasard.
    ///
    /// Quatre-vingt-seize octets — au-dessus des quarante-trois signes minimum
    /// de la spécification, et sous les cent vingt-huit maximum une fois
    /// encodés. Le tirage vient du système, pas d'une graine : un vérificateur
    /// prévisible ne protège de rien.
    pub fn neuf() -> Self {
        use rand::RngCore;
        let mut octets = [0u8; 96];
        rand::thread_rng().fill_bytes(&mut octets);

        let verificateur = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(octets);
        let empreinte = Sha256::digest(verificateur.as_bytes());
        let defi = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(empreinte);

        Self { verificateur, defi }
    }
}

/// Le backend de l'app, joint par HTTP.
///
/// ## Pourquoi le site n'échange pas lui-même
///
/// Les secrets clients vivent dans la Lambda du backend, et doivent y rester :
/// les dupliquer ici ferait **deux endroits à faire tourner** le jour d'une
/// rotation, et l'un des deux serait oublié.
///
/// Ce n'est pas seulement de l'hygiène : c'est aussi ce qui donne un compte
/// **unique** aux deux plateformes. Le lecteur qui a surligné sur son iPhone
/// retrouve ses surlignages ici parce que c'est le même serveur qui a délivré
/// les deux sessions.
pub struct ComptesDuBackend {
    client: reqwest::Client,
    racine: String,
}

impl ComptesDuBackend {
    /// `racine` est l'origine du backend, sans barre finale — l'adresse
    /// `execute-api` aujourd'hui, `api.ontbible.com` le jour où elle servira.
    pub fn new(racine: impl Into<String>) -> Self {
        Self {
            // Un délai borné, et court. Une Lambda de site qui attend un
            // backend muet finit par dépasser son propre délai d'exécution : le
            // lecteur reçoit alors une erreur de passerelle, sans page et sans
            // explication. Dix secondes laissent le temps d'un démarrage à
            // froid — mesuré à 450 ms — et coupent avant que le site ne tombe.
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("un client HTTP sans réglage exotique"),
            racine: racine.into(),
        }
    }

    /// Lit une réponse du backend, ou dit pourquoi elle n'en est pas une.
    ///
    /// Les trois cas du port se décident ici, et le partage est celui-ci : un
    /// `4xx` est un refus — le code est consommé, le jeton révoqué, rien ne
    /// sert de réessayer. Tout le reste est une indisponibilité.
    async fn lire(reponse: reqwest::Response) -> Result<Session, ErreurDeCompte> {
        let statut = reponse.status();
        if statut.is_client_error() {
            return Err(ErreurDeCompte::Refuse);
        }
        if !statut.is_success() {
            return Err(ErreurDeCompte::Indisponible);
        }
        reponse
            .json::<Session>()
            .await
            .map_err(|erreur| ErreurDeCompte::ContratRompu(erreur.to_string()))
    }
}

#[async_trait::async_trait]
impl Comptes for ComptesDuBackend {
    async fn ouvrir(
        &self,
        fournisseur: Fournisseur,
        code: &str,
        redirect_uri: &str,
        verificateur: Option<&str>,
    ) -> Result<Session, ErreurDeCompte> {
        // Le corps est bâti à la main plutôt que par une structure : le backend
        // **omet** `code_verifier` pour Apple, et un `Option::None` sérialisé
        // en `null` n'est pas la même chose qu'une clé absente. Son
        // `#[serde(default)]` couvre la clé manquante, pas la clé nulle — c'est
        // le piège déjà rencontré sur `reference` et `verse` au §8 bis.
        // `origine: "webapp"` — le champ que le backend a ajouté le 27 août 2026
        // pour choisir entre deux identités chez un même fournisseur.
        //
        // ## Pourquoi il existe, et pourquoi il vaut « web » ici sans condition
        //
        // Apple accorde un code au **flux natif** contre l'App ID
        // `com.labibleont.ONT`, et au **flux web** contre un Services ID.
        // GitHub, lui, n'accepte qu'une adresse de retour par application : le
        // site a donc la sienne, avec son propre couple identifiant/secret.
        //
        // Le site est la webapp. On envoie donc toujours `"webapp"`, y
        // compris pour Google — que le backend ignore, son client « Application
        // Web » servant aux deux. Conditionner l'envoi au fournisseur ferait une
        // règle de plus à tenir d'accord avec la sienne, pour rien.
        //
        // **L'absence vaut `"app"` côté backend**, et c'est ce qui rend le champ
        // sûr : les versions installées de l'app ne l'envoient pas et ne le
        // pourront jamais rétroactivement. Un défaut à « webapp » les casserait
        // toutes.
        //
        // La valeur est **`webapp`** et non `web`, choisie par Gloire : c'est le
        // nom du dépôt — `ONTBibleWebapp` — et celui du Services ID Apple,
        // `com.labibleont.ont.webapp`. Un troisième mot pour la même chose
        // aurait fait chercher lequel des trois fait foi.
        let mut corps = serde_json::json!({
            "code": code,
            "redirect_uri": redirect_uri,
            "origine": "webapp",
        });
        if let Some(v) = verificateur {
            corps["code_verifier"] = serde_json::Value::String(v.to_string());
        }

        let reponse = self
            .client
            .post(format!("{}/auth/{}", self.racine, fournisseur.cle()))
            .json(&corps)
            .send()
            .await
            .map_err(|_| ErreurDeCompte::Indisponible)?;

        Self::lire(reponse).await
    }

    async fn renouveler(&self, jeton: &str) -> Result<Session, ErreurDeCompte> {
        let reponse = self
            .client
            .post(format!("{}/auth/refresh", self.racine))
            .json(&serde_json::json!({ "refresh_token": jeton }))
            .send()
            .await
            .map_err(|_| ErreurDeCompte::Indisponible)?;

        Self::lire(reponse).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Le défi est l'empreinte du vérificateur, encodée comme la spécification
    /// l'exige.
    ///
    /// Le vecteur vient de la RFC 7636, annexe B — c'est le seul cas dont on
    /// connaisse la réponse d'avance, et c'est donc le seul qui vaille pour
    /// éprouver l'encodage.
    #[test]
    fn le_defi_suit_la_rfc_7636() {
        let verificateur = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let empreinte = Sha256::digest(verificateur.as_bytes());
        let defi = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(empreinte);
        assert_eq!(defi, "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM");
    }

    /// Un vérificateur neuf tient dans les bornes, et ne se répète pas.
    ///
    /// Les bornes viennent de la spécification : entre 43 et 128 signes. En
    /// dessous, le secret est trop court pour protéger ; au-dessus, des
    /// fournisseurs refusent.
    #[test]
    fn un_verificateur_neuf_est_conforme_et_unique() {
        let a = Pkce::neuf();
        let b = Pkce::neuf();

        assert!(
            (43..=128).contains(&a.verificateur.len()),
            "{} signes, hors des bornes de la RFC",
            a.verificateur.len()
        );
        assert_ne!(a.verificateur, b.verificateur, "deux tirages identiques");
        assert_ne!(a.defi, b.defi);
    }

    /// Ni remplissage, ni signe qui change de sens dans une adresse.
    ///
    /// Un `=` fait refuser l'échange ; un `+` devient une espace dans une chaîne
    /// de requête ; un `/` y ouvre un segment. Les trois donnent le même
    /// symptôme — un message qui parle de code invalide, donc qui désigne le
    /// mauvais coupable.
    #[test]
    fn l_encodage_ne_porte_rien_qui_casse_une_adresse() {
        for _ in 0..32 {
            let p = Pkce::neuf();
            for chaine in [&p.verificateur, &p.defi] {
                assert!(!chaine.contains('='), "remplissage laissé : {chaine}");
                assert!(!chaine.contains('+'), "un + devient une espace : {chaine}");
                assert!(!chaine.contains('/'), "un / ouvre un segment : {chaine}");
            }
        }
    }
}

// ───────────────────────── la synchronisation ─────────────────────────────────

use crate::application::ports::{Moisson, Synchronisation};
use crate::domaine::surlignage::Surlignage;

/// La synchronisation, chez le backend de l'app.
///
/// Le même client HTTP que les comptes, et pour la même raison : c'est le
/// serveur du site qui parle au backend, jamais le navigateur. Le jeton reste
/// dans le cookie `HttpOnly`, donc hors de portée de tout script.
pub struct SyncDuBackend {
    client: reqwest::Client,
    racine: String,
}

impl SyncDuBackend {
    pub fn new(racine: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("un client HTTP sans réglage exotique"),
            racine: racine.into(),
        }
    }

    /// L'en-tête d'authentification, **exactement** comme le backend l'attend.
    ///
    /// Son `strip_prefix("Bearer ")` est sensible à la casse et exige l'espace
    /// unique : `bearer x` échoue, `Bearer  x` aussi. Et l'échec est un `401`
    /// dont le message est volontairement pauvre — le backend l'assume :
    /// « détailler pourquoi une authentification échoue renseigne surtout celui
    /// qui cherche à la contourner ». On ne saura donc pas que c'est l'espace.
    fn porteur(jeton: &str) -> String {
        format!("Bearer {jeton}")
    }
}

#[async_trait::async_trait]
impl Synchronisation for SyncDuBackend {
    async fn tirer(&self, jeton: &str, depuis: Option<i64>) -> Result<Moisson, ErreurDeCompte> {
        let mut adresse = format!("{}/sync", self.racine);
        if let Some(quand) = depuis {
            adresse.push_str(&format!("?since={quand}"));
        }

        let reponse = self
            .client
            .get(adresse)
            .header(reqwest::header::AUTHORIZATION, Self::porteur(jeton))
            .send()
            .await
            .map_err(|_| ErreurDeCompte::Indisponible)?;

        let statut = reponse.status();
        if statut == reqwest::StatusCode::UNAUTHORIZED {
            return Err(ErreurDeCompte::Refuse);
        }
        if !statut.is_success() {
            return Err(ErreurDeCompte::Indisponible);
        }
        reponse
            .json::<Moisson>()
            .await
            .map_err(|erreur| ErreurDeCompte::ContratRompu(erreur.to_string()))
    }

    async fn pousser(
        &self,
        jeton: &str,
        surlignages: &[Surlignage],
        position: Option<&crate::domaine::surlignage::Position>,
    ) -> Result<(), ErreurDeCompte> {
        // La position est **omise**, jamais mise à `null`.
        //
        // Le backend la déclare `Option` avec `#[serde(default)]` : une clé
        // absente vaut « rien à changer ». Un `null` explicite passerait aussi —
        // `Option` le lit — mais la clé absente est ce que fait l'app, et deux
        // clients qui envoient deux formes pour la même intention finissent par
        // découvrir que l'une des deux ne marchait pas.
        let mut corps = serde_json::json!({ "highlights": surlignages });
        if let Some(p) = position {
            corps["position"] = serde_json::to_value(p).unwrap_or(serde_json::Value::Null);
        }

        let reponse = self
            .client
            .put(format!("{}/sync", self.racine))
            .header(reqwest::header::AUTHORIZATION, Self::porteur(jeton))
            .json(&corps)
            .send()
            .await
            .map_err(|_| ErreurDeCompte::Indisponible)?;

        let statut = reponse.status();
        if statut == reqwest::StatusCode::UNAUTHORIZED {
            return Err(ErreurDeCompte::Refuse);
        }
        // `204 No Content` : on ne lit pas le corps, il n'y en a pas. En
        // attendre du JSON ferait échouer un envoi qui a réussi.
        if statut.is_success() {
            Ok(())
        } else {
            Err(ErreurDeCompte::Indisponible)
        }
    }
}

#[cfg(test)]
mod tests_sync {
    use super::*;

    /// L'en-tête est celui que le backend sait lire.
    ///
    /// Son `strip_prefix` exige la capitale et l'espace unique, et son `401` ne
    /// dit pas lequel des deux manque. Un test vaut mieux qu'une relecture.
    #[test]
    fn le_porteur_a_la_forme_exacte() {
        let e = SyncDuBackend::porteur("abc.def.ghi");
        assert_eq!(e, "Bearer abc.def.ghi");
        assert!(e.starts_with("Bearer "), "capitale et espace unique");
        assert!(
            !e.starts_with("bearer"),
            "le backend est sensible à la casse"
        );
        assert!(
            !e.contains("  "),
            "un espace en trop suffit à faire échouer"
        );
    }
}
