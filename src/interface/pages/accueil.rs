use leptos::prelude::*;

use crate::api::verset_du_jour;
use crate::interface::design::{
    Bandeau, CarteVersetDuJour, Filet, Mention, Porte, Portes, Section,
};
use crate::interface::tete::Tete;

/// L'accueil — le principe, puis le verset du jour, puis les portes.
///
/// Le principe est posé dans un bandeau éclairé plutôt qu'à même la nuit :
/// c'est la thèse du projet, et elle doit arriver comme une affirmation, pas
/// comme un chapeau d'article.
#[component]
pub fn Accueil() -> impl IntoView {
    // `new_blocking` : le serveur attend la valeur et la pose dans le HTML.
    // Sans ça, la carte apparaîtrait après coup — absente pour un moteur de
    // recherche comme pour un lecteur sans JavaScript.
    let verset = Resource::new_blocking(|| (), |_| async { verset_du_jour().await });

    view! {
        <Tete
            titre="La Bible ONT"
            description="Une restitution française du corpus hébreu et araméen antique, \
                         fondée sur l'ontologie hébraïque fonctionnelle. Le cosmos hébreu \
                         n'est pas une usine : c'est un Temple."
            chemin="/fr"
        />

        <h1 class="sr-only">"La Bible ONT"</h1>

        <Bandeau>
            <p class="text-lg leading-relaxed text-pretty">
                "Une chose n'existe pas parce qu'elle occupe de la place. "
                "Elle existe parce qu'elle a une fonction, un nom, un rôle dans un ordre."
            </p>
            <p class="mt-6 text-lg leading-relaxed text-pretty">
                "Créer ne veut pas dire fabriquer. Créer veut dire ordonner, nommer, assigner."
            </p>
            <p class="mt-10 text-xl leading-snug text-balance">
                "Le cosmos hébreu n'est pas une usine. C'est un Temple."
            </p>
        </Bandeau>

        <Section>
            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    // Une panne du verset du jour ne doit pas emporter la page :
                    // elle se tait, et le reste tient.
                    match verset.await {
                        Ok(Some(v)) => view! { <CarteVersetDuJour verset=v /> }.into_any(),
                        _ => ().into_any(),
                    }
                })}
            </Suspense>

            <Filet orne=true />

            <Portes>
                <Porte
                    href="/fr/le-pourquoi"
                    titre="Le pourquoi"
                    glose="L'ontologie fonctionnelle, et les trois niveaux du texte."
                />
                <Porte
                    href="/fr/ce-que-l-ont-n-est-pas"
                    titre="Ce que l'ONT n'est pas"
                    glose="Cinq lignes. Elles suffisent."
                />
                <Porte
                    href="/fr/l-auteur"
                    titre="L'auteur"
                    glose="D'où vient ce travail, et comment il se fait."
                />
            </Portes>

            <Mention>
                "La restitution est en cours. Les unités verrouillées font référence ; "
                "les autres sont des brouillons, et le disent."
            </Mention>
        </Section>
    }
}
