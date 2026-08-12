use leptos::prelude::*;

/// L'ouverture du site — on entre dans un lieu.
///
/// C'est le parti pris qu'il a choisi entre trois : ni une affiche, ni une
/// notice, mais un **espace**. La lumière tombe du haut comme dans une nef,
/// la montagne ferme le bas comme un horizon, et le texte est dedans.
///
/// Deux massifs et non un seul : le premier en aubergine, franc, donne la
/// ligne d'horizon ; le second, plus large et à peine visible en or, suggère un
/// relief plus lointain derrière. C'est ce décalage qui fait la profondeur —
/// un massif isolé aurait l'air d'un autocollant.
///
/// Elle occupe l'écran entier : sur une ouverture, la barre de navigation fait
/// partie du lieu plutôt que de le surmonter.
#[component]
pub fn Hero(children: Children) -> impl IntoView {
    view! {
        <section class="voute relative isolate flex min-h-dvh flex-col items-center justify-center overflow-hidden px-6 py-24 text-center">
            <span
                aria-hidden="true"
                class="massif pointer-events-none absolute -bottom-[6%] left-1/2 -z-10 w-[165%] -translate-x-1/2 text-aubergine opacity-80"
            ></span>
            <span
                aria-hidden="true"
                class="massif pointer-events-none absolute -bottom-[13%] left-1/2 -z-10 w-[200%] -translate-x-1/2 text-or opacity-[0.055]"
            ></span>

            <div class="flex max-w-4xl flex-col items-center gap-8">{children()}</div>
        </section>
    }
}
