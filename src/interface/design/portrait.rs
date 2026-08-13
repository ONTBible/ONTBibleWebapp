use leptos::prelude::*;

/// Un portrait détouré, sans cadre.
///
/// ## Pourquoi pas de cadre
///
/// La version précédente l'inscrivait dans une arche bordée d'un filet d'or.
/// L'intention était de borner la clarté de la chemise ; l'effet était de
/// **l'enfermer dans une boîte** posée sur la page, et le rectangle se voyait
/// plus que le sujet.
///
/// Ce qui remplace le cadre est une **lueur** — un halo d'aubergine derrière
/// les épaules, flou et sans contour. Il fait le même travail (donner un fond
/// au sujet pour qu'il ne flotte pas dans le vide) sans rien dessiner : on ne
/// voit pas d'où vient la lumière, seulement qu'il y en a une.
///
/// Le bas se dissout dans la nuit. Le détourage laisse le vêtement
/// s'effilocher ; autant en faire une intention plutôt qu'une coupure nette.
///
/// ## La taille vient du parent
///
/// Ce composant remplit la largeur qu'on lui donne. C'est ce qui permet au même
/// portrait d'être une vignette dans une colonne et une pleine figure dans une
/// marge, sans multiplier les variantes.
#[component]
pub fn Portrait(
    /// Les deux tailles disponibles, laissées au navigateur : il prend celle
    /// qui convient à sa densité, sans qu'on ait à la deviner.
    #[prop(into)]
    source: String,
    #[prop(into)] source_large: String,
    #[prop(into)] texte: String,
    /// Ce que le navigateur doit réserver comme largeur d'affichage.
    #[prop(into, default = "22rem".to_string())]
    largeur_rendue: String,
) -> impl IntoView {
    view! {
        <figure class="relative isolate m-0">
            // La lueur, pas un cadre. Elle déborde le sujet de tous côtés et
            // s'éteint avant le bord : c'est ce flou qui la rend crédible comme
            // lumière plutôt que comme forme.
            <div
                aria-hidden="true"
                class="pointer-events-none absolute inset-[-12%] -z-10 rounded-[50%] bg-[radial-gradient(ellipse_at_50%_42%,var(--color-surface-haute),transparent_68%)] blur-2xl"
            ></div>
            <img
                src=source.clone()
                srcset=format!("{source} 640w, {source_large} 1024w")
                sizes=largeur_rendue
                alt=texte
                width="640"
                height="892"
                class="fondu-bas block h-auto w-full"
            />
        </figure>
    }
}
