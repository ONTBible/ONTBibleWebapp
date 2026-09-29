//! La liste groupée d'iOS — le gabarit que le site n'avait pas.
//!
//! ## Ce qui manquait, et ce n'était pas un jeton
//!
//! Le portage du 21 septembre a donné au site les **couleurs**, les **fontes**,
//! les **rayons** et la **navigation** de l'app. L'auteur a regardé et a dit :
//! « on est clairement pas sur une copie conforme — je veux pas juste une copie
//! des features, je veux aussi une copie de l'UI. »
//!
//! Il avait raison, et la comparaison le montrait sans discussion. Mises côte à
//! côte, la même page ne se ressemblait pas :
//!
//! | | l'app | le site |
//! |---|---|---|
//! | la liste | une **carte arrondie encartée**, filets fins entre les lignes | des lignes à plat |
//! | une ligne | titre + sous-titre à gauche, valeur + **chevron** à droite | titre et plage, pastille en dessous |
//! | le titre d'écran | un grand titre serré à gauche | œil-de-bœuf, titre d'affiche, **et un paragraphe** |
//! | l'en-tête de section | deux lignes petites | un grand titre avec le signe de la montagne |
//!
//! Le site composait une **édition** là où l'app compose une **liste**. Ce
//! n'est pas une différence de peau : c'est un gabarit absent.
//!
//! ## Ce que le gabarit dit, et que le site disait autrement
//!
//! Une liste iOS porte trois affirmations dans sa forme, et le site n'en
//! portait aucune :
//!
//! - **ce qui est dans la carte est une liste de destinations** — l'encart et
//!   le fond la détachent de la page, donc on sait qu'on choisit ;
//! - **chaque ligne mène quelque part** — c'est le chevron qui le dit, et il
//!   le dit sans un mot. Le site n'en avait pas : rien ne distinguait une ligne
//!   qu'on touche d'une ligne qu'on lit ;
//! - **ce qui est à droite est une mesure, pas une action** — le compteur y
//!   vit, gris, aligné, et l'œil apprend en deux lignes à ne plus le lire.
//!
//! ## Le filet part du texte, pas du bord
//!
//! Relevé au pixel sur la capture : la ligne fine commence à l'aplomb du titre
//! et court jusqu'au bord de la carte. C'est ce qui fait qu'on lit **un groupe**
//! et non des bandes empilées — un filet bord à bord découpe, un filet en
//! retrait relie.

use leptos::prelude::*;
use leptos_router::components::A;

/// L'en-tête d'une section, au-dessus de la carte.
///
/// Deux lignes : le nom, puis sa glose. Le nom prend l'encre de la marque
/// — `brandInk` chez l'app —, qui est l'aubergine sur un thème clair et l'or
/// sur un thème sombre. La glose reste grise.
///
/// **Son titre est un `slot` et non une chaîne**, et c'est la règle du §5 :
/// *ce texte peut-il contenir un nom propre ou un intraduisible ?* Ici oui —
/// « Kenesset » en est un.
#[component]
pub fn EnteteDeSection(
    children: Children,
    /// La seconde ligne. Absente quand elle redirait le titre.
    #[prop(optional)]
    glose: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="mt-8 mb-2 px-1 first:mt-0">
            <p class="m-0 font-titre text-[1.05em] font-semibold text-marque-encre">{children()}</p>
            {glose
                .map(|glose| {
                    view! {
                        <p class="m-0 mt-0.5 text-[0.9em] text-encre-douce">{glose()}</p>
                    }
                })}
        </div>
    }
}

/// La carte qui groupe les lignes.
///
/// `overflow-hidden` n'est pas décoratif : sans lui, le fond d'une ligne
/// survolée déborde des coins arrondis, et le premier angle de la carte se
/// remplit d'un carré de couleur.
///
/// Le filet de la dernière ligne se retire ici et non dans la ligne
/// elle-même — une ligne ne sait pas qu'elle est la dernière, et lui faire
/// porter cette question l'obligerait à compter ses sœurs.
#[component]
pub fn Groupe(children: Children) -> impl IntoView {
    view! {
        <ul class="carte-de-liste m-0 list-none overflow-hidden rounded-carte bg-surface p-0 [&>li:last-child_.filet]:border-0">
            {children()}
        </ul>
    }
}

/// Le chevron — ce qui dit qu'une ligne mène quelque part.
#[component]
fn Chevron() -> impl IntoView {
    view! {
        <svg
            aria-hidden="true"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2.4"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="size-[0.72em] shrink-0 text-encre-douce/60"
        >
            <path d="m9 5 7 7-7 7" />
        </svg>
    }
}

