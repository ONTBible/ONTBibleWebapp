//! Le profil du lecteur — ce qu'il choisit de dire de lui.
//!
//! ## Les noms sont ceux du backend, à la lettre
//!
//! `nom_dusage`, `prenom`, `nom`, `bio`, `portrait`, `updated_at` — c'est du
//! JSON qui traverse `/sync`, et un `nomDUsage` au lieu de `nom_dusage` donnerait
//! une désérialisation qui échoue là où tout semble juste. Une épreuve de
//! contrat les relit dans la source du backend plutôt que de les redire.
//!
//! ## Ce qui n'y est pas, et pourquoi
//!
//! Aucune adresse, aucun identifiant de fournisseur. Le backend range les
//! surlignages d'un lecteur de Bible sous l'article 9 du RGPD ; le profil suit
//! la même règle — on garde ce que le lecteur écrit, rien de ce qu'on pourrait
//! déduire.

use serde::{Deserialize, Serialize};

/// Ce qu'un lecteur a écrit de lui.
///
/// Tous les champs sont facultatifs à la lecture : un compte neuf n'a pas de
/// profil, et un profil écrit avant l'ajout d'un champ n'en porte pas la clé.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profil {
    #[serde(default)]
    pub nom_dusage: String,
    #[serde(default)]
    pub prenom: String,
    #[serde(default)]
    pub nom: String,
    #[serde(default)]
    pub bio: String,
    /// Une adresse d'image, si le lecteur en a posé une **dans l'app**.
    ///
    /// Le site l'affiche et ne la change pas : téléverser une image demande un
    /// stockage que le projet n'a pas, et un champ qu'on remplirait à moitié
    /// vaut moins qu'un champ qu'on laisse à qui sait le remplir.
    #[serde(default)]
    pub portrait: Option<String>,
    /// Millisecondes depuis l'époque.
    ///
    /// **Zéro** quand la clé manque, et non l'instant présent : le backend
    /// garde le plus récent, donc un profil sans horodatage doit *perdre*
    /// contre n'importe quel autre. L'app dit la même chose avec
    /// `.distantPast` — « un profil sans horodatage doit perdre contre
    /// n'importe quel autre, et non gagner parce qu'on vient de le lire ».
    #[serde(default)]
    pub updated_at: i64,
}

impl Profil {
    /// « Prénom Nom », ou rien si les deux sont vides.
    pub fn nom_affiche(&self) -> Option<String> {
        let entier = format!("{} {}", self.prenom, self.nom).trim().to_string();
        (!entier.is_empty()).then_some(entier)
    }

    /// Ce qu'on écrit quand il faut nommer quelqu'un.
    ///
    /// L'ordre est celui de l'app : le nom complet, puis le nom d'usage, puis
    /// « Vous ». On ne laisse jamais un blanc — un profil vide est un cas
    /// normal, pas une donnée manquante.
    pub fn nom_de_barre(&self) -> String {
        self.nom_affiche()
            .or_else(|| (!self.nom_dusage.is_empty()).then(|| self.nom_dusage.clone()))
            .unwrap_or_else(|| "Vous".to_string())
    }

    /// Une ou deux initiales, en capitales.
    pub fn initiales(&self) -> String {
        [&self.prenom, &self.nom]
            .into_iter()
            .filter_map(|part| part.trim().chars().next())
            .take(2)
            .flat_map(char::to_uppercase)
            .collect()
    }

    /// « @nom-d-usage », s'il y en a un.
    pub fn arobase(&self) -> Option<String> {
        (!self.nom_dusage.is_empty()).then(|| format!("@{}", self.nom_dusage))
    }

