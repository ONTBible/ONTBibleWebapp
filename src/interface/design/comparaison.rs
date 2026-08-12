use leptos::prelude::*;

use crate::domaine::texte::Verset as VersetDomaine;
use crate::interface::design::Verset;

/// Le même verset, deux fois.
///
/// C'est la pièce qui porte tout le site. Expliquer l'ontologie fonctionnelle
/// demande une page ; la **montrer** demande six lignes — la traduction
/// classique d'un côté, courte et close, la restitution de l'autre avec ce
/// qu'elle rend visible. L'écart se voit avant d'être compris, et c'est ce qui
/// donne envie de lire l'explication.
///
/// ## Deux colonnes sur grand écran, une seule sur téléphone
///
/// Côte à côte, l'écart se lit d'un coup d'œil : c'est la disposition qui fait
/// l'argument. Les deux colonnes ne sont **pas** de largeur égale — la
/// restitution est trois fois plus longue que la traduction de référence, et à
/// largeurs égales elle s'étire en ruban tandis que l'autre flotte dans le
/// vide. Le rapport 0,62 / 1,38 les fait finir à peu près à la même hauteur.
///
/// Sur un téléphone, deux colonnes de texte donneraient deux colonnes de trois
/// mots. En dessous du seuil, elles s'empilent — et le signe qui les sépare
/// passe d'un trait vertical à un trait horizontal, sans qu'on ait à écrire
/// deux fois le composant.
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
            <p class="mb-5 text-sm uppercase tracking-capitales text-encre-douce">{renvoi}</p>

            <div class="grid items-stretch gap-6 md:grid-cols-[minmax(0,0.62fr)_auto_minmax(0,1.38fr)] md:gap-8">
                <figure class="m-0 flex flex-col rounded-carte border border-filet bg-surface/50 px-6 py-6">
                    <blockquote class="m-0 grow text-encre-douce">{classique}</blockquote>
                    <figcaption class="mt-4 text-sm text-encre-douce/80">{source}</figcaption>
                </figure>

                // Le sens de lecture est marqué par un signe, pas par une
                // flèche : il ne s'agit ni d'un « avant / après » ni d'une
                // correction, mais de deux lectures du même hébreu.
                <div
                    aria-hidden="true"
                    class="flex items-center justify-center gap-4 text-accent md:flex-col"
                >
                    <span class="h-px grow bg-current opacity-25 md:h-auto md:w-px"></span>
                    <span class="signe-montagne w-7 shrink-0"></span>
                    <span class="h-px grow bg-current opacity-25 md:h-auto md:w-px"></span>
                </div>

                <figure class="m-0 flex flex-col rounded-carte border border-or/25 bg-surface px-6 py-7">
                    <div class="grow">
                        <Verset verset=ont />
                    </div>
                    <figcaption class="mt-4 text-sm uppercase tracking-capitales text-accent">
                        "La Bible ONT"
                    </figcaption>
                </figure>
            </div>
        </div>
    }
}
