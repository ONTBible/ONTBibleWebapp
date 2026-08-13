use leptos::prelude::*;

use crate::interface::design::{Bloc, Exergue, Hero, LegendeNiveaux, TitreDeSection, Verset};
use crate::interface::echantillon::bereshit_1_1;
use crate::interface::tete::Tete;

/// « Le pourquoi » — l'ontologie fonctionnelle, montrée plutôt qu'affirmée.
///
/// La page s'ouvre comme l'accueil, en plus bas : une ouverture qui remplit
/// l'écran et situe, puis un bloc par idée. La version précédente entassait
/// tout dans un seul bloc sous un titre centré — elle se lisait comme un
/// document déposé sur le site, pas comme le site.
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

        <Hero sobre=true>
            <p class="text-sm uppercase tracking-capitales text-accent">"Le principe"</p>
            <h1 class="text-balance">"Le pourquoi"</h1>
            <p class="max-w-xl text-encre-douce text-balance">
                "Les traductions françaises de la Bible descendent du grec et du latin. "
                "Elles pensent en substance. Le texte hébreu ne pense pas comme ça."
            </p>
        </Hero>

        <Bloc>
            <TitreDeSection numero="I" titre="Une chose existe quand elle a un rôle" />
            <p class="lettrine">
                "Dans le monde hébreu antique, une chose n'existe pas parce qu'elle a une "
                "substance matérielle. Elle existe parce qu'elle a une fonction assignée, un "
                "nom, un rôle dans un système ordonné."
            </p>
            <p>
                "Nommer n'est donc pas étiqueter. Nommer fait entrer dans l'existence. "
                "C'est " <i>"qara"</i> ". Et c'est pourquoi une réalité qui fonctionne "
                "autrement mérite un nom à elle."
            </p>
        </Bloc>

        <Bloc eclaire=true>
            <TitreDeSection numero="II" titre="« Créer » ne veut pas dire fabriquer" />
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
        </Bloc>

        <Bloc>
            <TitreDeSection numero="III" titre="Les trois niveaux du texte" />
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