    /// Vrai quand le lecteur n'a rien écrit.
    pub fn est_vide(&self) -> bool {
        self.nom_dusage.is_empty()
            && self.nom_affiche().is_none()
            && self.bio.trim().is_empty()
            && self.portrait.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profil(prenom: &str, nom: &str, usage: &str) -> Profil {
        Profil {
            prenom: prenom.into(),
            nom: nom.into(),
            nom_dusage: usage.into(),
            ..Profil::default()
        }
    }

    #[test]
    fn le_nom_complet_devance_le_nom_d_usage() {
        assert_eq!(
            profil("Gloire", "Bikouta", "sheeliel").nom_de_barre(),
            "Gloire Bikouta"
        );
        assert_eq!(profil("", "", "sheeliel").nom_de_barre(), "sheeliel");
        assert_eq!(profil("", "", "").nom_de_barre(), "Vous");
    }

    /// Un prénom seul ne doit pas rendre « Gloire  » avec son blanc de queue.
    #[test]
    fn un_seul_des_deux_noms_se_lit_sans_blanc() {
        assert_eq!(
            profil("Gloire", "", "").nom_affiche().as_deref(),
            Some("Gloire")
        );
        assert_eq!(
            profil("", "Bikouta", "").nom_affiche().as_deref(),
            Some("Bikouta")
        );
        assert_eq!(profil("", "", "").nom_affiche(), None);
    }

    #[test]
    fn les_initiales_sont_deux_au_plus_et_en_capitales() {
        assert_eq!(profil("gloire", "bikouta", "").initiales(), "GB");
        assert_eq!(profil("Gloire", "", "").initiales(), "G");
        assert_eq!(profil("", "", "").initiales(), "");
        // Un accent reste une lettre, et sa capitale est la sienne.
        assert_eq!(profil("Élie", "", "").initiales(), "É");
    }

    #[test]
    fn un_profil_neuf_est_vide() {
        assert!(Profil::default().est_vide());
        assert!(!profil("", "", "sheeliel").est_vide());
        assert!(!Profil {
            bio: "Je lis.".into(),
            ..Profil::default()
        }
        .est_vide());
        // Une bio de blancs n'est pas une bio.
        assert!(Profil {
            bio: "   \n ".into(),
            ..Profil::default()
        }
        .est_vide());
    }

    /// **Un profil sans horodatage doit perdre.**
    ///
    /// Le backend garde le plus récent. Si la clé absente valait l'instant
    /// présent, un profil vide relu écraserait celui que le lecteur a écrit sur
    /// son téléphone — la perte serait silencieuse et définitive.
    #[test]
    fn un_horodatage_absent_vaut_zero_et_non_maintenant() {
        let lu: Profil = serde_json::from_str(r#"{"prenom":"Gloire"}"#).unwrap();
        assert_eq!(lu.updated_at, 0);
        assert_eq!(lu.prenom, "Gloire");
    }

    /// Les noms JSON sont ceux du backend, et rien ne les renomme.
    #[test]
    fn les_cles_json_sont_celles_du_backend() {
        let ecrit = serde_json::to_string(&profil("Gloire", "Bikouta", "sheeliel")).unwrap();
        for cle in [
            "nom_dusage",
            "prenom",
            "nom",
            "bio",
            "portrait",
            "updated_at",
        ] {
            assert!(
                ecrit.contains(&format!("\"{cle}\"")),
                "« {cle} » manque de {ecrit}"
            );
        }
    }

    /// ## Les champs sont relus chez le backend, jamais redits ici
    ///
    /// C'est la garde qui manquait à `Session` et qui a coûté toute connexion :
    /// une épreuve qui lit la bonne source dans le mauvais fichier passe au vert
    /// exactement comme une garde qui lit la bonne.
    ///
    /// Elle **se tait** si le dépôt voisin n'est pas là — clone partiel, machine
    /// de compilation. Un test d'accord ne peut pas exiger la présence de ce
    /// avec quoi il accorde ; il peut seulement refuser de mentir sur ce qu'il
    /// n'a pas vu.
    #[cfg(feature = "ssr")]
    #[test]
    fn le_profil_du_backend_porte_les_memes_champs() {
        let chemin = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../ONTBibleApp/backend/src/domain/sync.rs"
        );
        let Ok(source) = std::fs::read_to_string(chemin) else {
            return;
        };
        let Some(bloc) = source
            .split("pub struct ProfilLecteur {")
            .nth(1)
            .and_then(|reste| reste.split("\n}").next())
        else {
            panic!(
                "`pub struct ProfilLecteur` introuvable dans {chemin}.\n\
                 \n\
                 Deux causes, et la première est la plus fréquente :\n\
                 1. l'arbre de travail d'ONTBibleApp est **en retard** — il suit \n\
                    une branche coupée avant ce champ. `git -C ../ONTBibleApp \n\
                    log --oneline -1 origin/main -- backend/src/domain/sync.rs` \n\
                    le dit en une ligne ;\n\
                 2. le backend l'a renommée, et le site doit suivre.\n\
                 \n\
                 La CI clone la référence à chaque fois, donc elle ne voit que \n\
                 la seconde. En local, c'est presque toujours la première."
            );
        };
        for (champ, attendu) in [
            ("nom_dusage", "String"),
            ("prenom", "String"),
            ("nom", "String"),
            ("bio", "String"),
            ("portrait", "Option<String>"),
            ("updated_at", "i64"),
        ] {
            let ligne = bloc
                .lines()
                .find(|l| l.trim_start().starts_with(&format!("pub {champ}:")))
                .unwrap_or_else(|| panic!("le backend ne porte plus de champ `{champ}`"));
            assert!(
                ligne.contains(attendu),
                "le backend écrit `{}` là où le site attend `{attendu}`",
                ligne.trim()
            );
        }
    }
}
