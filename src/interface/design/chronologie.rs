use leptos::prelude::*;

/// Une suite de jalons dans le temps.
///
/// Le filet vertical est ce qui la distingue d'une liste : il dit que ces
/// entrées ne sont pas juxtaposées mais **enchaînées**, et que l'ordre est la
/// moitié de ce qu'elles disent. Une dégradation qui s'étale sur dix-huit
/// siècles ne se démontre pas par cinq faits — elle se démontre par leur
/// suite.
///
/// Il est en or à faible opacité, comme tous les filets du site : sur une nuit
/// chaude, un gris paraît sale.
#[component]
pub fn Chronologie(children: Children) -> impl IntoView {
    view! {
        <ol class="relative m-0 mt-12 list-none border-s border-filet p-0 ps-8 sm:ps-10">
            {children()}
        </ol>
    }
}

/// Un jalon — une date, ce qui s'y produit, ce que ça change.
///
/// ## Le titre est un slot, et il l'a fallu
///
/// Il était un `String`. Un `String` ne peut porter aucune marque, donc
/// « La déportation à Bavel » et « La déportation en Mitsrayim » restaient en
/// encre pendant que les mêmes noms étaient en bordeaux trois lignes plus bas.
///
/// La règle du §5 vaut pour le titre autant que pour le corps : dans une
/// chronologie, le titre **est** le contenu — c'est lui qui nomme le fait. Un
/// prop de type texte fermait la porte à cette règle sans que rien ne le dise.
#[component]
pub fn Jalon(
    /// La date, telle qu'on la cite — « 312 av. », « IVᵉ siècle ».
    #[prop(into)]
    date: String,
    /// Ce qui se produit — il porte ses noms propres, donc c'est un slot.
    titre: Titre,
    children: Children,
) -> impl IntoView {
    view! {
        <li class="relative pb-14 last:pb-0">
            // La pastille se pose **sur** le filet, pas à côté : elle est
            // reculée de sa demi-largeur, et son anneau de nuit creuse le trait
            // au lieu de le recouvrir. Un point posé à côté du filet se lit
            // comme une puce ; posé dessus, il se lit comme un repère.
            //
            // Le retrait vaut celui du contenu **plus** le rayon de la
            // pastille — 2 rem + 0,25 sur téléphone, 2,5 + 0,25 au-dessus.
            // Calculé et non réglé à l'œil : un point décentré d'un pixel sur
            // un trait d'un pixel se lit comme un défaut de rendu.
            <span
                aria-hidden="true"
                class="absolute -start-9 top-2 size-2 rounded-full bg-or/70 ring-4 ring-nuit sm:-start-11"
            ></span>

            <p class="m-0 text-sm uppercase tracking-capitales text-accent">{date}</p>
            <p class="m-0 mt-2 font-titre text-xl text-encre-vive text-balance">{(titre.children)()}</p>
            <div class="mt-4 text-pretty">{children()}</div>
        </li>
    }
}

/// Ce que le jalon nomme — du contenu, pas une étiquette.
#[slot]
pub struct Titre {
    children: ChildrenFn,
}
