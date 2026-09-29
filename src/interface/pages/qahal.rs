use leptos::prelude::*;

use crate::api::verset_du_jour;
use crate::interface::design::{fournir_preferences, CarteVersetDuJour, PageDeLecture};
use crate::interface::tete::Tete;

/// `/fr/qahal` — l'assemblée.
///
/// ## Le nom dit la symétrie
///
/// *Qahal* (קָהָל), l'assemblée. Le commentaire de l'app la pose d'un trait, et
/// c'est elle qui justifie l'onglet : la **Kenesset** est le rassemblement des
/// *textes*, le **Qahal** celui des *lecteurs*.
///
/// ## Ce qu'il porte, et ce qu'il refuse de porter
///
/// Le verset du jour, et l'annonce de ce qui vient. Rien d'autre, et c'est une
/// décision de l'app qu'on reprend sans la discuter : *« tout ce qui suppose
/// d'autres lecteurs est annoncé sans être simulé — un faux fil d'activité
/// donnerait une idée fausse de ce qui existe »*.
///
/// C'est la règle du badge App Store, rejouée : une bannière n'a que deux
/// états justes, et « allumée vers rien » n'en est pas un.
///
/// ## Pourquoi cet onglet peut exister ici et pas Chuqqot
///
/// Parce qu'il a quelque chose à montrer **aujourd'hui**. Le verset du jour
/// est déjà porté par le site, et la même fonction de la date que l'app : le
/// lecteur qui l'a vu sur son téléphone ce matin retrouve le même ici.
///
/// Les **chuqqot**, non : quatorze sont écrites et retenues dans les
/// brouillons du vault jusqu'à ce que l'auteur les valide — un énoncé
/// permanent ne voyage pas « en attente de validation ». Rien n'atteint donc
/// les liseuses aujourd'hui, et un onglet Chuqqot serait exactement la chose
/// qu'on refuse au paragraphe précédent.
///
/// ## La carte garde sa mesure
///
/// L'app le dit, et c'est le même piège que la double mesure : *« la colonne
/// de la page fait 850 ; y étaler la carte du verset la transformait en bande,
/// et le verset ne tenait plus que sur deux lignes traversant l'écran »*. La
/// page est donc en registre de liste, et la carte se borne elle-même.
#[component]
pub fn Qahal() -> impl IntoView {
    // La carte du verset rend du corpus, donc des niveaux.
    let _preferences = fournir_preferences();

    let verset = Resource::new_blocking(|| (), |_| async { verset_du_jour().await });

    view! {
        <Tete
            titre="Qahal — l'assemblée des lecteurs"
            description="Le verset du jour de La Bible ONT, et ce que l'assemblée des \
                         lecteurs portera."
            chemin="/fr/qahal"
        />

        <PageDeLecture liste=true titre="Qahal">
            <div class="mx-auto max-w-mesure">
                <Suspense fallback=|| ()>
                    {move || Suspend::new(async move {
                        match verset.await {
                            Ok(Some(v)) => view! { <CarteVersetDuJour verset=v /> }.into_any(),
                            _ => ().into_any(),
                        }
                    })}
                </Suspense>

                <AVenir />
            </div>
        </PageDeLecture>
    }
}

/// Ce que le Qahal portera — annoncé, jamais simulé.
#[component]
fn AVenir() -> impl IntoView {
    // Les trois de l'app, avec ses libellés : un lecteur qui passe du téléphone
    // au site doit retrouver les mêmes mots pour les mêmes choses.
    let entrees = [
        ("Ce que le Qahal a retenu", "les versets les plus repris"),
        ("Échanges", "commenter un passage"),
        ("Parcours", "lire le corpus à plusieurs"),
    ];

    view! {
        <section class="mt-10 rounded-bloc border border-filet bg-surface px-6 py-6">
            <h2 class="m-0 mb-5 text-sm uppercase tracking-capitales text-encre-douce">
                "À venir"
            </h2>
            <ul class="m-0 flex list-none flex-col gap-4 p-0">
                {entrees
                    .into_iter()
                    .map(|(titre, note)| {
                        view! {
                            <li class="flex gap-3">
                                // Un point d'or plutôt qu'un symbole : trois
                                // signes différents feraient croire à trois
                                // destinations, or aucune n'existe encore.
                                <span
                                    aria-hidden="true"
                                    class="mt-2 size-1.5 shrink-0 rounded-full bg-accent"
                                ></span>
                                <span>
                                    <span class="block text-encre-douce">{titre}</span>
                                    <span class="block text-sm text-encre-douce/70">{note}</span>
                                </span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
            // **La phrase de l'app dit « hors ligne », et ce serait faux
            // ici.** Le site n'a délibérément pas de *service worker* (§8
            // quinquies) : la lecture demande le réseau. Ce qu'elle ne demande
            // pas, c'est un compte — et c'est ce qui se dit à sa place, parce
            // que c'est la même promesse : rien de ce qui compte n'attend
            // qu'on s'inscrive.
            <p class="mt-6 mb-0 text-sm text-encre-douce/70">
                "Ces fonctions demandent un serveur. La lecture, elle, ne demande aucun compte."
            </p>
        </section>
    }
}
