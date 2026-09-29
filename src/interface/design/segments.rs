//! Le sélecteur segmenté — `ONTSegments` chez l'app.

use leptos::prelude::*;

/// Un sélecteur segmenté : des parts d'un même tout, dont une seule vaut.
///
/// ## Ce qu'il dit, et qu'un menu ne dirait pas
///
/// Les segments sont visibles **tous à la fois**. Un menu déroulant cacherait
/// les trois autres, et le lecteur ne saurait pas qu'ils existent — or
/// « Vocabulaire fixé » est précisément ce qu'il ne pense pas à chercher.
///
/// C'est la même raison qui fait que le sommaire montre les soixante-sept
/// livres non traduits : **l'ampleur est le propos**, et elle ne se lit que si
/// on la voit.
///
/// ## Le curseur glisse, il ne clignote pas
///
/// Le fond du segment choisi se déplace avec le ressort de l'app — c'est ce
/// que fait iOS, et c'est ce qui dit que les segments sont **une** chose à
/// quatre positions plutôt que quatre boutons. Sans le glissement, on lit
/// quatre boutons dont un est allumé.
#[component]
pub fn Segments<T>(
    /// Les segments, dans l'ordre. La valeur, puis son libellé.
    parts: Vec<(T, &'static str)>,
    /// Ce qui est choisi.
    choisi: RwSignal<T>,
    #[prop(into)] nom: String,
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    view! {
        <div
            role="radiogroup"
            aria-label=nom
            class="mb-6 flex rounded-full border border-filet/60 bg-surface/60 p-1"
        >
            {parts
                .into_iter()
                .map(|(valeur, libelle)| {
                    let pour_comparer = valeur.clone();
                    let actif = Signal::derive(move || choisi.get() == pour_comparer);
                    view! {
                        <button
                            type="button"
                            role="radio"
                            aria-checked=move || actif.get().to_string()
                            on:click=move |_| choisi.set(valeur.clone())
                            // Le fond et l'encre bougent avec `ressort`, pas
                            // avec une durée posée là : c'est la signature de
                            // l'app, et elle dépasse sa cible de trois pour
                            // cent. Le dépassement ne se remarque pas ; c'est
                            // son absence qui se remarque.
                            class=move || {
                                let base = "segment flex-1 rounded-full px-3 py-1.5 \
                                            font-titre text-[0.78rem] leading-none \
                                            focus-visible:outline focus-visible:outline-2 \
                                            focus-visible:outline-offset-2 \
                                            focus-visible:outline-accent";
                                if actif.get() {
                                    format!("{base} bg-encre/10 text-marque-encre")
                                } else {
                                    format!("{base} text-encre-douce hover:text-encre")
                                }
                            }
                        >
                            {libelle}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Le rail alphabétique — on le touche, la liste saute.
///
/// ## Ce que la version de l'app fait et que celle-ci ne fait pas
///
/// Là-bas, c'est **un seul geste continu** : on pose le pouce et on descend,
/// avec une vibration à chaque lettre franchie. Son commentaire dit pourquoi
/// ce n'est pas une rangée de boutons — *viser une lettre haute de onze points
/// est impossible en marchant.*
///
/// Ici ce sont des ancres. Un glissement continu demanderait d'écouter
/// `touchmove` et de calculer la lettre sous le doigt, c'est-à-dire de
/// réimplémenter en JavaScript ce que le navigateur fait déjà pour un lien —
/// et la vibration n'existe pas dans Safari sur iOS.
///
/// **Ce qu'on garde du geste** : le saut est doux, parce que `scroll-behavior:
/// smooth` est posé sur `html` depuis le seuil de l'accueil. On ne se retrouve
/// donc pas ailleurs sans avoir vu qu'on y allait.
#[component]
pub fn RailDeLettres(lettres: Vec<char>) -> impl IntoView {
    view! {
        <nav
            aria-label="Aller à une lettre"
            // Collé au bord droit et centré dans la hauteur, comme chez l'app.
            // `pointer-events-none` sur le cadre, `auto` sur les lettres : le
            // rail est étroit, et le reste de sa colonne doit rester au texte.
            class="pointer-events-none fixed end-1 top-1/2 z-30 hidden -translate-y-1/2 flex-col items-center gap-0.5 sm:flex"
        >
            {lettres
                .into_iter()
                .map(|lettre| {
                    view! {
                        <a
                            href=format!("#lettre-{lettre}")
                            // Onze points chez l'app. Ici en `rem`, pour que la
                            // taille de police du lecteur le fasse grandir
                            // avec le reste — un rail figé devient intouchable
                            // au cran où l'on en a le plus besoin.
                            class="pointer-events-auto px-1.5 py-px font-titre text-[0.68rem] font-semibold leading-none text-encre-douce/70 no-underline transition-colors hover:text-accent"
                        >
                            {lettre.to_string()}
                        </a>
                    }
                })
                .collect_view()}
        </nav>
    }
}
