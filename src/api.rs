//! La frontière client ↔ serveur.
//!
//! C'est le seul module que les deux côtés partagent en entier, et c'est le
//! seul endroit où l'on sérialise. Les types qui voyagent sont donc des
//! **transports** et non des types du domaine : ils portent la forme du fil,
//! pas les invariants du métier.
//!
//! La conversion est ce qui protège le domaine. Le jour où le fil demandera un
//! champ de plus — un identifiant de fiche, un horodatage — c'est le transport
//! qui l'apprendra, pas `VersetQuotidien`.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Le verset du jour, tel qu'il voyage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersetDuJourDto {
    pub renvoi: String,
    pub texte: String,
    pub chemin: String,
}

/// Le verset du jour.
///
/// Rendu côté serveur, puis sérialisé dans la page : le navigateur n'émet
/// aucune requête pour l'obtenir. C'est ce qui permet à la carte d'être
/// lisible avant que le JavaScript n'arrive — et de le rester s'il n'arrive
/// jamais.
///
/// `Ok(None)` et non une erreur quand le vivier est vide : ce n'est pas une
/// panne, c'est un corpus qui n'a encore rien de verrouillé. La page se tait.
#[server(prefix = "/api", endpoint = "verset-du-jour")]
pub async fn verset_du_jour() -> Result<Option<VersetDuJourDto>, ServerFnError> {
    use std::sync::Arc;

    use crate::application::ports::{Horloge, Vivier};
    use crate::application::verset_du_jour::VersetDuJour;

    // Les deux dépendances viennent de la racine de composition, jamais d'un
    // global : c'est `main.rs` qui décide de quelle horloge et de quel vivier
    // il s'agit, et personne d'autre.
    let horloge = use_context::<Arc<dyn Horloge>>()
        .ok_or_else(|| ServerFnError::new("horloge absente du contexte"))?;
    let vivier = use_context::<Arc<dyn Vivier>>()
        .ok_or_else(|| ServerFnError::new("vivier absent du contexte"))?;

    Ok(VersetDuJour::new(horloge.as_ref(), vivier.as_ref())
        .aujourd_hui()
        .map(|v| VersetDuJourDto {
            renvoi: v.renvoi.clone(),
            texte: v.texte.clone(),
            chemin: v.chemin(),
        }))
}
