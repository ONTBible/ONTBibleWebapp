use leptos::prelude::*;

/// Le titre d'une page — la page de garde d'un chapitre.
///
/// Un `<h1>` nu au-dessus d'un paragraphe donne un article de blog. Un livre
/// annonce autrement : une ligne de rappel en capitales espacées, le titre,
/// puis un filet qui referme le bloc et ouvre le texte.
///
/// Le rappel n'est pas décoratif — il dit *de quoi* on parle avant de dire
/// *quoi*. Sur une page atteinte par un lien partagé, c'est souvent la seule
/// chose qui situe le lecteur.
#[component]
pub fn TitreDePage(
    /// La ligne de rappel, au-dessus du titre.
    #[prop(into)]
    rappel: String,
    #[prop(into)] titre: String,
) -> impl IntoView {
    view! {
        <div class="mb-12 text-center">
            <p class="mb-4 text-sm uppercase tracking-capitales text-or-profond">{rappel}</p>
            <h1 class="mb-0 font-titre">{titre}</h1>
            <div
                aria-hidden="true"
                class="mx-auto mt-8 flex w-32 items-center gap-3 text-or-profond \
                       before:h-px before:flex-1 before:bg-current/40 before:content-[''] \
                       after:h-px after:flex-1 after:bg-current/40 after:content-['']"
            >
                <span class="signe-montagne w-5"></span>
            </div>
        </div>
    }
}
