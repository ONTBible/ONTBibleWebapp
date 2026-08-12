use leptos::prelude::*;

use crate::domaine::texte::Verset as VersetDomaine;
use crate::interface::design::Verset;

/// Le même verset, deux fois.
///
/// C'est la pièce qui porte tout le site. Expliquer l'ontologie fonctionnelle
/// demande une page ; la **montrer** demande six lignes — la traduction
/// classique au-dessus, courte et close, la restitution en dessous avec ce
/// qu'elle rend visible. L'écart entre les deux se voit avant d'être compris,
/// et c'est ce qui donne envie de lire l'explication.
///
/// La traduction de référence est **Louis Segond 1910**, dans le domaine
/// public. Citer une traduction moderne serait une contrefaçon, et personne
/// n'a besoin de ça pour faire une démonstration.
#[component]
pub fn Comparaison(
    /// Le renvoi commun aux deux — « Bereshit 1:1 ».
    #[prop(into)]
    renvoi: String,
    /// Le texte de référence.
    #[prop(into)]
    classique: String,
    /// La source de ce texte, telle qu'on doit la citer.
    #[prop(into)]
    source: String,
    /// La restitution, avec ses trois niveaux.
    ont: VersetDomaine,
) -> impl IntoView {
    view! {
        <div class="space-y-4">
            <p class="text-sm uppercase tracking-capitales text-encre-douce">{renvoi}</p>

            <figure class="rounded-carte border border-filet bg-surface/60 px-6 py-6">
                <blockquote class="m-0 text-encre-douce">{classique}</blockquote>
                <figcaption class="mt-3 text-sm text-encre-douce/80">{source}</figcaption>
            </figure>

            // Le sens de lecture est marqué par un signe, pas par une flèche :
            // il ne s'agit pas d'un « avant / après » ni d'une correction, mais
            // de deux lectures du même hébreu.
            <div class="flex justify-center py-2 text-accent" aria-hidden="true">
                <span class="signe-montagne w-7"></span>
            </div>

            <figure class="rounded-carte border border-or/25 bg-surface px-6 py-7">
                <Verset verset=ont />
                <figcaption class="mt-3 text-sm uppercase tracking-capitales text-accent">
                    "La Bible ONT"
                </figcaption>
            </figure>
        </div>
    }
}
