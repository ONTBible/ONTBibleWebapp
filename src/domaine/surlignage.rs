//! Les surlignages du lecteur, et leur accord avec le serveur.
//!
//! ## Ce sont des données de catégorie particulière
//!
//! Le backend le dit en tête de son propre module, et ça vaut ici tel quel :
//!
//! > « Les surlignages et les notes d'un lecteur de Bible, rattachés à une
//! > identité, révèlent des convictions religieuses : article 9 du RGPD. »
//!
//! D'où deux règles qu'on ne relâche pas : la synchronisation est **facultative**
//! — le site se lit entièrement sans compte —, et l'on ne stocke que la
//! *référence* d'un verset, jamais son texte. Le corpus est déjà là.
//!
//! ## La granularité est le verset
//!
//! Jamais un décalage de caractères. Le backend l'explique : « une révision du
//! texte déplacerait les caractères et rendrait le surlignage faux, alors qu'un
//! numéro de verset reste juste ». Le corpus est en cours de traduction ; ses
//! versets seront révisés.

use serde::{Deserialize, Serialize};

/// Les cinq teintes, telles que l'app les nomme.
///
/// Les identifiants sont ceux qui partent au backend — `gold`, `olive`, `sky`,
/// `rose`, `violet` —, relevés dans `ONTKit/Reader/Reader.swift`. **Le backend
/// ne les valide pas** : `color` y est une chaîne libre. C'est donc au client de
/// tenir la liste, et deux clients qui divergeraient afficheraient deux couleurs
/// différentes pour la même marque.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Couleur {
    Gold,
    Olive,
    Sky,
    Rose,
    Violet,
}

impl Couleur {
    /// L'identifiant qui voyage.
    pub fn cle(self) -> &'static str {
        match self {
            Self::Gold => "gold",
            Self::Olive => "olive",
            Self::Sky => "sky",
            Self::Rose => "rose",
            Self::Violet => "violet",
        }
    }

    /// Ce qu'un lecteur d'écran annonce.
    pub fn nom(self) -> &'static str {
        match self {
            Self::Gold => "Or",
            Self::Olive => "Olive",
            Self::Sky => "Ciel",
            Self::Rose => "Rose",
            Self::Violet => "Violet",
        }
    }

    /// La teinte, telle que `ONTColors.highlight(_:)` la rend.
    ///
    /// Convertie depuis les flottants sRGB de l'app — elle n'écrit pas les
    /// hexadécimaux en commentaire, contrairement aux couleurs de marque. Le
    /// commentaire qui les justifie, lui, vaut d'être repris : « Cinq teintes
    /// tirées vers le pastel plutôt que le fluo d'écolier : un surlignage se
    /// pose sur un texte qu'on lit longtemps, il ne doit pas crier ni rendre le
    /// texte illisible. »
    ///
    /// **Une seule valeur, quel que soit le thème** : la fonction de l'app ne
    /// prend pas de mode. Le site n'ayant qu'une peau, la question ne se pose
    /// pas non plus ici.
    pub fn teinte(self) -> &'static str {
        match self {
            Self::Gold => "#E8C973",
            Self::Olive => "#B8C787",
            Self::Sky => "#9EC7DE",
            Self::Rose => "#EBADAD",
            Self::Violet => "#C7B3DE",
        }
    }

    /// Toutes, dans l'ordre de l'app.
    pub fn toutes() -> [Self; 5] {
        [Self::Gold, Self::Olive, Self::Sky, Self::Rose, Self::Violet]
    }

    /// La couleur d'une clé, ou rien.
    ///
    /// Rien plutôt qu'une erreur, et le commentaire de l'app dit pourquoi :
    /// « une couleur inconnue vient d'une version plus récente de l'app, et on
    /// préfère ignorer la ligne plutôt que de faire échouer toute la
    /// synchronisation ».
    pub fn depuis_cle(cle: &str) -> Option<Self> {
        Self::toutes().into_iter().find(|c| c.cle() == cle)
    }
}

