//! L'année de compilation, pour la mention de droit d'auteur.
//!
//! Le pied de page est rendu par le serveur **et** par le navigateur : il ne
//! peut donc pas lire l'horloge, qui n'existe que d'un côté. Et une année
//! écrite en dur se périme sans que personne ne s'en aperçoive.
//!
//! On la fige donc à la compilation. Chaque déploiement la rafraîchit, ce qui
//! est exactement le rythme voulu : un site redéployé est un site à jour.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("cargo:rustc-env=ANNEE_DE_COMPILATION={}", annee());
}

/// L'année grégorienne, calculée sans dépendance.
///
/// Le calcul passe par le nombre de jours depuis 1970 et un cycle de 400 ans —
/// la période exacte du calendrier grégorien, bissextiles comprises. Une
/// approximation en « 365,25 jours » dériverait d'un jour tous les siècles, et
/// se tromperait donc d'année un 31 décembre.
fn annee() -> i32 {
    let jours = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("horloge antérieure à 1970")
        .as_secs()
        / 86_400;

    let mut annee = 1970;
    let mut restant = jours as i64;
    loop {
        let longueur = if bissextile(annee) { 366 } else { 365 };
        if restant < longueur {
            return annee;
        }
        restant -= longueur;
        annee += 1;
    }
}

fn bissextile(annee: i32) -> bool {
    (annee % 4 == 0 && annee % 100 != 0) || annee % 400 == 0
}
