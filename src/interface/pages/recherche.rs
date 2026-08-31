//! `/fr/rechercher` — chercher dans le corpus.

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::domaine::recherche::{Portee, MINIMUM};
use crate::interface::design::{Bloc, Lien};
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
            chemin="/fr/rechercher"
        />
        <Bloc>
            <h1 class="mb-8">"Rechercher"</h1>

            // Un vrai formulaire, en `GET`. Sans JavaScript il marche quand
            // même : le navigateur compose l'adresse, le serveur rend la page.
            // C'est le même chemin que celui d'un lien partagé.
            <form method="get" action="/fr/rechercher" role="search" class="mb-10">
                <label class="block">
                    <span class="mb-2 block text-sm uppercase tracking-capitales text-encre-douce">
                        "Un mot, en français ou en hébreu"
                    </span>
                    <input
                        type="search"
                        name="q"
                        value=q
                        autocomplete="off"
                        placeholder="ruach, tohu, ברא…"
                        class="w-full rounded-sm border border-filet bg-surface/40 px-4 py-3 text-base text-encre placeholder:text-encre-douce/60 focus:border-accent focus:outline-none"
                    />
                </label>

                <div class="mt-4 flex flex-wrap items-center gap-4">
                    <div class="flex flex-wrap gap-2">
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
                                        <span class="block rounded-full border border-filet px-3 py-1 text-sm text-encre-douce peer-checked:border-accent peer-checked:text-encre-vive">
                                            {p.libelle()}
                                        </span>
                                    </label>
                                }
                            })
                            .collect_view()}
                    </div>
                    <button
                        type="submit"
                        class="rounded-full border border-accent px-5 py-1 text-sm uppercase tracking-capitales text-accent"
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
                                "Rien pour « " {mot} " ». Le corpus compte trois livres sur \
                                 soixante-dix : ce mot est peut-être dans un livre qui n'est \
                                 pas encore traduit."
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
        </Bloc>
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
        format!("{} : {}", t.unite_titre, t.verset)
    };
    let chemin = if t.verset == 0 {
        format!("/fr/lire/{}/{}", t.livre_id, t.unite_id)
    } else {
        format!("/fr/lire/{}/{}?v={}", t.livre_id, t.unite_id, t.verset)
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