/// Ce qu'on retient d'un verset marqué.
///
/// Les noms des champs sont ceux du backend, **à la lettre** : `snake_case`
/// littéral, aucun `rename` nulle part. Un `bookId` au lieu de `book_id` ferait
/// une désérialisation qui échoue là où tout paraît juste.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Surlignage {
    pub id: String,
    pub book_id: String,
    pub chapter_id: String,
    pub verse: u32,
    /// La couleur, en chaîne libre — le backend ne la valide pas, et une valeur
    /// inconnue doit survivre au voyage plutôt que faire échouer la ligne.
    pub color: String,
    /// Absent de la sortie quand il n'y a pas de note — `skip_serializing_if`
    /// côté backend, et non `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Millisecondes depuis l'époque.
    pub updated_at: i64,
    /// La pierre tombale. Toujours sérialisée, tolérée absente à la lecture.
    #[serde(default)]
    pub deleted: bool,
}

impl Surlignage {
    /// La clé qui identifie ce surlignage **pour la fusion**.
    ///
    /// ## Ce n'est pas `id`, et c'est le piège de ce module
    ///
    /// Le backend apparie par `sort_key()`, soit `(chapter_id, verse)`. Deux
    /// conséquences qu'aucune signature ne montre :
    ///
    /// - **deux couleurs ne coexistent pas sur un même verset** — la seconde
    ///   remplace la première ;
    /// - **un `id` neuf sur un verset déjà marqué écrase l'ancien**, au lieu de
    ///   créer un second surlignage comme on s'y attendrait.
    ///
    /// Le site doit donc apparier comme le backend, sans quoi il croirait avoir
    /// deux marques là où le serveur n'en garde qu'une — et l'écart ne se
    /// verrait qu'après un aller-retour.
    pub fn cle(&self) -> (String, u32) {
        (self.chapter_id.clone(), self.verse)
    }

    /// La couleur reconnue, ou rien.
    pub fn couleur(&self) -> Option<Couleur> {
        Couleur::depuis_cle(&self.color)
    }

    /// Vrai quand ce surlignage doit être **montré**.
    ///
    /// Une pierre tombale traverse la synchronisation — c'est ce qui propage une
    /// suppression — mais elle ne se dessine pas. L'app l'a appris à ses dépens,
    /// et son commentaire vaut d'être gardé ici :
    ///
    /// > « Une pierre tombale est en revanche **rendue**, pas écartée. Elle
    /// > l'était : `guard !deleted` renvoyait `nil`, donc une suppression faite
    /// > sur un autre appareil n'arrivait jamais jusqu'à la fusion. »
    ///
    /// Autrement dit : l'écarter **à la lecture du réseau** casse la
    /// synchronisation ; l'écarter **à l'affichage** est exactement ce qu'il
    /// faut. Les deux gestes se ressemblent et n'ont pas le même effet.
    pub fn visible(&self) -> bool {
        !self.deleted && self.couleur().is_some()
    }
}

/// Qui l'emporte, du serveur ou de ce qui arrive.
///
/// Dernier écrit gagné, **strictement**. À égalité, le serveur garde sa version
/// — et le backend explique pourquoi : « deux appareils dont les horloges
/// concordent à la milliseconde près sont plus probablement le même écrit
/// rejoué qu'un vrai conflit ».
pub fn l_entrant_gagne(serveur: &Surlignage, entrant: &Surlignage) -> bool {
    entrant.updated_at > serveur.updated_at
}

