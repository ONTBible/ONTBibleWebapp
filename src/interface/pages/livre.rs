use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::api::livre;
use crate::interface::design::{fournir_preferences, ListeDUnites, PageDeLecture};
use crate::interface::tete::Tete;

/// `/fr/webapp/{livre}` — les unités d'un livre.
///
/// L'étage qui manquait entre le sommaire et le texte. Sans lui, la liseuse
/// n'aurait que deux états : le plan des soixante-dix livres, et un chapitre
/// isolé — et rien pour passer de l'un à l'autre autrement qu'en devinant une
/// adresse.
#[component]
pub fn Livre() -> impl IntoView {
    let parametres = use_params_map();
    let identifiant = move || parametres.read().get("livre").unwrap_or_default();

    let ouvrage = Resource::new_blocking(identifiant, |id| async move { livre(id).await });

    // **Les réglages sont installés ici aussi**, bien que cette page n'offre
    // pas de panneau pour les changer.
    //
    // Ce qu'elle compose lit les réglages retenus — un lecteur qui a éteint les
    // gloses les veut éteintes ici aussi. Sans ce contexte, tout ce qui lit les
    // préférences reçoit un signal *constant* : la page s'affiche, le texte est
    // juste, et le réglage n'a jamais d'effet. La panne ressemble alors
    // exactement à un fonctionnement.
    let _preferences = fournir_preferences();

    let edition = crate::interface::arbre::sous_l_edition();

    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                match ouvrage.await {
                    Ok(Some(livre)) => {
                        let description = format!(
                            "{} — {}. {} unités traduites, {} versets, dans La Bible ONT.",
                            livre.titre,
                            livre.francais,
                            livre.unites.len(),
                            livre.versets,
                        );
                        let hebreu = livre.hebreu.clone();
                        // **Le second nom, et il manquait.**
                        //
                        // Le commentaire du chapeau, deux écrans plus bas,
                        // promet « le second nom sous le titre, petit, avec le
                        // nom hébreu ». Il n'y était pas : la valeur était
                        // relevée, puis jamais posée — un `unused_variable` que
                        // rien d'autre ne signalait, la page s'affichant très
                        // bien sans.
                        //
                        // Même garde que la balise de titre : un livre dont le
                        // nom ONT contient déjà son nom français — ou qui n'en
                        // a pas — ne le redit pas.
                        let francais = (!livre.francais.is_empty()
                            && !livre.titre.contains(&livre.francais))
                            .then(|| livre.francais.clone());
                        let unites = livre.unites.len();
                        let versets = livre.versets;
                        view! {
                            // Le renvoi reçu entre au titre, comme sur un
                            // passage : le sommaire d'un livre est la page
                            // qu'on atteint en cherchant « Genèse », et
                            // « Bereshit » seul ne répond à personne qui ne
                            // connaît pas déjà le projet.
                            <Tete
                                titre=if livre.francais.is_empty()
                                    || livre.titre.contains(&livre.francais)
                                {
                                    livre.titre.clone()
                                } else {
                                    format!("{} ({})", livre.titre, livre.francais)
                                }
                                description=description
                                chemin=crate::domaine::chemins::livre(crate::interface::arbre::arbre_maintenant(), &livre.id)
                            />

                            <PageDeLecture
                                liste=true
                                fil=vec![crate::interface::arbre::maillon_de_la_bible()]
                                // **Le second nom change de place selon le
                                // registre, et une seule fois.**
                                //
                                // Sous l'app il est *sous* le titre, petit, avec
                                // le nom hébreu — c'est ce que fait le chapeau,
                                // et l'app n'a pas de rappel. Sous l'édition il
                                // remonte en capitales espacées au-dessus du
                                // titre, comme `main` : « GENÈSE », puis
                                // « Bereshit ».
                                //
                                // Il n'est donc jamais écrit deux fois — le
                                // chapeau le tait quand le rappel le porte.
                                //
                                // Le fil d'Ariane reste des deux côtés : il tient
                                // la place du « ‹ » de l'app, qui est une pile
                                // native que le web n'a pas.
                                rappel=if edition {
                                    francais.clone().unwrap_or_default()
                                } else {
                                    String::new()
                                }
                                titre=livre.titre.clone()
                                // **Le chapeau tient sur une ligne**, comme
                                // chez l'app. Il portait le nom hébreu en corps
                                // 2xl sur sa propre ligne, puis le compte sur
                                // une seconde : trois lignes de titre avant la
                                // première entrée de la liste.
                                //
                                // L'app met le second nom sous le titre, petit,
                                // et rien d'autre. Le compte d'unités et de
                                // versets reste — c'est une mesure du chantier,
                                // et elle n'existe pas chez elle — mais il
                                // rejoint la même ligne que l'hébreu.
                                chapeau=Some(Box::new(move || {
                                    view! {
                                        <p class="m-0 flex flex-wrap items-baseline gap-x-3 gap-y-1">
                                            {francais
                                                .filter(|_| !edition)
                                                .map(|francais| {
                                                    view! {
                                                        <span class="italic text-encre-douce">
                                                            {francais}
                                                        </span>
                                                    }
                                                })}
                                            <span
                                                dir="rtl"
                                                lang="he"
                                                class="font-hebreu text-[1.05em] text-encre-douce"
                                            >
                                                {hebreu}
                                            </span>
                                            <span class="chiffres-tableau text-sm text-encre-douce">
                                                {unites} " unités · " {versets} " versets"
                                            </span>
                                        </p>
                                    }
                                        .into_any()
                                }))
                            >
                                <ListeDUnites livre=livre.id unites=livre.unites />
                            </PageDeLecture>
                        }
                            .into_any()
                    }
                    // Un livre du plan qui n'a pas encore de texte, ou un
                    // identifiant inventé. Les deux méritent la même réponse :
                    // le sommaire ne mène jamais ici, donc on y arrive par une
                    // adresse tapée ou un lien ancien.
                    _ => view! { <Absent /> }.into_any(),
                }
            })}
        </Suspense>
    }
}

