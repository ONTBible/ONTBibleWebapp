//! Le vivier, embarqué dans le binaire.
//!
//! ## Pourquoi embarqué, et non lu à l'exécution
//!
//! Le site tourne en Lambda. Un fichier lu au démarrage s'ajoute au démarrage
//! à froid, déjà mesuré à ~450 ms ; 58 Ko dans le binaire ne coûtent rien et
//! ne peuvent pas manquer à l'appel. Un vivier absent en production serait une
//! page d'accueil muette, découverte par un lecteur avant nous.
//!
//! ## Pourquoi le fichier du dépôt voisin, et non une copie
//!
//! `daily.json` est produit par le pipeline de `ONTBibleApp`. Le copier ici
//! créerait une seconde vérité : le jour où le pipeline change sa règle
//! éditoriale — la longueur retenue, l'exclusion des brouillons — le site
//! garderait l'ancienne, et annoncerait un autre verset que l'app.
//!
//! On le lit donc **là où il est**, à la compilation. Si le dépôt voisin
//! manque, la compilation échoue avec le chemin en clair. C'est le
//! comportement voulu : mieux vaut un build qui refuse qu'un site qui ment.
//!
//! Le jour où le site se déploiera sans ce voisin, la sortie est connue :
//! `ONTBibleApp` publie `dist/` comme artefact de version, et un script le
//! récupère avant la compilation. Le chemin devient une variable, rien de
//! plus.

use serde::Deserialize;

use crate::application::ports::Vivier;
use crate::domaine::vivier::VersetQuotidien;

const SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../ONTBibleApp/dist/daily.json"
));

/// Le fichier tel que le pipeline l'écrit.
///
/// Les noms sont ceux du JSON — courts, parce qu'ils sont répétés 251 fois et
/// que ce fichier voyage aussi dans un widget. Ils s'arrêtent ici : le domaine
/// nomme les choses en clair, et c'est cette conversion qui l'en protège.
#[derive(Deserialize)]
struct Fichier {
    verses: Vec<Ligne>,
}

#[derive(Deserialize)]
struct Ligne {
    b: String,
    c: String,
    n: u32,
    r: String,
    t: String,
}

pub struct VivierEmbarque {
    versets: Vec<VersetQuotidien>,
}

impl VivierEmbarque {
    /// Analyse le vivier.
    ///
    /// À faire une fois, au démarrage : la racine de composition le garde et
    /// le prête. Le refaire à chaque requête coûterait l'analyse de 58 Ko de
    /// JSON pour un résultat identique.
    pub fn charger() -> Result<Self, serde_json::Error> {
        let fichier: Fichier = serde_json::from_str(SOURCE)?;
        Ok(Self {
            versets: fichier
                .verses
                .into_iter()
                .map(|l| VersetQuotidien {
                    livre: l.b,
                    unite: l.c,
                    numero: l.n,
                    renvoi: l.r,
                    texte: l.t,
                })
                .collect(),
        })
    }
}

impl Vivier for VivierEmbarque {
    fn versets(&self) -> &[VersetQuotidien] {
        &self.versets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_vivier_se_lit() {
        let vivier = VivierEmbarque::charger().expect("daily.json illisible");
        let versets = vivier.versets();

        // Le pipeline en produit 251 — unités verrouillées seulement. Le
        // nombre bougera à chaque verrouillage ; ce qui ne doit jamais bouger,
        // c'est qu'il y en ait.
        assert!(!versets.is_empty(), "vivier vide");

        // Aucun champ vide : un renvoi manquant donnerait une carte sans
        // référence, et un lien mort.
        for v in versets {
            assert!(!v.livre.is_empty() && !v.unite.is_empty(), "{v:?}");
            assert!(!v.renvoi.is_empty() && !v.texte.is_empty(), "{v:?}");
            assert!(v.numero >= 1, "{v:?}");
        }
    }

    #[test]
    fn l_ordre_du_vivier_est_celui_du_fichier() {
        // Le choix du verset est un indice dans ce tableau. Le trier, le
        // dédupliquer ou le filtrer ici ferait diverger le site de l'app sans
        // qu'aucun test de l'un ou de l'autre ne s'en aperçoive.
        let vivier = VivierEmbarque::charger().unwrap();
        let fichier: Fichier = serde_json::from_str(SOURCE).unwrap();
        assert_eq!(vivier.versets().len(), fichier.verses.len());
        assert_eq!(vivier.versets()[0].renvoi, fichier.verses[0].r);
    }
}
