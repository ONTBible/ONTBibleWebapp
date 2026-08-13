use leptos::prelude::*;

/// La légende des trois niveaux du texte.
///
/// Elle appartient au design system et non à une page, parce que **c'est le
/// design system qu'elle explique** : chaque échantillon y est composé avec la
/// règle réelle. Le jour où l'or change de valeur, cette légende change avec
/// lui, sans qu'on ait à y penser.
///
/// Les deux échantillons colorés sont volontairement **inertes** : dans une
/// légende, un intraduisible cliquable mènerait à la fiche d'un mot qui n'est
/// pas un mot du corpus.
#[component]
pub fn LegendeNiveaux() -> impl IntoView {
    view! {
        <dl class="mt-10">
            <Niveau>
                <Terme slot>{"Le corps"}</Terme>
                "Ce que l'hébreu dit directement. Rien d'ajouté."
            </Niveau>

            <Niveau>
                <Terme slot>
                    <span class="font-semibold text-accent">"L'intraduisible"</span>
                </Terme>
                "Un mot qu'aucun mot français ne rend sans perte. Il reste en hébreu, en or — "
                "et il mène à sa fiche. L'or promet une explication, et il la tient."
            </Niveau>

            <Niveau>
                <Terme slot>
                    <span class="font-semibold text-important">"Le terme important"</span>
                </Terme>
                "Un mot qui porte le poids de la phrase, mais qui se traduit. Il est marqué, "
                "et il ne promet rien : il ne mène nulle part."
            </Niveau>

            <Niveau>
                <Terme slot>
                    <span class="italic text-encre-douce">"[La glose]"</span>
                </Terme>
                "Ce que le champ sémantique hébreu portait pour son lecteur, et que le "
                "français perd. Elle explicite l'implicite ; elle n'invente pas."
            </Niveau>

            <Niveau>
                <Terme slot>
                    <span class="text-encre-douce">
                        "("<i>"la source"</i>" / "
                        <span dir="rtl" lang="he" class="font-hebreu not-italic">"הָעִבְרִית"</span>
                        ")"
                    </span>
                </Terme>
                "La translittération et l'hébreu, toujours les deux. Pour qu'on puisse "
                "vérifier au lieu de croire."
            </Niveau>
        </dl>
    }
}

/// Une entrée de la légende — un terme, sa définition, un filet entre les deux.
#[component]
fn Niveau(terme: Terme, children: Children) -> impl IntoView {
    view! {
        <div class="border-t border-filet py-4 first:border-t-0">
            <dt class="text-sm">{(terme.children)()}</dt>
            <dd class="mt-1 ml-0 text-sm text-encre-douce">{children()}</dd>
        </div>
    }
}

/// Le terme d'une entrée, composé avec la règle qu'il illustre.
#[slot]
struct Terme {
    children: ChildrenFn,
}
