//! L'adresse d'une image, empreinte comprise.
//!
//! ## Pourquoi ça existe
//!
//! Les feuilles et le WASM portent leur empreinte dans le nom du fichier —
//! `ontbible.<empreinte>.css`. Un contenu nouveau a une adresse nouvelle, donc
//! aucun cache ne peut servir l'ancien. C'est le §8 ter, et c'est la seule
//! chose qui rende un cache long acceptable.
//!
//! Les images n'avaient pas ce traitement. `wordmark.svg` garde son nom quand
//! son dessin change, et le déploiement le sert avec un cache d'une journée :
//! **une image corrigée reste invisible jusqu'à vingt-quatre heures** pour qui
//! a déjà visité le site.
//!
//! Le ® a été retiré de la marque le 13 août 2026 ; le lendemain on en
//! débattait encore, en regardant deux versions différentes du même fichier.
//! La production était juste depuis le début — c'était un cache de navigateur.
//!
//! ## Ce que ça change, et ce que ça ne change pas
//!
//! Le fichier garde son nom sur le seau. Seule l'**adresse** change :
//! `/images/wordmark.svg?v=k3x9p2q`. Le navigateur y voit une autre ressource
//! et la retélécharge ; S3 ignore le paramètre et sert le même fichier.
//!
//! Le bord CloudFront, lui, n'entre pas le paramètre dans sa clé de cache — sa
//! politique le dit — donc c'est l'invalidation de `/images/*` au déploiement
//! qui s'en charge. Les deux couches sont couvertes, chacune par le moyen qui
//! lui convient.
//!
//! ## Une image inconnue passe telle quelle
//!
//! Sans empreinte plutôt qu'avec une fausse. Ce n'est pas une tolérance : un
//! nom mal orthographié donne un 404 franc, que la page rende `?v=` ou non, et
//! ajouter un paramètre inventé masquerait la faute d'un jour de cache
//! supplémentaire.
//!
//! Le test plus bas exige que **toute** image du dossier soit trouvée, et le
//! `rerun-if-changed` de `build.rs` porte sur le dossier : une image ajoutée
//! entre dans la table sans que personne n'ait à y penser.

// `EMPREINTES: &[(&str, &str)]` — le nom du fichier et l'empreinte de son
// contenu, engendrés par `build.rs`.
include!(concat!(env!("OUT_DIR"), "/images.rs"));

/// L'adresse d'une image de `public/images/`, empreinte comprise.
///
/// On passe le **nom du fichier**, pas le chemin : `image("wordmark.svg")`. Le
/// dossier est une constante du site, et le répéter à chaque appel invite à
/// écrire `/image/` un jour où l'on va vite.
pub fn image(nom: &str) -> String {
    match EMPREINTES.iter().find(|(fichier, _)| *fichier == nom) {
        Some((_, empreinte)) => format!("/images/{nom}?v={empreinte}"),
        None => format!("/images/{nom}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn une_image_connue_porte_son_empreinte() {
        let adresse = image("wordmark.svg");
        assert!(adresse.starts_with("/images/wordmark.svg?v="), "{adresse}");
        assert!(
            adresse.len() > "/images/wordmark.svg?v=".len(),
            "empreinte vide : {adresse}"
        );
    }

    #[test]
    fn deux_images_differentes_ont_deux_empreintes() {
        // Une empreinte constante ne servirait à rien, et ça ne se verrait pas
        // en regardant une seule adresse.
        assert_ne!(image("wordmark.svg"), image("logomark.svg"));
    }

    /// Toute image du dossier est dans la table.
    ///
    /// C'est ce qui rend la porte de sortie acceptable : sans ce test, une
    /// image ajoutée pourrait sortir sans empreinte, et personne ne le verrait
    /// — la page s'afficherait, elle serait simplement périmée un jour de plus.
    #[test]
    fn le_dossier_entier_est_couvert() {
        let dossier = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("public/images");
        let fichiers: Vec<String> = std::fs::read_dir(&dossier)
            .expect("public/images illisible")
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|c| c.is_file())
            .filter_map(|c| c.file_name()?.to_str().map(str::to_string))
            .collect();

        assert!(!fichiers.is_empty(), "aucune image trouvée");
        for fichier in fichiers {
            assert!(
                image(&fichier).contains("?v="),
                "« {fichier} » n'a pas d'empreinte — relancer une compilation propre"
            );
        }
    }
}
