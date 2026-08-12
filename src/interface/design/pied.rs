use leptos::prelude::*;

/// Le pied de page.
///
/// Il porte quatre choses, et rien de plus : la marque, les deux pages que
/// l'App Store réclamera, le code, et la mention de droit d'auteur. Pas de
/// plan du site — il a cinq pages, un plan serait une redite.
///
/// L'année vient de la compilation (`build.rs`) : le pied est rendu par le
/// serveur **et** par le navigateur, donc il ne peut pas lire l'horloge, qui
/// n'existe que d'un côté.
#[component]
pub fn PiedDePage() -> impl IntoView {
    view! {
        <footer class="border-t border-filet px-6 py-12 text-sm text-encre-douce">
            <div class="mx-auto flex max-w-mesure-large flex-col items-center gap-8">
                <div class="flex items-center gap-3">
                    <span class="signe-montagne w-8 text-accent" aria-hidden="true"></span>
                    <span class="uppercase tracking-capitales">"La Bible ONT"</span>
                </div>

                <nav
                    aria-label="Mentions et code source"
                    class="flex flex-wrap justify-center gap-x-6 gap-y-2"
                >
                    <a href="/fr/confidentialite" class=LIEN>"Confidentialité"</a>
                    <a href="/fr/conditions" class=LIEN>"Conditions"</a>
                    <a href="https://github.com/ONTBible" rel="noopener" class=LIEN>
                        "GitHub"
                    </a>
                </nav>

                <div class="flex flex-col items-center gap-2 text-center text-xs">
                    <p>"© " {env!("ANNEE_DE_COMPILATION")} " Gloire Bikouta"</p>
                    <p>
                        "Le corpus et le code sont "
                        <a
                            href="https://github.com/ONTBible/ONTBibleTranslation"
                            rel="noopener"
                            class="underline"
                        >
                            "publics"
                        </a>
                        ". La traduction est en cours."
                    </p>
                </div>
            </div>
        </footer>
    }
}

const LIEN: &str = "no-underline transition-colors hover:text-encre hover:underline \
                    hover:underline-offset-4";
