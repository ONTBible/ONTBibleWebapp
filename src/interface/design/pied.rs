use leptos::prelude::*;

/// Le pied de page.
///
/// Il porte la marque, l'entrée du corpus, les deux pages que l'App Store
/// réclamera, le code, et la mention de droit d'auteur.
///
/// L'entrée du corpus y est **depuis la liseuse** : tant que le site avait cinq
/// pages, un plan en pied aurait été une redite de l'en-tête. Avec un corpus,
/// quelqu'un qui arrive au bas d'une page longue doit pouvoir y entrer sans
/// remonter.
///
/// L'année vient de la compilation (`build.rs`) : le pied est rendu par le
/// serveur **et** par le navigateur, donc il ne peut pas lire l'horloge, qui
/// n'existe que d'un côté.
#[component]
pub fn PiedDePage() -> impl IntoView {
    view! {
        <footer class="border-t border-filet px-6 py-12 text-sm text-encre-douce">
            <div class="mx-auto flex max-w-large flex-col items-center gap-8">
                <div class="flex items-center gap-3">
                    <span class="signe-montagne w-8 text-accent" aria-hidden="true"></span>
                    <span class="uppercase tracking-capitales">"La Bible ONT"</span>
                </div>

                // Le corpus d'abord, les mentions ensuite. Deux rangs et non
                // un seul : « Lire » et « Confidentialité » ne sont pas de même
                // nature, et les aligner ferait de la liseuse une mention
                // légale de plus.
                //
                // « L'app » est de ce rang-ci et pas du second : c'est une façon
                // de lire le corpus, pas une mention. Le pied ne la portait pas
                // du tout — or c'est le seul chemin d'installation du site, et
                // un lecteur arrivé en bas de page l'y cherche.
                <nav
                    aria-label="Le corpus et l'application"
                    class="flex flex-wrap justify-center gap-x-6 gap-y-2 uppercase tracking-capitales text-encre"
                >
                    <a href="/fr/lire" class=LIEN>"Lire"</a>
                    <a href="/fr/lexique" class=LIEN>"Lexique"</a>
                    <a href="/fr/l-app" class=LIEN>"L'app"</a>
                </nav>

                <nav
                    aria-label="Mentions et code source"
                    class="flex flex-wrap justify-center gap-x-6 gap-y-2"
                >
                    // Le compte est dans le second rang, avec les mentions, et
                    // pas dans le premier avec Lire et Lexique : il ne sert pas
                    // à lire. C'est un réglage, pas une façon d'entrer dans le
                    // corpus — le mettre en tête laisserait croire qu'il faut
                    // s'inscrire pour lire.
                    <a href="/fr/compte" class=LIEN>"Votre compte"</a>
                    <a href="/fr/assistance" class=LIEN>"Assistance"</a>
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
