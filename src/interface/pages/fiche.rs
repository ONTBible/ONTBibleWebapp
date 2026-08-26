use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::api::fiche;
use crate::interface::design::verset::composer;
use crate::interface::design::{fournir_preferences, Blocs, Occurrences, PageDeLecture};
use crate::interface::tete::Tete;

/// `/fr/lexique/{lemme}` — la fiche d'un intraduisible.
///
/// ## C'est la page que l'or promet
///
/// Tout le site repose sur une distinction : **l'or promet une fiche et la
/// tient**, le bordeaux marque sans rien promettre. Chaque mot d'or du corpus,
/// de l'accueil, des essais, mène ici. Tant que cette page n'existait pas, l'or
/// mentait — et il mentait sur la page d'accueil, sur le premier verset que
/// voit un visiteur.
///
/// La fiche porte donc **et** la définition **et** les occurrences. Une
/// définition seule demanderait qu'on la croie sur parole ; les occurrences
/// donnent le corpus, qui est ce qui prouve.
#[component]
pub fn Fiche() -> impl IntoView {
    let parametres = use_params_map();
    let lemme = move || parametres.read().get("lemme").unwrap_or_default();

    let entree = Resource::new_blocking(lemme, |lemme| async move { fiche(lemme).await });

    // **Les réglages sont installés ici aussi**, bien que cette page n'offre
    // pas de panneau pour les changer.
    //
    // Ce qu'elle compose lit les réglages retenus — un lecteur qui a éteint les
    // gloses les veut éteintes ici aussi. Sans ce contexte, tout ce qui lit les
    // préférences reçoit un signal *constant* : la page s'affiche, le texte est
    // juste, et le réglage n'a jamais d'effet. La panne ressemble alors
    // exactement à un fonctionnement.
    let _preferences = fournir_preferences();

    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                match entree.await {
                    Ok(Some(f)) => {
                        let combien = f.occurrences.len();
                        let e = f.entree;

                        // La description porte le rendu, l'hébreu, et le nombre
                        // d'occurrences — dans cet ordre.
                        //
                        // Le rendu d'abord parce que c'est la décision de
                        // traduction, donc la chose la plus dense qu'on puisse
                        // mettre en une ligne. L'hébreu ensuite parce qu'on
                        // cherche aussi בָּרָא, et qu'une fiche qui ne le porte pas
                        // dans sa description ne répond pas à cette recherche.
                        //
                        // Le compte enfin, et il n'est pas décoratif : il dit
                        // que la page tient sa promesse. « bara — orchestrer »
                        // seul fait cinquante-deux signes, un tiers de ce qu'un
                        // moteur affiche — et sous soixante-dix signes il ne la
                        // montre plus du tout, il lui préfère un extrait pris
                        // dans la page, qui commence par une définition sans
                        // son mot.
                        let hebreu_cite = if e.hebreu.is_empty() {
                            String::new()
                        } else {
                            format!(" ({})", e.hebreu)
                        };
                        // Un rendu qui répète le lemme n'est pas un rendu.
                        //
                        // C'est le cas des intraduisibles **purs** — *nefesh*
                        // reste *nefesh*, *shem* reste *shem* —, et c'est le
                        // cœur du projet : ces mots-là sont précisément ceux
                        // qu'on a refusé de traduire. Les afficher comme
                        // « Nefesh : Nefesh » donnait un titre qui a l'air
                        // cassé, là où c'est la promesse même de la fiche.
                        //
                        // La comparaison est insensible à la casse : le vault
                        // capitalise le titre d'une fiche, pas toujours son
                        // rendu.
                        let rendu_distinct = !e.rendu.is_empty()
                            && !e.rendu.eq_ignore_ascii_case(&e.titre);
                        let rendu_cite = if rendu_distinct {
                            format!(" — {}", e.rendu)
                        } else {
                            String::new()
                        };
                        let occurrences_citees = match combien {
                            0 => String::new(),
                            1 => ", et l'unique verset où il paraît".to_string(),
                            n => format!(", et les {n} versets où il paraît"),
                        };
                        let description = if rendu_distinct {
                            format!(
                                "{}{hebreu_cite}{rendu_cite}. Un intraduisible hébreu de La \
                                 Bible ONT : ce qu'il porte{occurrences_citees}.",
                                e.titre,
                            )
                        } else {
                            format!(
                                "{}{hebreu_cite} — l'intraduisible hébreu que La Bible ONT \
                                 laisse debout : ce qu'il porte, pourquoi il ne se traduit \
                                 pas{occurrences_citees}.",
                                e.titre,
                            )
                        };
                        let hebreu = e.hebreu.clone();
                        let rendu = e.rendu.clone();
                        let formes = e.formes.clone();
                        let titre = e.titre.clone();

                        view! {
                            // Le titre porte le rendu **et** le mot « hébreu ».
                            //
                            // « bara » seul ne répond qu'à qui l'écrit déjà
                            // correctement. On cherche « bara hébreu », « bara
                            // signification », « bara traduction » — et la fiche
                            // répond à ces trois questions sans qu'aucune ne
                            // figurât dans son titre.
                            <Tete
                                titre=if rendu_distinct {
                                    format!("{} (hébreu) : {}", e.titre, e.rendu)
                                } else {
                                    format!("{}, l'intraduisible hébreu", e.titre)
                                }
                                description=description
                                chemin=format!("/fr/lexique/{}", e.lemme)
                            />

                            <PageDeLecture
                                fil=vec![("/fr/lexique".to_string(), "Lexique".to_string())]
                                rappel="Intraduisible"
                                titre=titre
                                chapeau=Box::new(move || {
                                    view! {
                                        {(!hebreu.is_empty())
                                            .then(|| {
                                                view! {
                                                    // Un `span` en ligne, et non
                                                    // un bloc en `dir="rtl"` :
                                                    // celui-ci alignerait le mot
                                                    // à droite de l'écran, où il
                                                    // flotte seul, détaché du
                                                    // titre qu'il double.
                                                    <p class="mb-4">
                                                        <span
                                                            dir="rtl"
                                                            lang="he"
                                                            class="font-hebreu text-3xl text-encre-vive"
                                                        >
                                                            {hebreu}
                                                        </span>
                                                    </p>
                                                }
                                            })}
                                        {(!rendu.is_empty())
                                            .then(|| {
                                                view! {
                                                    <p class="text-lg italic text-accent text-pretty">
                                                        {composer(&rendu)}
                                                    </p>
                                                }
                                            })}
                                        // Les formes attestées, quand elles
                                        // diffèrent du lemme : c'est ce qui permet
                                        // de reconnaître le mot dans le texte, où
                                        // il paraît fléchi.
                                        {(formes.len() > 1)
                                            .then(|| {
                                                view! {
                                                    <p class="mt-4 text-sm text-encre-douce">
                                                        "Formes : " {formes.join(", ")}
                                                    </p>
                                                }
                                            })}
                                    }
                                        .into_any()
                                })
                            >
                                <Blocs blocs=e.definition />
                                {(!f.occurrences.is_empty())
                                    .then(|| view! { <Occurrences occurrences=f.occurrences /> })}
                            </PageDeLecture>
                        }
                            .into_any()
                    }
                    _ => view! { <Absente /> }.into_any(),
                }
            })}
        </Suspense>
    }
}

/// La fiche qui n'existe pas.
///
/// Elle devrait être introuvable : les liens d'or sont produits depuis le même
/// pipeline que le lexique, et un test vérifie que chaque occurrence pointe un
/// verset réel. Si quelqu'un arrive ici, c'est par une adresse tapée — ou parce
/// qu'un terme a été retiré du glossaire sans que le corpus le sache.
#[component]
fn Absente() -> impl IntoView {
    view! {
        <Tete
            titre="Fiche introuvable"
            description="Ce terme n'a pas de fiche dans le lexique."
            chemin="/fr/lexique"
        />
        <leptos_meta::Meta name="robots" content="noindex, follow" />

        <PageDeLecture
            fil=vec![("/fr/lexique".to_string(), "Lexique".to_string())]
            rappel="Les intraduisibles"
            titre="Ce terme n'a pas de fiche"
        >
            <p class="text-encre-douce text-pretty">
                "Le lexique porte les termes que la restitution a choisi de laisser debout. \
                 Celui-ci n'en fait pas partie — ou pas encore."
            </p>
        </PageDeLecture>
    }
}
