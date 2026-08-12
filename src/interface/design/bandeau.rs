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
    // Pas de montagne en filigrane ici, et c'est une correction : posée à 130 %
    // de large et rognée par le cadre, elle ne se lisait plus comme une
    // montagne mais comme des polygones au hasard. Un signe qu'on ne reconnaît
    // pas n'est pas un signe discret, c'est du bruit.
    //
    // Le dégradé suffit à donner la profondeur ; les filets d'or aux deux
    // bords font le reste — ils posent le bandeau dans la page au lieu de le
    // laisser flotter.
    view! {
        <div class="voile-aubergine relative isolate overflow-hidden border-y border-or/25 text-or">
            <div class="mx-auto max-w-mesure px-6 py-24 text-center">{children()}</div>
        </div>
    }
}
