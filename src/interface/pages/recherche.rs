//! `/fr/rechercher` — chercher dans le corpus.

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::domaine::recherche::{Portee, MINIMUM};
use crate::interface::design::{Lien, PageDeLecture};
use crate::interface::tete::Tete;

/// La page de recherche.
///
/// ## La requête vit dans l'adresse, et c'est délibéré
///
/// Contrairement au filtre de couleur des surlignages — un regard qu'on porte
/// sur sa propre liste —, une recherche **se partage** : « regarde ce que donne
/// *ruach* ». Elle se met donc en signet, se recharge, et revient par le bouton
/// « précédent » du navigateur, qui est le geste naturel après avoir ouvert un
/// résultat.
///
/// Elle est aussi rendue par le serveur : le premier écran de résultats est
/// dans le HTML, donc lisible sans JavaScript et par un moteur.
#[component]
pub fn Recherche() -> impl IntoView {
    let requete = use_query_map();

    let q = move || requete.read().get("q").unwrap_or_default();
    let ou = move || Portee::depuis_cle(&requete.read().get("ou").unwrap_or_default());

    let trouvailles = Resource::new_blocking(
        move || (q(), ou().cle().to_string()),
        |(q, ou)| async move {
            if q.trim().chars().count() < MINIMUM {
                return Ok(Vec::new());
            }
            crate::api::rechercher(q, ou).await
        },
    );

    view! {
        <Tete
            titre="Rechercher dans le corpus hébreu"
            description="Chercher un mot dans La Bible ONT — dans le texte, dans les gloses, \
                         ou en hébreu. Les résultats mènent au verset."
            chemin=crate::domaine::chemins::rechercher(crate::interface::arbre::arbre_maintenant())
        />
        // ## C'est un écran de la webapp, et ce paragraphe disait le contraire
        //
        // Elle portait un `Hero` — une ouverture plein écran, avec le massif et
        // la marque — et le §8 undecies s'en servait pour l'exclure du thème :
        // *« la recherche n'en est pas, bien qu'elle rende du corpus : elle
        // porte une ouverture, avec le même massif, et un bouton “Chercher” en
        // or qui disparaît sur du clair. »*
        //
        // L'argument était juste et il est devenu faux le jour où la recherche
        // a cessé d'être une page du site pour devenir **un bouton de la barre
        // de la Bible** (§8 undecies, 29 septembre 2026). On y arrive
        // maintenant depuis l'intérieur de la liseuse : garder l'ouverture
        // ferait exactement ce que « Vous » faisait encore ce matin — sortir de
        // la webapp d'un toucher, sans barre, sans thème, sans retour visible.
        //
        // La règle du §8 undecies n'a pas bougé pour autant, et c'est elle qui
        // tranche : **les pages de la liseuse sont exactement celles qui
        // emploient `PageDeLecture`.** La recherche en est une désormais, donc
        // elle l'emploie — et le thème la suit par construction, sans table de
        // chemins à tenir d'accord.
        //
        // Ce que l'ancien montage avait raison de vouloir : que le champ soit
        // **immédiatement là**. Il l'est — premier objet sous le titre, sans
        // un écran de défilement devant lui.
        <PageDeLecture liste=true titre="Rechercher">
            // Un vrai formulaire, en `GET`. Sans JavaScript il marche quand
            // même : le navigateur compose l'adresse, le serveur rend la page.
            // C'est le même chemin que celui d'un lien partagé.
            <form method="get" action=crate::domaine::chemins::rechercher(crate::interface::arbre::arbre_maintenant()) role="search" class="mb-8">
                <label class="block">
                    <span class="sr-only">"Le mot à chercher"</span>
                    <input
                        type="search"
                        name="q"
                        value=q
                        autocomplete="off"
                        // **Pas de capitale automatique.** C'est le seul des
                        // accidents de `ONTPlateformes.swift` que la session
                        // macOS jugeait transposable, et c'est exactement ici
                        // qu'il vaut : un clavier mobile capitalise le premier
                        // mot, et l'on cherche `ruach`, pas `Ruach`.
                        autocapitalize="none"
                        spellcheck="false"
                        placeholder="ruach, tohu, ברא…"
                        class="verre w-full rounded-full px-5 py-3 text-base text-encre placeholder:text-encre-douce/60 focus:outline focus:outline-2 focus:outline-accent"
                    />
                </label>

                // Les portées passent par les **segments de l'app**, comme le
                // lexique : quatre choix exclusifs sur une rangée, et la
                // capsule dit lequel tient. C'étaient des pastilles cerclées,
                // qui se lisaient comme quatre boutons indépendants.
                // **Une seule rangée**, segments à gauche et bouton à droite.
                // `flex-wrap` les laissait passer à la ligne, et un segmenté
                // qui se replie cesse d'en être un — on y lit alors trois
                // boutons empilés, dont rien ne dit qu'ils s'excluent.
                <div class="mt-3 flex items-center gap-3">
                    // **Largeur au contenu**, et non trois parts égales : à parts
                    // égales, le plus long — « Partout » — décide pour les
                    // trois et se tronque quand même, ce qui donne « Part… »
                    // sur l'option qui est le **défaut**. Un segmenté dont
                    // l'option courante est illisible ne dit plus où l'on est.
                    <div class="flex shrink-0 rounded-full border border-filet/60 bg-surface/60 p-1">
                        {Portee::toutes()
                            .into_iter()
                            .map(|p| {
                                view! {
                                    <label class="cursor-pointer">
                                        <input
                                            type="radio"
                                            name="ou"
                                            value=p.cle()
                                            checked=move || ou() == p
                                            class="peer sr-only"
                                        />
                                        <span class="segment block rounded-full px-3.5 py-1.5 text-center text-sm text-encre-douce peer-checked:bg-encre/10 peer-checked:text-marque-encre peer-focus-visible:outline peer-focus-visible:outline-2 peer-focus-visible:outline-accent">
                                            {p.libelle()}
                                        </span>
                                    </label>
                                }
                            })
                            .collect_view()}
                    </div>
                    <button
                        type="submit"
                        class="presse verre shrink-0 rounded-full px-5 py-1.5 font-titre text-sm text-accent"
                    >
                        "Chercher"
                    </button>
                </div>
            </form>

            <Suspense fallback=|| {
                view! { <p class="text-encre-douce">"…"</p> }
            }>
                {move || Suspend::new(async move {
                    let mot = q();
                    if mot.trim().chars().count() < MINIMUM {
                        return view! {
                            <p class="text-encre-douce">
                                "Deux lettres au moins. Une seule ramènerait la moitié du corpus."
                            </p>
                        }
                            .into_any();
                    }
                    let liste = trouvailles.await.unwrap_or_default();
                    if liste.is_empty() {
                        return view! {
                            <p class="text-encre-douce">
                                "Rien pour « " {mot} " ». Le corpus compte "
                                {crate::domaine::nombres::en_lettres(
                                    env!("CORPUS_LIVRES_ECRITS").parse().unwrap_or(0),
                                )}
                                " livres sur "
                                {crate::domaine::nombres::en_lettres(
                                    env!("CORPUS_LIVRES").parse().unwrap_or(0),
                                )}
                                " : ce mot est peut-être dans un livre qui n'est pas \
                                 encore traduit."
                            </p>
                        }
                            .into_any();
                    }
                    let combien = liste.len();
                    view! {
                        <p class="chiffres-tableau mb-8 text-sm text-encre-douce">
                            {combien} " résultat" {(combien > 1).then_some("s")}
                        </p>
                        <ul class="m-0 list-none p-0">
                            {liste
                                .into_iter()
                                .map(|t| view! { <UneTrouvaille t /> })
                                .collect_view()}
                        </ul>
                    }
                        .into_any()
                })}
            </Suspense>
        </PageDeLecture>
    }
}

