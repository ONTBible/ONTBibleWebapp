use leptos::prelude::*;

use crate::domaine::texte::Verset as VersetDomaine;
use crate::interface::design::Verset;

/// Le même verset, deux fois — un plan en retrait, un plan éclairé.
///
/// C'est la pièce qui porte tout le site. Expliquer l'ontologie fonctionnelle
/// demande une page ; la **montrer** demande six lignes.
///
/// La disposition fait l'argument : la traduction de référence est sombre et
/// discrète, la restitution est en avant, sur une surface plus claire, cernée
/// d'or et halée. On voit lequel des deux porte ce que l'hébreu disait avant
/// d'avoir lu un mot.
///
/// Les colonnes ne sont **pas** de largeur égale — la restitution est trois
/// fois plus longue que la référence, et à largeurs égales elle s'étire en
/// ruban pendant que l'autre flotte dans le vide.
///
/// Sur un téléphone, deux colonnes donneraient deux colonnes de trois mots :
/// elles s'empilent, la référence au-dessus.
///
/// La traduction de référence est **Louis Segond 1910**, dans le domaine
/// public. Citer une traduction moderne serait une contrefaçon, et une
/// traduction ancienne rend d'ailleurs l'écart plus parlant.
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
        <div>
            <p class="mb-6 text-sm uppercase tracking-capitales text-encre-douce">{renvoi}</p>

            <div class="grid items-center gap-8 md:grid-cols-[minmax(0,0.7fr)_minmax(0,1.3fr)]">
                <figure class="m-0 rounded-carte border border-filet bg-surface/40 px-6 py-6">
                    <blockquote class="m-0 text-encre-douce">{classique}</blockquote>
                    <figcaption class="mt-4 text-sm text-encre-douce/70">{source}</figcaption>
                </figure>

                <figure class="halo m-0 rounded-carte border border-or/25 bg-surface-haute px-7 py-8">
                    <Verset verset=ont />
                    <figcaption class="mt-5 text-sm uppercase tracking-capitales text-accent">
                        "La Bible ONT"
                    </figcaption>
                </figure>
            </div>
        </div>
    }
}
