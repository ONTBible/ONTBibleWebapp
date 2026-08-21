use leptos::prelude::*;

/// Ce qu'un mot est devenu en changeant de langue.
///
/// Chaque entrée descend : l'hébreu, le grec qui l'a traduit, le français qui
/// en a hérité. Trois états d'un même mot, dans l'ordre du temps.
///
/// **La dégradation est dans la couleur, pas dans le commentaire.** L'hébreu
/// est en or plein — c'est l'intraduisible du corpus, la valeur la plus haute
/// de la rampe. Le grec est en encre. Le français est en encre douce, entre
/// guillemets, deux crans plus bas. On voit la lumière baisser avant d'avoir
/// lu la colonne de droite, et c'est ce qui rend l'argument sensible plutôt
/// que su.
///
/// Les guillemets autour du mot français ne sont pas un ornement : ils disent
/// que c'est **une citation d'un usage**, pas une traduction que l'ONT
/// propose. On rapporte ce que le mot est devenu ; on ne le reprend pas.
#[component]
pub fn Correspondances(children: Children) -> impl IntoView {
    view! { <ul class="m-0 mt-12 list-none p-0">{children()}</ul> }
}

/// Un mot, et ce qu'il est devenu.
#[component]
pub fn Correspondance(
    /// Le mot hébreu, pointé.
    #[prop(into)]
    hebreu: String,
    /// Sa translittération.
    #[prop(into)]
    translitteration: String,
    /// Ce que l'hébreu dit — court, une ligne.
    #[prop(into)]
    sens: String,
    /// Le mot grec que la Septante a posé.
    #[prop(into)]
    grec: String,
    /// Sa translittération.
    #[prop(into)]
    grec_translitteration: String,
    /// Le mot français qui en descend.
    #[prop(into)]
    francais: String,
    /// Ce que le passage a coûté.
    children: Children,
) -> impl IntoView {
    view! {
        <li class="border-t border-filet py-10 first:border-t-0 first:pt-0">
            <div class="grid gap-8 md:grid-cols-[minmax(0,0.8fr)_minmax(0,1.2fr)] md:gap-12">
                // ── La descente ──────────────────────────────────────────
                <div>
                    <p class="m-0 flex flex-wrap items-baseline gap-x-3 gap-y-1">
                        // `dir` sur un span en ligne et non sur le bloc : sur
                        // un bloc, il alignerait aussi tout le contenu à
                        // droite, et la translittération partirait se coller
                        // au bord. C'est le défaut déjà corrigé sur les titres
                        // de livre.
                        <span dir="rtl" lang="he" class="font-hebreu text-2xl text-accent">
                            {hebreu}
                        </span>
                        <i class="text-accent">{translitteration}</i>
                    </p>
                    <p class="m-0 mt-2 text-sm text-encre-douce">{sens}</p>

                    // Le trait descend d'un état à l'autre, et il pâlit avec
                    // eux. C'est la seule chose qui dise « ceci vient de
                    // cela » sans l'écrire.
                    <span aria-hidden="true" class="my-3 block h-6 w-px bg-or/30"></span>

                    <p class="m-0 flex flex-wrap items-baseline gap-x-3 gap-y-1">
                        // Literata porte le grec polytonique : aucune fonte de
                        // plus à charger, et le mot reste dans la lettre du
                        // corps plutôt que de retomber sur une fonte système.
                        <span lang="grc" class="text-xl">{grec}</span>
                        <i class="text-encre-douce">{grec_translitteration}</i>
                    </p>

                    <span aria-hidden="true" class="my-3 block h-6 w-px bg-or/15"></span>

                    <p class="m-0 text-encre-douce">"«\u{202F}"{francais}"\u{202F}»"</p>
                </div>

                // ── Ce que ça a coûté ────────────────────────────────────
                <div class="text-pretty">{children()}</div>
            </div>
        </li>
    }
}