/// Une ligne de liste.
///
/// Sans `chemin`, elle ne se touche pas et ne porte pas de chevron : c'est la
/// forme d'un livre annoncé mais pas encore traduit. **Le chevron promet, comme
/// l'or promet une fiche** — en poser un sur une ligne inerte serait la même
/// faute que d'écrire un intraduisible en or sans lui donner de fiche.
#[component]
pub fn Ligne(
    /// Où elle mène. Absent : la ligne est inerte, et se lit en encre douce.
    ///
    /// **`optional_no_strip`**, et il a fallu deux essais pour trouver
    /// pourquoi. `optional` seul *déshabille* l'`Option` : le prop devient un
    /// `String`, et son absence vaut `None`. C'est juste quand on écrit
    /// `chemin="/fr/lire"` ; c'est faux ici, où l'appelant a déjà un
    /// `Option<String>` — « elle mène quelque part **si** le livre est écrit ».
    ///
    /// Avec `into` en plus, Rust demandait un `String: From<Option<String>>` et
    /// parlait d'un trait au lieu de la vraie question.
    #[prop(optional_no_strip)]
    chemin: Option<String>,
    /// Le titre — un `slot`, pour qu'il puisse porter un nom hébreu.
    titre: Children,
    /// La seconde ligne, sous le titre.
    #[prop(optional)]
    sous_titre: Option<Children>,
    /// Ce qui s'aligne à droite — un compteur, une mesure.
    #[prop(optional)]
    valeur: Option<Children>,
    /// Une pastille dans la ligne, avant la valeur.
    #[prop(optional)]
    pastille: Option<Children>,
) -> impl IntoView {
    let dedans = move || {
        view! {
            // Le filet porte le retrait du texte à gauche et court jusqu'au
            // bord à droite : un filet bord à bord découpe, un filet en retrait
            // relie. Relevé au pixel sur la capture de l'app.
            //
            // **`flex-1` et il le fallait.** Sans lui cette boîte se serrait sur
            // son contenu, et la valeur — « 1/6 » — se posait juste après le
            // titre au lieu d'aller au bord. Une colonne de compteurs qui ne
            // s'aligne pas cesse d'être une colonne : l'œil doit la chercher à
            // chaque ligne, ce qui est exactement ce qu'une liste évite.
            <div class="filet flex flex-1 items-center gap-3 border-b border-filet/50 py-2.5 pe-4">
                <div class="min-w-0 flex-1">
                    // `truncate` et non un retour à la ligne : « Chapitre 1 »
                    // coupé en deux lignes fait perdre à la liste son pas
                    // régulier, et c'est ce pas qui permet de balayer soixante
                    // entrées sans les lire.
                    <div class="truncate font-titre text-[1.02em] font-semibold leading-tight">
                        {titre()}
                    </div>
                    {sous_titre
                        .map(|s| {
                            view! {
                                <div class="truncate text-[0.86em] leading-tight text-encre-douce">
                                    {s()}
                                </div>
                            }
                        })}
                </div>
                {pastille.map(|p| p())}
                {valeur
                    .map(|v| {
                        view! {
                            <span class="shrink-0 text-[0.88em] tabular-nums text-encre-douce">
                                {v()}
                            </span>
                        }
                    })}
            </div>
        }
    };

    view! {
        <li class="ps-4">
            {match chemin {
                Some(chemin) => {
                    view! {
                        // **`<A>` et non `<a>`.** C'est la règle de `lien.rs`,
                        // et elle y est écrite depuis août : *un chemin du site
                        // passe par le routeur — sinon la page entière se
                        // recharge pour un lien interne.*
                        //
                        // Ces listes l'avaient enfreinte le jour de leur
                        // écriture. Chaque toucher rechargeait le document :
                        // la liseuse avait l'apparence d'une app et la
                        // mécanique d'un site des années deux mille.
                        //
                        // C'est ce que l'auteur a senti sans le nommer, en
                        // demandant une webapp « en CSR/SPA ». Elle l'est déjà
                        // — Leptos navigue côté client une fois hydraté —, et
                        // c'étaient mes liens qui s'en excluaient.
                        <A
                            href=chemin
                            attr:class="flex items-center gap-3 text-encre no-underline transition-colors hover:text-encre-vive"
                        >
                            {dedans()}
                            <Chevron />
                        </A>
                    }
                        .into_any()
                }
                // Inerte : pas de chevron, et l'encre baisse. La ligne dit
                // qu'elle existe et qu'elle n'est pas encore là.
                None => view! { <div class="text-encre-douce">{dedans()}</div> }.into_any(),
            }}
        </li>
    }
}

/// Une pastille d'état — « brouillon », et ce que l'app en fait.
///
/// Petite, grise, **dans** la ligne. Le site la posait en or bordé sur sa
/// propre ligne : elle y pesait autant que le titre, alors qu'elle dit une
/// réserve, pas un titre.
#[component]
pub fn Pastille(children: Children) -> impl IntoView {
    view! {
        <span class="shrink-0 rounded-full bg-encre-douce/15 px-2 py-0.5 text-[0.72em] uppercase tracking-[0.08em] text-encre-douce">
            {children()}
        </span>
    }
}

/// Le grand titre d'un écran de liste.
///
/// Celui de l'app : serré à gauche, gras, avec un sous-titre discret dessous.
/// Pas d'œil-de-bœuf, pas de paragraphe — **une liste s'ouvre, elle ne
/// s'introduit pas.** C'est la différence de registre entre une édition, qui
/// présente, et une liseuse, où l'on revient.
#[component]
pub fn TitreDeListe(
    children: Children,
    #[prop(optional)] sous_titre: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="mb-6 px-1">
            <h1 class="m-0 font-titre text-2xl font-semibold leading-none text-encre-vive">
                {children()}
            </h1>
            {sous_titre
                .map(|s| {
                    view! {
                        <p class="m-0 mt-1.5 text-[0.9em] text-encre-douce">{s()}</p>
                    }
                })}
        </div>
    }
}
