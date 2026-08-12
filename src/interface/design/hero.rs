use leptos::prelude::*;

/// L'ouverture du site.
///
/// Elle ne présente pas le projet, elle l'**affirme**. La montagne en grand,
/// une phrase, deux chemins. C'est une page de titre de livre : on n'y explique
/// rien, on pose le nom et on ouvre.
///
/// Elle occupe volontairement une hauteur d'écran sans la remplir. Le vide
/// autour du signe fait partie de ce qu'on montre — un livre ancien ne remplit
/// pas sa page de garde.
#[component]
pub fn Hero(children: Children) -> impl IntoView {
    view! {
        <section class="voile-aubergine relative isolate overflow-hidden">
            // La montagne, très grande, très pâle, débordant par le haut. Elle
            // n'est pas là pour être vue mais pour que la page ait un centre —
            // rognée par le cadre, elle se lit comme un massif dont on ne voit
            // que la crête.
            <span
                aria-hidden="true"
                class="signe-montagne pointer-events-none absolute -top-[14%] left-1/2 -z-10 w-[150%] -translate-x-1/2 text-or opacity-[0.06]"
            ></span>

            <div class="mx-auto flex max-w-mesure flex-col items-center gap-8 px-6 py-32 text-center">
                {children()}
            </div>
        </section>
    }
}
