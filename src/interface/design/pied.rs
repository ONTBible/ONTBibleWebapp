use leptos::prelude::*;
use leptos_router::components::A;

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
pub fn PiedDePage(
    /// Une classe de plus sur l'élément racine — l'habillage, et rien d'autre.
    ///
    /// **Elle ne s'enveloppe pas.** Un `<div>` posé autour de ce composant tue
    /// l'hydratation : Leptos compte des marqueurs, et une enveloppe les fait
    /// tomber ailleurs. Mesuré le 30 septembre 2026, deux fois de suite et à
    /// deux endroits — `navigation_de_la_liseuse.rs:312`, puis `bloc.rs:106`.
    ///
    /// ==Un habillage se pose sur l'élément, jamais autour de lui.==
    #[prop(optional, into)]
    classe: Option<&'static str>,
) -> impl IntoView {
    // **Sous la webapp, il se range derrière les barres.**
    //
    // Elles sont en `fixed` : la latérale occupe seize rem et demie à gauche
    // au-delà de `lg`, les onglets flottent en bas en dessous. Le contenu s'en
    // écarte lui-même — `pb-24 lg:ps-[16.5rem] lg:pb-0` —, mais le pied est
    // rendu par `App`, **hors** de ce conteneur : sans le même dégagement, il
    // passe sous la barre latérale sur un grand écran et sous les onglets sur un
    // téléphone.
    //
    // ==Ce qu'un `fixed` libère, chacun doit se l'ajouter : il ne pousse
    // personne.== Et c'est pourquoi la valeur est écrite deux fois — ici et sur
    // le conteneur de `PageDeLecture`. Les deux décrivent la même géométrie,
    // qui est celle des barres ; le jour où elle bouge, les deux bougent.
    let sous_l_app = crate::interface::arbre::sous_l_arbre(crate::domaine::lecture::Arbre::Webapp);

    view! {
        <footer class=move || {
            format!(
                "{} {} border-t border-filet px-6 py-12 text-sm text-encre-douce",
                classe.unwrap_or_default(),
                if sous_l_app.get() { "pb-24 lg:ps-[16.5rem] lg:pb-12" } else { "" },
            )
        }>
            <div class="mx-auto flex max-w-large flex-col items-center gap-8">
                // **La marque mène à l'accueil. Corrigé le 2 octobre 2026**,
                // et l'auteur l'a dit ainsi : *« un manquement que j'ai
                // remarqué depuis le début du site mais j'ai oublié de te le
                // signaler »*.
                //
                // Elle était un `<div>` inerte. C'est la convention la plus
                // ancienne du web — la marque d'un pied ou d'un en-tête ramène
                // chez soi —, et un lecteur l'essaie **avant** de chercher un
                // lien nommé. Qu'elle ne réponde pas ne se lit pas comme une
                // absence : ça se lit comme une page qui ne marche pas.
                //
                // ==Un signe dont l'usage est acquis n'a pas besoin d'être
                // annoncé, mais il a besoin de répondre.== C'est la règle du
                // §5 sur les liens de prose, prise par l'autre bout : là, le
                // trait manquait à un lien ; ici, le lien manque à un signe
                // qu'on prend pour un lien.
                //
                // Le logomark et le nom sont **dans la même ancre** : ce sont
                // deux moitiés du même objet, et deux liens côte à côte vers la
                // même adresse donnent deux arrêts au clavier pour un seul
                // geste.
                //
                // `aria-label` plutôt que le texte seul : il porte « La Bible
                // ONT » en capitales espacées, qu'un lecteur d'écran épelle
                // lettre par lettre, et il ne dit pas où l'on va.
                <A
                    href="/fr"
                    attr:aria-label="La Bible ONT — l'accueil"
                    attr:class="flex items-center gap-3 no-underline transition-opacity hover:opacity-80"
                >
                    <span class="signe-montagne w-8 text-accent" aria-hidden="true"></span>
                    <span aria-hidden="true" class="uppercase tracking-capitales">
                        "La Bible ONT"
                    </span>
                </A>

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
                    <a href=crate::domaine::chemins::bible(crate::interface::arbre::arbre_maintenant()) class=LIEN>"Webapp"</a>
                    <a href=crate::domaine::chemins::lexique(crate::interface::arbre::arbre_maintenant()) class=LIEN>"Lexique"</a>
                    <a href=crate::domaine::chemins::rechercher(crate::interface::arbre::arbre_maintenant()) class=LIEN>"Rechercher"</a>
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
                    <a href=crate::domaine::chemins::compte(crate::interface::arbre::arbre_maintenant()) class=LIEN>"Votre compte"</a>
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
