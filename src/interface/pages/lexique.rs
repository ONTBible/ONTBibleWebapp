use leptos::prelude::*;

use crate::api::lexique;
use crate::interface::design::PageDeLecture;
use crate::interface::tete::Tete;

/// `/fr/lexique` — l'index des intraduisibles.
///
/// La liste de ce que la traduction a décidé de **ne pas** traduire. C'est
/// peut-être la page qui dit le mieux ce qu'est l'ONT : cent cinq mots qu'une
/// traduction ordinaire aurait rendus, et qui sont restés debout parce que les
/// rendre aurait coûté ce qu'ils portent.
///
/// Chaque entrée montre son **rendu** — « l'Être façonné du sol
/// (Bereshit 1-7) » — et non sa définition. Le rendu est la décision ; la
/// définition est ce qui la justifie, et elle a sa page.
#[component]
pub fn Lexique() -> impl IntoView {
    let entrees = Resource::new_blocking(|| (), |_| async { lexique().await });

    view! {
        <Tete
            // Le titre indexable dit ce que la page contient ; le titre
            // visible reste « Lexique », qui suffit à qui est arrivé.
            // « Lexique » seul ne répond à aucune recherche : c'est un mot
            // de sommaire, pas un mot de question.
            titre="Lexique des intraduisibles hébreux"
            description="Les intraduisibles de La Bible ONT — les termes hébreux et araméens \
                         laissés debout, et pourquoi."
            chemin="/fr/lexique"
        />

        <PageDeLecture
            rappel="Les intraduisibles"
            titre="Lexique"
            chapeau=Box::new(|| {
                view! {
                    <p class="text-encre-douce text-pretty">
                        "Ce que la restitution a choisi de ne pas traduire, et comment elle \
                         le rend. Chaque mot d'or du corpus mène ici."
                    </p>
                }
                    .into_any()
            })
        >
            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    match entrees.await {
                        Ok(entrees) => {
                            view! {
                                <ul class="m-0 list-none p-0">
                                    {entrees
                                        .into_iter()
                                        .map(|entree| {
                                            view! {
                                                <li class="border-b border-filet/40 last:border-0">
                                                    <a
                                                        href=format!("/fr/lexique/{}", entree.lemme)
                                                        class="block py-4 no-underline"
                                                    >
                                                        <span class="flex items-baseline justify-between gap-4">
                                                            <span class="font-semibold text-accent">
                                                                {entree.titre}
                                                            </span>
                                                            <span
                                                                aria-hidden="true"
                                                                dir="rtl"
                                                                lang="he"
                                                                class="font-hebreu text-[1.05em] text-encre-douce"
                                                            >
                                                                {entree.hebreu}
                                                            </span>
                                                        </span>
                                                        {(!entree.rendu.is_empty())
                                                            .then(|| {
                                                                view! {
                                                                    <span class="mt-1 block text-[0.92em] text-encre-douce">
                                                                        {entree.rendu}
                                                                    </span>
                                                                }
                                                            })}
                                                    </a>
                                                </li>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            }
                                .into_any()
                        }
                        Err(_) => ().into_any(),
                    }
                })}
            </Suspense>
        </PageDeLecture>
    }
}
