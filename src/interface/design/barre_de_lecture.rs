use leptos::prelude::*;
use leptos_router::components::A;

/// La barre du haut de l'écran de lecture — la pastille de renvoi et « aA ».
///
/// ## Deux capsules qui flottent, et non un bandeau
///
/// C'est le dessin d'iOS 26, et il se lit dans le Swift plutôt que dans une
/// capture : `ChapterView` pose `.ontVerre(dans: Capsule())` **sur la pastille
/// elle-même**, pas sur une barre qui la contiendrait. Le verre appartient à
/// l'objet, le fond reste le texte. Un bandeau plein d'une largeur d'écran
/// serait le dessin d'avant — et il couperait la page en deux là où l'app
/// laisse le chapitre passer derrière.
///
/// C'est aussi ce que fait déjà la barre d'onglets en bas : même matière, même
/// façon de flotter.
///
/// ## La pastille est une porte, pas une étiquette
///
/// Le commentaire de l'app le dit : *« Elle dit où l'on est **et** sert de
/// porte : sans elle, aller de Bereshit 1 à Bereshit 18 demande de remonter à
/// la table, replier, déplier, redescendre. »*
///
/// L'app ouvre un sélecteur modal ; ici c'est un **lien** vers la liste des
/// unités du livre. La destination est la même, le retour est gratuit — c'est
/// l'historique du navigateur —, et une page atteignable par une adresse vaut
/// mieux qu'une feuille qu'on ne peut pas partager.
///
/// ## Elle remplace le fil d'Ariane, elle ne s'y ajoute pas
///
/// Les deux disent la même chose — « tu es dans Bereshit, au 3 » — et les
/// empiler donnerait deux appareils de navigation pour un seul écran. Le fil
/// reste sur les écrans de liste, où il n'y a pas de pastille.
#[component]
pub fn BarreDeLecture(
    /// Où mène la pastille : la liste des unités du livre.
    #[prop(into)]
    chemin: String,
    /// Ce qu'elle porte — « Bereshit · 3 ».
    ///
    /// **Un `Signal`**, pour la même raison que le titre de `PageDeLecture` :
    /// le nom d'une unité dépend du registre de lecture, que le lecteur
    /// bascule sans recharger.
    #[prop(into)]
    pastille: Signal<String>,
    /// Ce qui se pose à droite — le bouton « aA ».
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        // `pointer-events-none` sur l'enveloppe et `auto` sur les capsules :
        // sans ça, la bande transparente entre les deux prendrait les clics
        // sur toute la largeur de la colonne, et l'on ne pourrait plus
        // sélectionner le premier verset du chapitre.
        <div class="pointer-events-none sticky top-0 z-30 -mx-1 mb-8 flex items-center justify-between gap-3 py-2">
            <A
                href=chemin
                attr:class="verre pointer-events-auto flex max-w-[70%] items-center gap-1.5 rounded-full px-3 py-1.5 text-sm font-semibold text-encre no-underline transition-transform duration-150 ease-out active:scale-95 motion-reduce:transition-none"
                attr:aria-label=move || {
                    format!("Aller à un autre passage — actuellement {}", pastille.get())
                }
            >
                // `truncate` plutôt qu'un retour à la ligne : une pastille est
                // une adresse, et une adresse sur deux lignes cesse d'être une
                // capsule. L'app tient la même règle par `lineLimit(1)`.
                <span class="truncate">{move || pastille.get()}</span>
                <svg
                    aria-hidden="true"
                    viewBox="0 0 12 12"
                    class="size-2.5 shrink-0"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2.2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="M2.5 4.5 6 8l3.5-3.5" />
                </svg>
            </A>

            {children.map(|enfants| {
                view! { <div class="pointer-events-auto flex items-center">{enfants()}</div> }
            })}
        </div>
    }
}

/// Le bouton de recherche, en haut à droite d'un écran de liste.
///
/// ## Sa place vient de l'app, pas d'un goût
///
/// `BibleTab` le pose en `ONTPlacement.principale` :
///
/// ```swift
/// ToolbarItem(placement: ONTPlacement.principale) {
///     Button("Rechercher", systemImage: "magnifyingglass") { searching = true }
/// }
/// ```
///
/// **Et sur la Bible seulement.** Le Lexique n'en a pas : il a son rail de
/// lettres et ses quatre segments, qui font le même travail sur un corpus
/// fermé de quelques centaines d'entrées. Chercher dans une liste qu'on peut
/// parcourir d'un pouce n'est pas le même geste que chercher dans soixante-dix
/// livres.
///
/// ## Un lien, pas une feuille
///
/// L'app ouvre une `.ontFeuille`. Ici la recherche est une **page**, avec son
/// adresse — `/fr/rechercher?q=…` se partage, se met en signet, et le retour
/// du navigateur la referme. Même arbitrage que la feuille de prononciation,
/// et pour la même raison : ce qui ne demande rien n'a pas à être modal.
///
/// ## Il porte le même verre que la pastille
///
/// Une capsule qui flotte sur le texte, et non un bouton posé dans un bandeau.
/// C'est la matière de toute la chrome de la liseuse depuis le portage.
#[component]
pub fn BoutonDeRecherche() -> impl IntoView {
    view! {
        <A
            href="/fr/rechercher"
            attr:aria-label="Rechercher dans le corpus"
            attr:class="verre pointer-events-auto flex size-9 items-center justify-center rounded-full text-encre-douce no-underline transition-transform duration-150 ease-out hover:text-encre active:scale-95 motion-reduce:transition-none"
        >
            <svg
                aria-hidden="true"
                viewBox="0 0 24 24"
                class="size-[1.1rem]"
                fill="none"
                stroke="currentColor"
                stroke-width="1.8"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="M10.5 4a6.5 6.5 0 1 1 0 13 6.5 6.5 0 0 1 0-13Zm5 11.5L20 20" />
            </svg>
        </A>
    }
}
