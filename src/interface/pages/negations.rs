use leptos::prelude::*;

use crate::interface::design::{Bloc, Hero, ListeAffirmations, Principe};
use crate::interface::tete::Tete;

/// « Ce que l'ONT n'est pas ».
///
/// Les cinq lignes sont celles du vault (§10), reprises telles quelles. C'est
/// la page la plus courte du site et la seule qui prenne un risque : elle dit
/// ce que le projet refuse, donc elle se laisse contredire.
///
/// Elle n'a qu'un bloc après son ouverture, et c'est voulu : cinq refus tiennent
/// sur un écran, et les étaler sur trois affaiblirait chacun.
#[component]
pub fn Negations() -> impl IntoView {
    view! {
        <Tete
            titre="Ce que l'ONT n'est pas"
            description="Ni littéralisme, ni paraphrase, ni traduction confessionnelle. \
                         L'ONT affirme, il ne polémique pas."
            chemin="/fr/ce-que-l-ont-n-est-pas"
        />

        <Hero sobre=true>
            <p class="text-sm uppercase tracking-capitales text-accent">"Les limites"</p>
            <h1 class="text-balance">"Ce que l'ONT n'est pas"</h1>
            <p class="max-w-xl text-encre-douce text-balance">
                "Cinq refus. Ils disent le projet aussi précisément que ce qu'il affirme."
            </p>
        </Hero>

        <Bloc eclaire=true>
            <ListeAffirmations lignes=vec![
                "Ce n'est pas une traduction littéraliste mot à mot.",
                "Ce n'est pas une paraphrase libre.",
                "Ce n'est pas une traduction confessionnelle — ni protestante, ni catholique, ni juive.",
                "Ce n'est pas une réfutation d'autres traductions. L'ONT affirme, il ne polémique pas.",
                "Ce n'est pas une imposition de théologie moderne sur le texte ancien.",
            ] />

            <div class="mt-16">
                <Principe chute=true>
                    "L'ONT est une restitution de ce que le texte hébreu disait à ses lecteurs "
                    "originaux — en rendant visible pour le lecteur français ce qui était "
                    "invisible parce qu'implicite."
                </Principe>
            </div>
        </Bloc>
    }
}
