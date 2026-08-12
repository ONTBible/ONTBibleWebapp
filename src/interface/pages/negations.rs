use leptos::prelude::*;

use crate::interface::design::{Bloc, Filet, ListeAffirmations, Principe, TitreDePage};
use crate::interface::tete::Tete;

/// « Ce que l'ONT n'est pas ».
///
/// Les cinq lignes sont celles du vault (§10), reprises telles quelles. C'est
/// la page la plus courte du site et la seule qui prenne un risque : elle dit
/// ce que le projet refuse, donc elle se laisse contredire.
#[component]
pub fn Negations() -> impl IntoView {
    view! {
        <Tete
            titre="Ce que l'ONT n'est pas"
            description="Ni littéralisme, ni paraphrase, ni traduction confessionnelle. \
                         L'ONT affirme, il ne polémique pas."
            chemin="/fr/ce-que-l-ont-n-est-pas"
        />

        <Bloc>
            <TitreDePage rappel="Les limites" titre="Ce que l'ONT n'est pas" />

            <ListeAffirmations lignes=vec![
                "Ce n'est pas une traduction littéraliste mot à mot.",
                "Ce n'est pas une paraphrase libre.",
                "Ce n'est pas une traduction confessionnelle — ni protestante, ni catholique, ni juive.",
                "Ce n'est pas une réfutation d'autres traductions. L'ONT affirme, il ne polémique pas.",
                "Ce n'est pas une imposition de théologie moderne sur le texte ancien.",
            ] />

            <Filet orne=true />

            <Principe chute=true>
                "L'ONT est une restitution de ce que le texte hébreu disait à ses lecteurs "
                "originaux — en rendant visible pour le lecteur français ce qui était "
                "invisible parce qu'implicite."
            </Principe>
        </Bloc>
    }
}