/// Fusionne ce qui vient du serveur dans ce qu'on a, surlignage par surlignage.
///
/// **Individuellement, jamais en bloc.** Le backend le fait ainsi et le dit :
/// « un appareil resté longtemps hors ligne ne doit pas écraser en bloc ce
/// qu'un autre a fait entre-temps ». Le site est exactement cet appareil-là
/// quand on rouvre un onglet laissé de côté.
pub fn fusionner(locaux: &mut Vec<Surlignage>, entrants: Vec<Surlignage>) {
    for entrant in entrants {
        match locaux.iter_mut().find(|l| l.cle() == entrant.cle()) {
            Some(local) => {
                if l_entrant_gagne(local, &entrant) {
                    *local = entrant;
                }
            }
            None => locaux.push(entrant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn marque(chapitre: &str, verset: u32, couleur: &str, quand: i64) -> Surlignage {
        Surlignage {
            id: format!("{chapitre}-{verset}"),
            book_id: "bereshit".into(),
            chapter_id: chapitre.into(),
            verse: verset,
            color: couleur.into(),
            note: None,
            updated_at: quand,
            deleted: false,
        }
    }

    #[test]
    fn les_cles_de_couleur_sont_celles_de_l_app() {
        for c in Couleur::toutes() {
            assert_eq!(Couleur::depuis_cle(c.cle()), Some(c));
            assert!(
                c.teinte().starts_with('#') && c.teinte().len() == 7,
                "{}",
                c.teinte()
            );
        }
        assert_eq!(Couleur::depuis_cle("turquoise"), None);
    }

    /// Le JSON produit est celui que le backend attend, à la lettre.
    ///
    /// Deux dissymétries à ne pas rater, et elles viennent du backend : `note`
    /// est **absent** quand il n'y en a pas — pas `null` —, et `deleted` est
    /// **toujours** présent.
    #[test]
    fn le_json_est_celui_du_backend() {
        let m = marque("bereshit-1", 3, "gold", 1_700_000_000_000);
        let json = serde_json::to_value(&m).expect("sérialisable");

        for cle in [
            "id",
            "book_id",
            "chapter_id",
            "verse",
            "color",
            "updated_at",
            "deleted",
        ] {
            assert!(json.get(cle).is_some(), "« {cle} » manque : {json}");
        }
        assert!(
            json.get("note").is_none(),
            "une note absente ne doit pas paraître, même à null : {json}"
        );

        let avec = Surlignage {
            note: Some("ici".into()),
            ..m
        };
        assert_eq!(serde_json::to_value(&avec).unwrap()["note"], "ici");
    }

    /// `deleted` peut manquer à la lecture, et vaut alors faux.
    #[test]
    fn un_deleted_absent_se_lit_comme_faux() {
        let brut = r#"{"id":"a","book_id":"b","chapter_id":"c","verse":1,
                       "color":"gold","updated_at":1}"#;
        let lu: Surlignage = serde_json::from_str(brut).expect("lisible sans deleted");
        assert!(!lu.deleted);
        assert_eq!(lu.note, None);
    }

    /// La clé de fusion est le verset, pas l'identifiant.
    ///
    /// C'est le piège du module : un `id` neuf sur un verset déjà marqué
    /// **écrase** au lieu d'ajouter. Le site doit s'en accorder, sinon il
    /// croirait avoir deux marques là où le serveur n'en garde qu'une.
    #[test]
    fn deux_identifiants_sur_un_meme_verset_ne_font_qu_une_marque() {
        let mut locaux = vec![marque("bereshit-1", 3, "gold", 100)];
        let autre = Surlignage {
            id: "identifiant-tout-neuf".into(),
            ..marque("bereshit-1", 3, "sky", 200)
        };
        fusionner(&mut locaux, vec![autre]);

        assert_eq!(locaux.len(), 1, "le verset 3 ne porte qu'une marque");
        assert_eq!(locaux[0].color, "sky", "la plus récente l'emporte");
    }

    #[test]
    fn le_plus_recent_gagne_et_l_egalite_garde_l_existant() {
        let mut locaux = vec![marque("bereshit-1", 1, "gold", 500)];
        fusionner(&mut locaux, vec![marque("bereshit-1", 1, "rose", 400)]);
        assert_eq!(locaux[0].color, "gold", "un plus ancien ne remplace pas");

        fusionner(&mut locaux, vec![marque("bereshit-1", 1, "olive", 500)]);
        assert_eq!(locaux[0].color, "gold", "à égalité, l'existant reste");

        fusionner(&mut locaux, vec![marque("bereshit-1", 1, "violet", 501)]);
        assert_eq!(locaux[0].color, "violet", "une milliseconde suffit");
    }

    /// Une pierre tombale se garde et ne se montre pas.
    ///
    /// Les deux moitiés comptent : la garder propage la suppression aux autres
    /// appareils, ne pas la montrer évite un surlignage fantôme. L'app a perdu
    /// des suppressions en écartant trop tôt.
    #[test]
    fn une_pierre_tombale_traverse_mais_ne_se_dessine_pas() {
        let mut locaux = vec![marque("bereshit-1", 5, "gold", 100)];
        let effacee = Surlignage {
            deleted: true,
            ..marque("bereshit-1", 5, "gold", 200)
        };
        fusionner(&mut locaux, vec![effacee]);

        assert_eq!(
            locaux.len(),
            1,
            "elle reste dans l'état — c'est ce qui propage"
        );
        assert!(!locaux[0].visible(), "mais elle ne se dessine pas");
    }

    /// Une couleur inconnue ne casse rien, elle ne se dessine simplement pas.
    #[test]
    fn une_couleur_inconnue_ne_fait_pas_echouer_la_ligne() {
        let m = marque("bereshit-1", 2, "turquoise", 100);
        assert_eq!(m.couleur(), None);
        assert!(!m.visible());
        assert!(
            serde_json::to_value(&m).is_ok(),
            "elle doit repartir telle quelle"
        );
    }

    /// Des versets différents cohabitent.
    #[test]
    fn des_versets_differents_cohabitent() {
        let mut locaux = vec![marque("bereshit-1", 1, "gold", 100)];
        fusionner(
            &mut locaux,
            vec![
                marque("bereshit-1", 2, "sky", 100),
                marque("bereshit-2", 1, "rose", 100),
            ],
        );
        assert_eq!(locaux.len(), 3);
    }
}

/// Le contrat avec le backend, éprouvé sur sa **source** et non sur nos souvenirs.
///
/// ## Le défaut que ces tests attrapent n'existe dans aucun des deux dépôts
///
/// Il vit **entre** eux. Un `#[serde(rename_all = "camelCase")]` posé côté
/// backend est une ligne qui a l'air d'une amélioration de style : elle ne casse
/// aucune compilation là-bas, aucun de ses tests, et elle ne touche à rien ici.
///
/// Elle rend simplement une réponse que le site ne sait plus lire — et comme
/// `mes_surlignages` traduit un échec en liste vide, le lecteur verrait « aucun
/// surlignage » au lieu d'une erreur. **Un `Ok(Vec::new())` parfaitement bien
/// formé sur une synchronisation qui n'a pas eu lieu.**
///
/// C'est le motif que les sessions se renvoient depuis deux jours, à son point
/// extrême : le silence n'est pas l'absence d'événement, c'est l'absence de
/// question posée sur l'événement.
///
/// ## Pourquoi lire la source plutôt que fabriquer un exemple
///
/// Un exemple écrit à la main éprouve ce qu'on **croit** que le backend produit.
/// C'est exactement la supposition qu'on veut retirer. Le test lit donc
/// `../ONTBibleApp/backend/src/domain/sync.rs` et compare champ par champ.
///
/// Il **se tait** si le dépôt voisin n'est pas là — clone partiel, machine de
/// compilation. Un test d'accord ne peut pas exiger la présence de ce avec quoi
/// il accorde ; il peut seulement refuser de mentir sur ce qu'il n'a pas vu.
#[cfg(all(test, feature = "ssr"))]
mod contrat {
    use super::*;

    /// La source de `Highlight` côté backend, si le dépôt voisin est là.
    fn source_du_backend() -> Option<String> {
        let chemin = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../ONTBibleApp/backend/src/domain/sync.rs"
        );
        std::fs::read_to_string(chemin).ok()
    }

    /// Le backend n'a posé aucun renommage — nos noms sont donc les siens.
    ///
    /// C'est **la** condition qui rend le reste vrai. Tant qu'elle tient, un
    /// champ Rust `book_id` voyage en `book_id`. Le jour où elle tombe, tout ce
    /// module devient faux d'un coup, et il vaut mieux l'apprendre ici qu'en
    /// voyant un lecteur perdre ses marques.
    #[test]
    fn le_backend_ne_renomme_rien() {
        let Some(source) = source_du_backend() else {
            return;
        };
        assert!(
            !source.contains("rename_all"),
            "le backend a posé un `rename_all` : les noms JSON ont changé, et le \
             site lira une réponse vide qu'il prendra pour « pas de compte »"
        );
        assert!(
            !source.contains("#[serde(rename"),
            "le backend a renommé un champ — même remarque"
        );
    }

    /// Chaque champ que nous déclarons existe chez lui, au même nom.
    ///
    /// On cherche `pub <nom>:` dans sa structure : c'est la forme exacte d'une
    /// déclaration Rust, donc un nom trouvé ainsi est un nom qui voyage.
    #[test]
    fn chaque_champ_existe_chez_le_backend() {
        let Some(source) = source_du_backend() else {
            return;
        };
        let structure = source
            .split("pub struct Highlight")
            .nth(1)
            .and_then(|reste| reste.split("\n}").next())
            .expect("la structure Highlight doit exister côté backend");

        for champ in [
            "id",
            "book_id",
            "chapter_id",
            "verse",
            "color",
            "note",
            "updated_at",
            "deleted",
        ] {
            assert!(
                structure.contains(&format!("pub {champ}:")),
                "`{champ}` n'existe plus dans `Highlight` côté backend — le site \
                 l'envoie pourtant, et le recevra sans le comprendre"
            );
        }
    }

    /// Les deux dissymétries de sérialisation sont toujours celles qu'on suit.
    ///
    /// `note` est **absent** quand il n'y en a pas — pas `null` — et `deleted`
    /// tolère l'absence à la lecture. Ce sont deux attributs, et les deux ont un
    /// effet visible : un `null` inattendu fait échouer une désérialisation
    /// stricte, et un `deleted` obligatoire ferait rejeter les réponses d'une
    /// version antérieure.
    #[test]
    fn les_deux_dissymetries_tiennent() {
        let Some(source) = source_du_backend() else {
            return;
        };
        assert!(
            source.contains("skip_serializing_if = \"Option::is_none\""),
            "`note` ne se tait plus quand elle est absente : le backend enverra \
             `null`, que notre `Option` lit encore — mais l'inverse ne serait pas \
             vrai, et c'est le sens de ce test"
        );
        assert!(
            source.contains("#[serde(default)]"),
            "`deleted` n'est plus tolérant à l'absence"
        );
    }

    /// Un objet du site se relit par un type de la forme du backend.
    ///
    /// C'est l'aller-retour, la seule épreuve qui ne dépende d'aucune lecture de
    /// source : on sérialise ce qu'on enverrait, et on le relit avec un type
    /// **écrit à part**, dont les noms sont copiés du backend à la main. Si les
    /// deux divergent, la relecture échoue ici plutôt qu'en production.
    #[test]
    fn ce_que_le_site_envoie_se_relit_comme_le_backend_le_lit() {
        // Écrit à la main, d'après `backend/src/domain/sync.rs`. Volontairement
        // **pas** un alias de `Surlignage` : un alias se contenterait de
        // confirmer que le type est d'accord avec lui-même.
        #[derive(serde::Deserialize)]
        #[allow(dead_code)]
        struct CommeLeBackend {
            id: String,
            book_id: String,
            chapter_id: String,
            verse: u32,
            color: String,
            #[serde(default)]
            note: Option<String>,
            updated_at: i64,
            #[serde(default)]
            deleted: bool,
        }

        let envoye = Surlignage {
            id: "bereshit-1-3".into(),
            book_id: "bereshit".into(),
            chapter_id: "bereshit-1".into(),
            verse: 3,
            color: "gold".into(),
            note: Some("la parole qui accomplit".into()),
            updated_at: 1_756_000_000_000,
            deleted: false,
        };

        let fil = serde_json::to_string(&envoye).expect("sérialisable");
        let relu: CommeLeBackend =
            serde_json::from_str(&fil).expect("le backend doit savoir relire ce qu'on envoie");

        assert_eq!(relu.chapter_id, "bereshit-1");
        assert_eq!(relu.verse, 3);
        assert_eq!(relu.note.as_deref(), Some("la parole qui accomplit"));

        // Et sans note : la clé doit être **absente**, pas nulle.
        let sans = Surlignage {
            note: None,
            ..envoye
        };
        let fil = serde_json::to_string(&sans).expect("sérialisable");
        assert!(
            !fil.contains("note"),
            "une note absente ne doit pas paraître dans le fil : {fil}"
        );
        let _: CommeLeBackend = serde_json::from_str(&fil).expect("relisible sans note");
    }
}
