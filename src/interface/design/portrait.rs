use leptos::prelude::*;

/// Un portrait, inscrit dans une arche.
///
/// ## Pourquoi une arche et non une vignette flottante
///
/// Détouré sur transparence et posé à même la page, le sujet a l'air d'un
/// autocollant — et sa chemise blanche fait une masse lumineuse au milieu
/// d'une nuit. L'arche règle les deux : elle borne la clarté, et elle donne au
/// portrait le statut qu'il a dans un livre ancien, celui d'une niche.
///
/// C'est aussi là que se joue « ancien mais moderne » : la forme est celle
/// d'un portrait gravé, le trait est un filet d'or d'un pixel et le fond une
/// surface unie. Aucune ornementation, une seule idée de forme.
///
/// Le bas se dissout dans le fond de l'arche plutôt que d'être coupé net : le
/// détourage laisse le vêtement s'effilocher, autant en faire une intention.
#[component]
pub fn Portrait(
    /// Les deux tailles disponibles, laissées au navigateur : il prend celle
    /// qui convient à sa densité, sans qu'on ait à la deviner.
    #[prop(into)]
    source: String,
    #[prop(into)] source_large: String,
    #[prop(into)] texte: String,
) -> impl IntoView {
    view! {
        <figure class="mx-auto w-56 max-w-full">
            // Le cadre impose ses proportions et l'image le remplit en se
            // recadrant : la photo a de la marge au-dessus de la tête, et
            // affichée telle quelle dans une niche, le visage y paraît petit
            // et lointain. Le cadrage est calé un peu au-dessus du centre,
            // là où se trouvent les yeux.
            <div class="relative isolate aspect-[4/5] overflow-hidden rounded-t-full rounded-b-carte border border-or/25 bg-surface">
                // Une lueur au sommet de l'arche — la même lumière que celle
                // des bandeaux, pour que le portrait appartienne à la page.
                <div
                    aria-hidden="true"
                    class="pointer-events-none absolute inset-0 -z-10 bg-[radial-gradient(ellipse_80%_50%_at_50%_0%,color-mix(in_srgb,var(--color-or)_10%,transparent),transparent_70%)]"
                ></div>
                <img
                    src=source.clone()
                    srcset=format!("{source} 640w, {source_large} 1024w")
                    sizes="14rem"
                    alt=texte
                    width="640"
                    height="892"
                    class="fondu-bas block h-full w-full object-cover object-[center_22%]"
                />
            </div>
        </figure>
    }
}
