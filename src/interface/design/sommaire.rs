use leptos::prelude::*;

use crate::domaine::corpus::Ensemble;

/// Le sommaire du corpus — les 70 livres, écrits ou non.
///
/// ## Pourquoi les livres non écrits restent
///
/// Trois livres sur soixante-dix sont traduits. Un sommaire qui ne montrerait
/// que ces trois-là serait plus flatteur et moins vrai : il laisserait croire
/// que le corpus tient en trois livres, et il effacerait ce que le projet est
/// réellement — un chantier dont on voit le plan entier dès la première visite.
///
/// L'ampleur **est** le propos. Elle se lit d'un coup d'œil : quelques titres
/// en or au milieu de soixante-sept en encre atténuée.
///
/// Ils ne sont donc ni cachés, ni grisés jusqu'à l'illisible — l'encre douce
/// tient 6,5:1, au-delà du seuil AA. Ce qui les distingue est qu'ils ne sont
/// pas cliquables : un lien qui ne mène nulle part est la seule chose qu'un
/// sommaire ne doit pas faire.
#[component]
pub fn Sommaire(ensembles: Vec<Ensemble>) -> impl IntoView {
    ensembles
        .into_iter()
        .map(|ensemble| {
            view! {
                <section class="mb-20 last:mb-0">
                    <h2 class="mb-10 flex items-center gap-4 text-encre-vive">
                        <span class="massif w-8 shrink-0 text-accent"></span>
                        {ensemble.titre}
                    </h2>

                    {ensemble
                        .sections
                        .into_iter()
                        .map(|section| {
                            view! {
                                <div class="mb-12 last:mb-0">
                                    <h3 class="mb-5 text-sm uppercase tracking-capitales text-accent">
                                        {section.titre}
                                    </h3>
                                    <ul class="m-0 list-none p-0">
                                        {section
                                            .livres
                                            .into_iter()
                                            .map(|livre| {
                                                let nom = livre.titre.clone();
                                                let francais = livre.francais.clone();
                                                let hebreu = livre.hebreu.clone();
                                                // Le nom hébreu n'est lu par
                                                // personne à voix haute ici : il
                                                // double le titre latin, et un
                                                // lecteur d'écran le prononcerait
                                                // deux fois.
                                                let cote = view! {
                                                    <span
                                                        aria-hidden="true"
                                                        dir="rtl"
                                                        lang="he"
                                                        class="font-hebreu text-[0.95em] text-encre-douce"
                                                    >
                                                        {hebreu}
                                                    </span>
                                                };
                                                view! {
                                                    <li class="border-b border-filet/40 last:border-0">
                                                        {if livre.ecrit {
                                                            view! {
                                                                <a
                                                                    href=format!("/fr/lire/{}", livre.id)
                                                                    class="flex items-baseline justify-between gap-4 py-3.5 no-underline"
                                                                >
                                                                    <span class="text-accent">
                                                                        {nom}
                                                                        <span class="ms-2.5 text-[0.86em] text-encre-douce">
                                                                            {francais}
                                                                        </span>
                                                                    </span>
                                                                    {cote}
                                                                </a>
                                                            }
                                                                .into_any()
                                                        } else {
                                                            view! {
                                                                <div class="flex items-baseline justify-between gap-4 py-3.5 text-encre-douce">
                                                                    <span>
                                                                        {nom}
                                                                        <span class="ms-2.5 text-[0.86em] opacity-70">
                                                                            {francais}
                                                                        </span>
                                                                    </span>
                                                                    {cote}
                                                                </div>
                                                            }
                                                                .into_any()
                                                        }}
                                                    </li>
                                                }
                                            })
                                            .collect_view()}
                                    </ul>
                                </div>
                            }
                        })
                        .collect_view()}
                </section>
            }
        })
        .collect_view()
}
