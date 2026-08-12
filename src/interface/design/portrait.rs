use leptos::prelude::*;

/// Un portrait détouré sur transparence.
///
/// Le détourage laisse le bas du vêtement se dissoudre dans le fond ; le
/// dégradé de `fondu-bas` rend cette dissolution volontaire, au lieu de la
/// laisser paraître ratée.
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
        <figure class="mb-10">
            <img
                src=source.clone()
                srcset=format!("{source} 640w, {source_large} 1024w")
                sizes="(max-width: 34rem) 60vw, 18rem"
                alt=texte
                width="640"
                height="892"
                class="fondu-bas h-auto w-[min(18rem,60%)]"
            />
        </figure>
    }
}
