use leptos::prelude::*;

use crate::api::{ma_position, sommaire};
use crate::interface::design::{
    fournir_preferences, BoutonDeRecherche, CarteDeReprise, PageDeLecture, Sommaire,
};
use crate::interface::tete::Tete;

/// `/fr/webapp` — le sommaire du corpus.
///
/// C'est la porte de la liseuse, et la première page du site où l'on ne
/// **dit** pas ce qu'est l'ONT : on le montre en donnant le plan entier. Trois
/// livres en or au milieu de soixante-sept en encre atténuée disent l'état du
/// chantier plus vite qu'une phrase, et sans se périmer — les chiffres viennent
/// du pipeline.
#[component]
pub fn Lire() -> impl IntoView {
    // Le sommaire nomme les unités selon le registre choisi — « Chapitre 3 »
    // ou « Parashah 3 » —, donc il **lit** les préférences. Sans ce
    // fournisseur, `preferences()` retombait sur un signal constant : le
    // réglage n'avait aucun effet sur cette page, et rien ne le disait.
    //
    // La correction du 25 août avait couvert `livre.rs` et `fiche.rs` en
    // relevant qui appelait `preferences()` **directement**. Celle-ci consomme
    // par composant interposé : `Lire` ne cite ni `preferences` ni
    // `nom_d_unite`, c'est `Sommaire` qui les appelle. Un relevé par appel
    // direct ne pouvait pas la voir.
    //
    // C'est le `debug_assert!` posé le même jour qui l'a trouvée, en faisant
    // paniquer la page en développement. En `--release` il ne s'arme pas : la
    // page s'affichait, se lisait, et le réglage restait mort.
    let _preferences = fournir_preferences();

    let plan = Resource::new_blocking(|| (), |_| async { sommaire().await });

    // **La position, et une ressource séparée du plan.** Le sommaire est le
    // même pour tout le monde et se met en cache au bord ; la position
    // appartient au lecteur et ne doit jamais y entrer. Les fondre en une
    // seule réponse rendrait le plan incachable pour gagner un aller-retour.
    //
    // Elle n'est pas `blocking` : le plan est ce qu'on vient chercher, et
    // retarder le premier octet du corpus pour un signet inverserait les
    // priorités. La carte se pose après, comme le bouton « aA ».
    let position = Resource::new(|| (), |_| async { ma_position().await });

    // **Sous quel registre.** L'édition présente le corpus, l'app l'ouvre — et
    // c'est tout l'écart que l'auteur a relevé le 2 octobre 2026 en mettant les
    // deux écrans côte à côte : *« pour l'UI de la liseuse je veux vraiment la
    // prod »*.
    let edition = crate::interface::arbre::sous_l_edition();

    view! {
        <Tete
            // Même règle qu'au lexique : « Lire » nomme une action dans une
            // navigation, il ne nomme pas un contenu pour un moteur.
            titre="Lire le corpus hébreu et araméen"
            // Le total aussi vient du pipeline : le plan du corpus se
            // remanie, et une description qui l'écrit à la main mentirait au
            // premier remaniement — dans un résultat de recherche, là où
            // personne ne la relit.
            description=format!(
                "Le corpus de La Bible ONT — les {} livres du Kenesset et de la Berit \
                 Hadashah, et l'état de leur restitution.",
                crate::domaine::nombres::en_lettres(env!("CORPUS_LIVRES").parse().unwrap_or(0)),
            )
            chemin=crate::domaine::chemins::bible(crate::interface::arbre::arbre_maintenant())
        />

        // **Deux registres pour une même liste.**
        //
        // *Sous l'app* : ni œil-de-bœuf ni chapeau, et le titre est celui de
        // l'écran — « La Bible ONT ». Une liste ne s'introduit pas, on y
        // revient ; et ce que le chapeau disait, la forme le dit — une ligne
        // sans chevron ne se touche pas.
        //
        // *Sous l'édition* : le rappel, le titre de la navigation et le chapeau
        // de `main`. « Lire » suffit ici parce que la page **présente** le
        // corpus à qui arrive de l'accueil, là où l'app s'adresse à qui revient.
        <PageDeLecture
            liste=true
            rappel=if edition { "Le corpus" } else { "" }
            titre=if edition { "Lire" } else { "La Bible ONT" }
            chapeau=edition
                .then(|| {
                    Box::new(|| {
                        view! {
                            <p class="text-encre-douce text-pretty">
                                "Le plan entier, et ce qui en est traduit. Les titres en or se lisent ; \
                                 les autres attendent leur tour."
                            </p>
                        }
                            .into_any()
                    }) as leptos::children::Children
                })
            // **La recherche est ici, et non dans la barre d'onglets.** C'est
            // la place que `BibleTab` lui donne — en haut à droite, et sur la
            // Bible seulement. Arbitré par l'auteur le 29 septembre 2026 :
            // « je veux la même tabbar ».
            action=Some(Box::new(|| view! { <BoutonDeRecherche /> }.into_any()) as leptos::children::Children)
        >
            // **Avant le corpus et détachée de lui** : ce n'est pas une
            // destination de plus, c'est un signet. L'app le range de même,
            // dans sa propre section.
            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    match position.await {
                        Ok(Some(position)) => view! { <CarteDeReprise position /> }.into_any(),
                        // Sans compte, ou sans rien lu encore. Les deux se
                        // taisent : il n'y a rien à reprendre, et le dire
                        // serait un reproche.
                        _ => ().into_any(),
                    }
                })}
            </Suspense>
            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    match plan.await {
                        Ok(ensembles) => view! { <Sommaire ensembles /> }.into_any(),
                        // Le sommaire est analysé au démarrage du serveur : s'il
                        // manque ici, c'est le contexte qui n'a pas été fourni,
                        // pas le corpus qui serait absent. La page se tait
                        // plutôt que d'annoncer un corpus vide.
                        Err(_) => ().into_any(),
                    }
                })}
            </Suspense>
        </PageDeLecture>
    }
}
