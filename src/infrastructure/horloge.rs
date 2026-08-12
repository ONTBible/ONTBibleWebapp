//! L'horloge du système.

use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Europe::Paris;

use crate::application::ports::Horloge;

/// L'horloge de l'édition — Europe/Paris.
///
/// ## Pourquoi pas UTC
///
/// Le serveur tourne en UTC. Le verset changerait donc à 1 h ou 2 h du matin
/// heure française : un lecteur qui ouvre le site à minuit et demi verrait
/// celui de la veille.
///
/// ## Le décalage d'un jour, et pourquoi on le reproduit
///
/// L'app calcule son numéro de jour ainsi (`DailySelection`) :
///
/// ```swift
/// let jour = calendar.startOfDay(for: date)          // minuit LOCAL
/// let numero = Int(jour.timeIntervalSince1970 / 86_400)  // divisé comme de l'UTC
/// ```
///
/// `startOfDay` rend minuit dans le fuseau du lecteur ; la division traite cet
/// instant comme s'il était UTC. Pour tout fuseau à l'est de Greenwich, le
/// numéro obtenu vaut donc **la date civile moins un**. À Paris, minuit vaut
/// 22 h ou 23 h UTC la veille.
///
/// Ce n'est pas un défaut d'affichage : le verset change bien à minuit chez le
/// lecteur, seul l'entier est décalé. Et comme la fonction de choix est une
/// permutation de cet entier, un décalage d'un jour donne un **autre verset**.
///
/// Le site doit donc reproduire le calcul de l'app, pas le corriger. Corriger
/// ici ferait diverger le site du téléphone du même lecteur, le même matin —
/// exactement ce que §7 interdit. Le jour où l'app changera, ce module
/// changera avec elle, et les témoins ci-dessous le diront.
///
/// ## La réserve, et elle est réelle
///
/// L'app suit le fuseau de **l'appareil** ; le site suit celui de l'édition.
/// Pour un lecteur à Tokyo, les deux divergent. Le site ne peut pas faire
/// mieux sans connaître son fuseau, et le lui demander coûterait soit un rendu
/// vide au premier affichage, soit un cookie — deux prix plus élevés que le
/// défaut.
#[derive(Default)]
pub struct HorlogeSysteme;

impl Horloge for HorlogeSysteme {
    fn jour(&self) -> i64 {
        numero_de_jour(Utc::now())
    }
}

/// Le numéro de jour d'un instant, tel que l'app le calcule.
///
/// Séparé de l'horloge pour être éprouvable : on ne teste pas « maintenant »,
/// on teste des instants connus dont on possède la réponse de l'original.
pub fn numero_de_jour(instant: DateTime<Utc>) -> i64 {
    let minuit_local = instant
        .with_timezone(&Paris)
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .expect("minuit existe toujours");

    // Le passage à l'heure d'été saute de 2 h à 3 h : minuit n'est jamais
    // ambigu ni absent à Paris. `earliest` reste le repli du repli.
    let horodatage = Paris
        .from_local_datetime(&minuit_local)
        .earliest()
        .expect("minuit local existe à Paris")
        .timestamp();

    // `div_euclid` et non une division simple : elle arrondit vers le bas même
    // pour un horodatage négatif, là où la division tronquerait vers zéro et
    // se tromperait d'un jour avant 1970.
    horodatage.div_euclid(86_400)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Les numéros que produit l'original, relevés en exécutant le Swift.
    ///
    /// Le témoin précédent employait un calendrier en **UTC** — et il passait,
    /// tout en laissant le site annoncer un autre verset que l'app. C'est le
    /// fuseau de l'édition qu'il fallait éprouver, puisque c'est celui du
    /// lecteur visé.
    fn temoin(iso: &str) -> i64 {
        numero_de_jour(iso.parse::<DateTime<Utc>>().expect("instant valide"))
    }

    #[test]
    fn le_numero_de_jour_s_accorde_avec_l_original_swift() {
        // Heure d'été — Paris est à UTC+2, minuit local vaut 22 h UTC la veille.
        assert_eq!(temoin("2026-08-12T10:00:00Z"), 20_676);
        // Heure d'hiver — UTC+1.
        assert_eq!(temoin("2026-01-15T10:00:00Z"), 20_467);
    }

    #[test]
    fn le_verset_change_a_minuit_chez_le_lecteur() {
        // 23 h 30 UTC, c'est 1 h 30 à Paris : on est déjà le lendemain pour le
        // lecteur, et le numéro a déjà changé.
        assert_eq!(temoin("2026-08-11T21:30:00Z"), 20_675);
        assert_eq!(temoin("2026-08-11T23:30:00Z"), 20_676);
    }

    #[test]
    fn aujourd_hui_est_plausible() {
        // Un garde-fou grossier : entre 2020 et 2100. Il n'attrape pas une
        // erreur d'un jour — c'est le rôle des témoins — mais il attrape un
        // facteur mille.
        let jour = HorlogeSysteme.jour();
        assert!((18_262..47_482).contains(&jour), "jour improbable : {jour}");
    }
}
