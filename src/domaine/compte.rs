//! Le compte du lecteur — ce qu'il est, et ce qui le prouve.
//!
//! ## Le site ne signe rien lui-même
//!
//! L'échange du code d'autorisation contre une session est fait par le
//! **backend de l'app**, qui détient les secrets clients. Le site ne fait que
//! conduire le lecteur chez le fournisseur, recevoir le code au retour, et le
//! passer au backend.
//!
//! C'est ce qui permet aux deux plateformes de partager un compte : le lecteur
//! qui a surligné sur son iPhone retrouve ses surlignages ici, parce que c'est
//! le même serveur qui a délivré les deux sessions.
//!
//! ## Ce module ne connaît ni réseau ni horloge
//!
//! Il décrit ce qu'est une session et quand elle est périmée — en recevant
//! l'instant plutôt qu'en le lisant. C'est ce qui le rend éprouvable sans
//! attendre une heure.

use serde::{Deserialize, Serialize};

/// Chez qui le lecteur prouve son identité.
///
/// L'ordre est celui du site : Google, Apple, GitHub. Ce n'est pas un goût —
/// Google est le seul dont le client est **déjà** en « Application Web » chez le
/// backend, donc le seul qui ne demande qu'une adresse de retour de plus. Apple
/// exige un Services ID à créer, et GitHub une seconde application, son portail
/// n'acceptant qu'une adresse de retour par client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Fournisseur {
    Google,
    Apple,
    Github,
}

impl Fournisseur {
    /// Le nom que le backend attend dans son chemin — `/auth/{ce nom}`.
    ///
    /// Repris tel quel de `Provider::parse` côté backend. Une divergence ici
    /// donnerait un `UnknownProvider`, c'est-à-dire une connexion qui échoue
    /// sans que le lecteur puisse rien y faire.
    pub fn cle(self) -> &'static str {
        match self {
            Self::Google => "google",
            Self::Apple => "apple",
            Self::Github => "github",
        }
    }

    /// Ce qu'on écrit sur le bouton.
    pub fn nom(self) -> &'static str {
        match self {
            Self::Google => "Google",
            Self::Apple => "Apple",
            Self::Github => "GitHub",
        }
    }

    /// Le fournisseur désigné par une clé, ou rien.
    ///
    /// Rien plutôt qu'une erreur : cette clé vient d'une adresse, donc de
    /// l'extérieur. Un chemin bricolé mène à une page qui le dit, pas à une
    /// panne.
    pub fn depuis_cle(cle: &str) -> Option<Self> {
        match cle {
            "google" => Some(Self::Google),
            "apple" => Some(Self::Apple),
            "github" => Some(Self::Github),
            _ => None,
        }
    }

    /// Tous, dans l'ordre d'affichage.
    pub fn tous() -> [Self; 3] {
        [Self::Google, Self::Apple, Self::Github]
    }

    /// Vrai quand le flux exige un vérificateur PKCE.
    ///
    /// Apple n'en veut pas — son flux natif s'en passe, et le backend le note :
    /// « le vérificateur PKCE, pour Google et GitHub. Absent pour Apple ». En
    /// envoyer un là où il n'est pas attendu fait échouer l'échange.
    pub fn exige_pkce(self) -> bool {
        !matches!(self, Self::Apple)
    }
}

/// Ce que le backend rend après un échange réussi.
///
/// Les noms des champs sont ceux du backend, à la lettre : c'est du JSON qui
/// traverse, et un `refreshToken` au lieu de `refresh_token` donnerait une
/// désérialisation qui échoue là où tout semble juste.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    /// Durée de vie du jeton d'accès, en secondes. Une heure côté backend.
    pub expires_in: i64,
    /// L'instant de création, en millisecondes depuis l'époque.
    pub created: i64,
}

impl Session {
    /// Vrai quand le jeton d'accès ne vaut plus rien à cet instant.
    ///
    /// ## La marge de soixante secondes n'est pas de la prudence
    ///
    /// Entre le moment où l'on juge un jeton valide et celui où le backend le
    /// reçoit, il y a un aller-retour réseau. Un jeton qui expire dans deux
    /// secondes passerait le test ici et serait refusé là-bas — et l'erreur
    /// serait un `401` qu'on prendrait pour une déconnexion.
    ///
    /// Une minute est plus longue que n'importe quel appel, et vingt fois plus
    /// courte que la durée de vie du jeton.
    pub fn perimee(&self, maintenant_ms: i64) -> bool {
        const MARGE_MS: i64 = 60_000;
        maintenant_ms + MARGE_MS >= self.created + self.expires_in * 1_000
    }