/// Un résultat.
///
/// L'extrait passe par `composer` : il vient du corpus, donc il porte les
/// espaces ordinaires devant les ponctuations doubles que le français veut
/// insécables. C'est la règle du §8 bis, et elle vaut pour toute chaîne du
/// corpus posée dans une page.
#[component]
fn UneTrouvaille(t: crate::api::TrouvailleDto) -> impl IntoView {
    // Un renvoi sans numéro pour un titre de section : « Bereshit 1:0 »
    // désignerait un verset qui n'existe pas.
    let renvoi = if t.verset == 0 {
        t.unite_titre.clone()
    } else {
        format!("{} : {}", t.unite_titre, t.verset)
    };
    let chemin = if t.verset == 0 {
        crate::domaine::chemins::unite(
            crate::interface::arbre::arbre_maintenant(),
            &t.livre_id,
            &t.unite_id,
        )
    } else {
        crate::domaine::chemins::unite_au_verset(
            crate::interface::arbre::arbre_maintenant(),
            &t.livre_id,
            &t.unite_id,
            &t.verset.to_string(),
        )
    };

    view! {
        <li class="mb-6 border-s border-filet ps-4">
            <div class="flex items-baseline justify-between gap-4">
                <Lien href=chemin>
                    <span class="chiffres-tableau text-sm text-encre-douce">{renvoi}</span>
                </Lien>
                // La glose est dite, pas montrée autrement : sans ce mot, on ne
                // comprend pas pourquoi le texte affiché ne contient pas ce
                // qu'on a tapé.
                {t
                    .dans_une_glose
                    .then(|| {
                        view! {
                            <span class="shrink-0 text-sm text-encre-douce/70">"dans une glose"</span>
                        }
                    })}
            </div>
            <p class="mt-1 mb-0">
                {crate::interface::design::verset::composer(&t.extrait)}
            </p>
        </li>
    }
}
