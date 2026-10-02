//! Où le lecteur en était — retenu par le navigateur, et non par le compte.
//!
//! ## Pourquoi elle ne demande rien
//!
//! « Reprendre » n'existait qu'avec un compte : `ma_position()` interroge le
//! backend, et sans session elle rend `None`. Le §8 undecies le défendait ainsi —
//! *lire sans compte est le cas normal du site, et « connectez-vous pour
//! reprendre » ferait de la lecture une chose qu'on mérite.*
//!
//! L'argument visait le **message**, pas l'absence. Le retirer entièrement
//! revenait à faire de la reprise une chose qu'on mérite, par omission au lieu
//! de le dire. L'auteur a tranché le 2 octobre 2026 : *qu'elle marche sans
//! compte.*
//!
//! ==Une place gardée sur l'appareil ne demande rien et ne révèle rien : elle ne
//! le quitte pas.== Le backend note que les marques d'un lecteur de Bible,
//! rattachées à une identité, révèlent des convictions religieuses — article 9
//! du RGPD. Un signet qui n'est rattaché à personne et qui reste dans le
//! navigateur ne porte pas cette charge.
//!
//! Le compte garde donc son rôle, et il change de nature : il ne **donne** plus
//! la reprise, il la **transporte** d'un appareil à l'autre.
//!
//! ## La plus récente gagne, et c'est `updated_at` qui tranche
//!
//! Les deux sources portent le même horodatage en millisecondes — le serveur
//! l'écrit avec `SystemTime`, le navigateur avec `Date.now()`. Comparer deux
//! nombres suffit, et c'est la règle du backend reprise telle quelle : *dernier
//! écrit gagné.*
//!
//! Sans cette comparaison, il faudrait choisir une source qui l'emporte toujours
//! — et chacune des deux a tort la moitié du temps : le compte est en retard sur
//! l'appareil qu'on tient, l'appareil est en retard sur celui qu'on vient de
//! quitter.

use crate::domaine::surlignage::Position;

/// La clé du stockage. À côté de `ont.lecture`, et séparée d'elle : les
/// réglages se perdent sans conséquence, une place se perd une fois.
#[cfg(feature = "hydrate")]
const CLE: &str = "ont.position";

#[cfg(feature = "hydrate")]
fn stockage() -> Option<web_sys::Storage> {
    // `local_storage()` lève quand le stockage est refusé — navigation privée
    // stricte, cookies bloqués. Une place perdue n'est pas une panne : la page
    // se lit entière, elle ne propose simplement rien à reprendre.
    web_sys::window()?.local_storage().ok().flatten()
}

/// La place retenue sur cet appareil, s'il y en a une.
///
/// Hors du navigateur — le rendu du serveur, une épreuve — elle rend `None`, et
/// c'est juste : le serveur ne connaît que ce que le compte lui dit.
pub fn lire() -> Option<Position> {
    #[cfg(feature = "hydrate")]
    {
        let brut = stockage()?.get_item(CLE).ok().flatten()?;
        // Une valeur illisible est jetée plutôt que gardée : elle vient d'une
        // version d'avant, et une place fausse enverrait le lecteur ailleurs
        // que là où il s'était arrêté.
        serde_json::from_str(&brut).ok()
    }
    #[cfg(not(feature = "hydrate"))]
    None
}

/// Retient la place, en datant l'écriture.
///
/// Appelée partout où `retenir_la_position` l'est, et pour la même raison : à
/// l'ouverture d'une unité, puis à chaque verset franchi. Les deux écritures
/// vont de pair — ==une place qui ne vaut que pour qui a un compte est une place
/// qu'on perd en se déconnectant.==
#[allow(unused_variables)]
pub fn retenir(livre: &str, unite: &str, titre: &str, verset: u32) {
    #[cfg(feature = "hydrate")]
    {
        let Some(stockage) = stockage() else { return };
        let position = Position {
            book_id: livre.to_string(),
            chapter_id: unite.to_string(),
            chapter_title: titre.to_string(),
            verse: verset,
            // En millisecondes, comme le serveur : c'est ce qui permet de
            // comparer les deux sources sans convertir.
            updated_at: js_sys::Date::now() as i64,
        };
        if let Ok(json) = serde_json::to_string(&position) {
            let _ = stockage.set_item(CLE, &json);
        }
    }
}

/// Des deux places, celle qu'on propose : la plus récemment écrite.
///
/// Prend la valeur du compte quand elle existe et qu'elle est plus fraîche, la
/// locale sinon. À égalité d'horodatage, c'est la locale — on est sur
/// l'appareil qui l'a écrite, et un aller-retour de synchronisation rend
/// souvent la même milliseconde.
pub fn la_plus_fraiche(du_compte: Option<Position>, locale: Option<Position>) -> Option<Position> {
    match (du_compte, locale) {
        (Some(compte), Some(locale)) => Some(if locale.updated_at >= compte.updated_at {
            locale
        } else {
            compte
        }),
        (compte, None) => compte,
        (None, locale) => locale,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(unite: &str, quand: i64) -> Position {
        Position {
            book_id: "bereshit".into(),
            chapter_id: unite.into(),
            chapter_title: "Chapitre 1".into(),
            verse: 1,
            updated_at: quand,
        }
    }

    /// La plus récente l'emporte, d'où qu'elle vienne.
    ///
    /// La propriété est écrite dans les deux sens : une règle qui ne vérifie
    /// qu'un sens passe aussi quand la source gagnante est écrite en dur.
    #[test]
    fn la_place_la_plus_recente_l_emporte() {
        let vieux = position("bereshit-1", 100);
        let neuf = position("bereshit-9", 200);

        assert_eq!(
            la_plus_fraiche(Some(vieux.clone()), Some(neuf.clone())).map(|p| p.chapter_id),
            Some("bereshit-9".to_string()),
            "la locale est plus fraîche, c'est elle qu'on propose"
        );
        assert_eq!(
            la_plus_fraiche(Some(neuf.clone()), Some(vieux.clone())).map(|p| p.chapter_id),
            Some("bereshit-9".to_string()),
            "le compte est plus frais, c'est lui qu'on propose"
        );
    }

    /// Une seule source suffit — c'est tout l'objet du changement.
    #[test]
    fn une_place_sans_compte_est_une_place() {
        let locale = position("bereshit-3", 1);
        assert_eq!(
            la_plus_fraiche(None, Some(locale)).map(|p| p.chapter_id),
            Some("bereshit-3".to_string()),
            "sans compte, la place retenue sur l'appareil doit être proposée"
        );
        assert!(
            la_plus_fraiche(None, None).is_none(),
            "sans rien, on ne propose rien — et surtout pas une invitation à se connecter"
        );
    }
}
