use leptos::prelude::*;

use crate::interface::design::{Bloc, Exergue, Filet, LegendeNiveaux, TitreDePage, Verset};
use crate::interface::echantillon::bereshit_1_1;
use crate::interface::tete::Tete;

/// « Le pourquoi » — l'ontologie fonctionnelle, montrée plutôt qu'affirmée.
#[component]
pub fn Pourquoi() -> impl IntoView {
    view! {
        <Tete
            titre="Le pourquoi"
            description="Dans le monde hébreu antique, une chose existe parce qu'elle a une \
                         fonction, pas parce qu'elle a une substance. Bara ne veut pas dire \
                         fabriquer."
            chemin="/fr/le-pourquoi"
        />

        <Bloc>
            <TitreDePage rappel="Le principe" titre="Le pourquoi" />

            <p>
                "Les traductions françaises de la Bible descendent du grec et du latin. "
                "Elles pensent en substance : une chose est réelle parce qu'elle est faite "
                "de quelque chose. Le texte hébreu ne pense pas comme ça."
            </p>

            <h2>"Une chose existe quand elle a un rôle"</h2>
            <p>
                "Dans le monde hébreu antique, une chose n'existe pas parce qu'elle a une "
                "substance matérielle. Elle existe parce qu'elle a une fonction assignée, un "
                "nom, un rôle dans un système ordonné."
            </p>
            <p>
                "Nommer n'est donc pas étiqueter. Nommer fait entrer dans l'existence. "
                "C'est " <i>"qara"</i> ". Et c'est pourquoi une réalité qui fonctionne "
                "autrement mérite un nom à elle."
            </p>

            <h2>"« Créer » ne veut pas dire fabriquer"</h2>
            <p>
                "« Au commencement Dieu créa le ciel et la terre » suppose un atelier, de la "
                "matière, un avant et un après. Rien de tout cela n'est dans le verbe hébreu."
            </p>
            <p>
                <i>"Bara"</i> " n'est pas un acte d'artisan. C'est un acte de roi : inaugurer "
                "un espace, attribuer des rôles, mettre en fonction. Le cosmos ne sort pas "
                "d'une usine. Il est inauguré comme on inaugure un Temple — et un Temple "
                "commence à exister le jour où l'on y entre pour y résider."
            </p>

            <Exergue>
                "Le récit ne raconte pas la fabrication de la matière. "
                "Il raconte la mise en ordre d'un sanctuaire."
            </Exergue>

            <Filet orne=true />

            <h2>"Les trois niveaux du texte"</h2>
            <p>
                "Une restitution ne peut pas tout dire dans la même ligne. L'ONT sépare donc "
                "ce que l'hébreu dit, ce qu'il porte implicitement, et ce qu'il dit "
                "littéralement. Trois niveaux, jamais confondus."
            </p>

            <Verset verset=bereshit_1_1() />
            <LegendeNiveaux />
        </Bloc>
    }
}
