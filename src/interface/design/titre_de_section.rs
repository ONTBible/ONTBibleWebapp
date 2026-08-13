use leptos::prelude::*;

/// Le titre d'une section, numéroté.
///
/// Le chiffre romain est ce qui fait basculer la page d'« un site » vers « un
/// livre » : il annonce que ce qui suit a un ordre, et que cet ordre a été
/// pensé. C'est un usage de traité ancien.
///
/// Ce qui le rend moderne, c'est l'exécution — pas d'enluminure, pas de
/// cartouche : un chiffre en capitales espacées, un filet court, et beaucoup
/// d'air. L'ancien donne la structure, le moderne donne la retenue.
///
/// Le chiffre est `aria-hidden` : à l'oreille, « I » avant un titre n'apporte
/// rien et casse le fil de la lecture.
#[component]
pub fn TitreDeSection(
    /// Le chiffre romain — « I », « II », « III »…
    #[prop(into)]
    numero: String,
    #[prop(into)] titre: String,
) -> impl IntoView {
    view! {
        <div class="mb-8">
            <div
                aria-hidden="true"
                class="mb-5 flex items-center gap-4 text-sm uppercase tracking-capitales text-accent"
            >
                <span>{numero}</span>
                <span class="h-px w-10 bg-current opacity-40"></span>
            </div>
            <h2 class="mt-0">{titre}</h2>
        </div>
    }
}