    /// L'instant, en millisecondes, où le jeton cesse d'être utilisable.
    pub fn fin_ms(&self) -> i64 {
        self.created + self.expires_in * 1_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session(created: i64, expires_in: i64) -> Session {
        Session {
            access_token: "a".into(),
            refresh_token: "r".into(),
            expires_in,
            created,
        }
    }

    #[test]
    fn les_cles_sont_celles_du_backend() {
        for f in Fournisseur::tous() {
            assert_eq!(Fournisseur::depuis_cle(f.cle()), Some(f));
        }
        assert_eq!(Fournisseur::depuis_cle("facebook"), None);
        assert_eq!(Fournisseur::depuis_cle(""), None);
    }

    #[test]
    fn apple_seul_se_passe_de_pkce() {
        assert!(Fournisseur::Google.exige_pkce());
        assert!(Fournisseur::Github.exige_pkce());
        assert!(
            !Fournisseur::Apple.exige_pkce(),
            "le backend dit « absent pour Apple » — en envoyer un ferait échouer l'échange"
        );
    }

    /// Un jeton d'une heure est périmé une minute avant l'heure.
    ///
    /// La marge est le cœur du test : sans elle, un jeton jugé valide ici
    /// arriverait expiré au backend, et le `401` se lirait comme une
    /// déconnexion.
    #[test]
    fn la_marge_perime_avant_l_echeance() {
        let s = session(1_000_000, 3600);
        assert!(!s.perimee(1_000_000), "neuf à la création");
        assert!(!s.perimee(1_000_000 + 3_500_000), "encore bon à 3 500 s");
        assert!(
            s.perimee(1_000_000 + 3_541_000),
            "à 59 secondes de l'échéance, on renouvelle déjà"
        );
        assert!(s.perimee(1_000_000 + 3_600_000), "à l'échéance");
        assert!(s.perimee(i64::MAX / 2), "bien après");
    }

    /// Une session de durée nulle ou négative est périmée d'emblée.
    ///
    /// Le backend n'en produit pas, mais ce qui traverse un réseau se lit comme
    /// venant de l'extérieur : une durée absurde doit conduire à renouveler,
    /// jamais à faire confiance.
    #[test]
    fn une_duree_absurde_perime_tout_de_suite() {
        assert!(session(1_000, 0).perimee(1_000));
        assert!(session(1_000, -3600).perimee(1_000));
    }
}

/// L'accord avec le backend sur le champ `origine`.
///
/// Il décide quelle identité le backend présente au fournisseur — App ID pour le
/// flux natif, Services ID pour le web. Une valeur qui ne serait plus reconnue
/// ferait présenter la mauvaise, et Apple rendrait `invalid_grant` : une erreur
/// qui parle du **code**, donc qui désigne le mauvais coupable.
#[cfg(all(test, feature = "ssr"))]
mod contrat {
    /// Le backend connaît toujours `origine`, et « web » y reste une valeur.
    ///
    /// Se tait si le dépôt voisin n'est pas là — un test d'accord ne peut pas
    /// exiger la présence de ce avec quoi il accorde.
    #[test]
    fn le_backend_connait_toujours_l_origine() {
        let chemin = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../ONTBibleApp/backend/src/interface/mod.rs"
        );
        let Ok(source) = std::fs::read_to_string(chemin) else {
            return;
        };
        // Tant que le champ n'est pas fusionné chez lui, il ignore ce qu'on
        // envoie — aucun `deny_unknown_fields` sur `SignInBody`. Le test ne
        // rougit donc pas avant l'heure, il attend.
        if !source.contains("origine") {
            return;
        }
        assert!(
            source.contains("\"web\"") || source.contains("Web"),
            "le backend connaît `origine` mais « web » n'y paraît plus : le site \
             enverrait une valeur qu'il ne reconnaît pas, et présenterait la \
             mauvaise identité au fournisseur"
        );
    }

    /// Aucun `deny_unknown_fields` sur le corps de connexion.
    ///
    /// C'est ce qui permet au site d'envoyer `origine` **avant** que le backend
    /// ne le connaisse — serde l'ignore. Le jour où quelqu'un l'ajoute par
    /// hygiène, tout envoi du site échouerait d'un coup, et le message parlerait
    /// d'un champ inconnu sans dire lequel des deux dépôts a bougé.
    #[test]
    fn le_corps_de_connexion_tolere_un_champ_de_plus() {
        let chemin = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../ONTBibleApp/backend/src/interface/mod.rs"
        );
        let Ok(source) = std::fs::read_to_string(chemin) else {
            return;
        };
        let corps = source
            .split("struct SignInBody")
            .nth(1)
            .and_then(|reste| reste.split('}').next())
            .unwrap_or("");
        assert!(
            !source
                .split("struct SignInBody")
                .next()
                .unwrap_or("")
                .lines()
                .rev()
                .take(3)
                .any(|l| l.contains("deny_unknown_fields")),
            "un `deny_unknown_fields` est apparu sur `SignInBody` : le site \
             envoie `origine`, et tous ses envois échoueraient"
        );
        assert!(!corps.is_empty(), "SignInBody doit exister côté backend");
    }
}