/// Ce que voit quelqu'un dont le livre n'existe pas — ou pas encore.
///
/// Elle ne dit pas « erreur » : dans un corpus dont soixante-sept livres
/// restent à traduire, « ce livre n'est pas encore là » est la réponse vraie
/// dans la grande majorité des cas.
#[component]
fn Absent() -> impl IntoView {
    view! {
        <Tete
            titre="Livre introuvable"
            description="Ce livre n'a pas encore été restitué."
            chemin=crate::domaine::chemins::bible(crate::interface::arbre::arbre_maintenant())
        />
        <leptos_meta::Meta name="robots" content="noindex, follow" />

        <PageDeLecture
            fil=vec![crate::interface::arbre::maillon_de_la_bible()]
            rappel="Le corpus"
            titre="Ce livre n'est pas encore là"
        >
            <p class="text-encre-douce text-pretty">
                // **Le reste à traduire se calcule**, il ne s'écrit pas.
                //
                // Il disait « soixante-sept des soixante-dix », c'est-à-dire
                // 70 − 3, quand le vault en porte cinq. Le même écart que
                // l'accueil, sur une page qu'on n'atteint qu'en cherchant un
                // livre absent — donc au moment précis où l'on compte.
                {crate::domaine::nombres::en_lettres_capitale(
                    env!("CORPUS_LIVRES").parse::<u32>().unwrap_or(0)
                        - env!("CORPUS_LIVRES_ECRITS").parse::<u32>().unwrap_or(0),
                )}
                " des "
                {crate::domaine::nombres::en_lettres(
                    env!("CORPUS_LIVRES").parse().unwrap_or(0),
                )}
                " livres attendent leur restitution. Le sommaire dit lesquels se lisent \
                 aujourd'hui."
            </p>
        </PageDeLecture>
    }
}
