use leptos::prelude::*;

/// Un bandeau d'aubergine, pleine largeur.
///
/// C'est là que la direction « un peu ancienne » se joue vraiment. Un aplat
/// d'aubergine serait sobre ; ce bandeau est **profond** — une lueur dorée
/// haute, l'aubergine qui s'assombrit vers le bas, et la montagne en filigrane
/// derrière le texte. La lumière semble venir d'au-dessus, comme dans une nef.
///
/// Il s'affranchit de la mesure de la page pour toucher les deux bords, puis
/// la rétablit à l'intérieur : le fond respire, le texte reste lisible.
///
/// Les couleurs ne suivent pas le thème. Un bandeau qui s'éclaircirait en
/// thème clair perdrait exactement ce qu'il est venu chercher.
#[component]
pub fn Bandeau(children: Children) -> impl IntoView {
    view! {
        <div class="voile-aubergine relative isolate overflow-hidden text-or">
            // La montagne, très grande et très pâle. Elle n'est pas là pour
            // être vue — elle est là pour qu'on sente qu'il y a quelque chose.
            <span
                aria-hidden="true"
                class="filigrane-montagne pointer-events-none absolute -bottom-[12%] left-1/2 \
                       -z-10 w-[130%] -translate-x-1/2 opacity-8"
            ></span>

            <div class="mx-auto max-w-mesure px-6 py-24 text-center">{children()}</div>
        </div>
    }
}
