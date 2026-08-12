use leptos::prelude::*;

/// Un portrait détouré sur transparence.
///
/// Rond et de la taille d'une vignette d'auteur, pas d'une illustration : la
/// première version l'affichait sur dix-huit rem, ce qui laissait un grand
/// vide sous lui et donnait à la page l'air d'attendre quelque chose.
///
/// Le détourage laisse le bas du vêtement se dissoudre ; `fondu-bas` rend
/// cette dissolution volontaire au lieu de la laisser paraître ratée.
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
        <figure class="mb-10 flex justify-center">
            <img
                src=source.clone()
                srcset=format!("{source} 640w, {source_large} 1024w")
                sizes="14rem"
                alt=texte
                width="640"
                height="892"
                class="fondu-bas h-auto w-56"
            />
        </figure>
    }
}
