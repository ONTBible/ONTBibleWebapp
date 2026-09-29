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

    view! {
        <Tete
            // Même règle qu'au lexique : « Lire » nomme une action dans une
            // navigation, il ne nomme pas un contenu pour un moteur.
            titre="Lire le corpus hébreu et araméen"
            description="Le corpus de La Bible ONT — les soixante-dix livres du Kenesset et \
                         de la Berit Hadashah, et l'état de leur restitution."
            chemin="/fr/webapp"
        />

        // **Ni œil-de-bœuf ni chapeau.** Ils y étaient — « Le corpus », puis
        // « Le plan entier, et ce qui en est traduit » — et ils faisaient de ce
        // sommaire une page d'édition. L'app ouvre sa Bible sur un titre et
        // rien d'autre : une liste ne s'introduit pas, on y revient.
        //
        // Ce que le chapeau disait n'est pas perdu : « les titres se lisent,
        // les autres attendent » est maintenant dit par la **forme** — une
        // ligne sans chevron ne se touche pas.
        // **Le titre est celui de l'app**, pas le mot de la navigation.
        // « Lire » nomme une action — juste dans une barre, insuffisant en tête
        // d'écran, où il faut dire *ce qu'on ouvre*. L'app dit « La Bible ONT »,
        // et c'est ce que le lecteur retrouve.
        <PageDeLecture
            liste=true
            titre="La Bible ONT"
            // **La recherche est ici, et non dans la barre d'onglets.** C'est
            // la place que `BibleTab` lui donne — en haut à droite, et sur la
            // Bible seulement. Arbitré par l'auteur le 29 septembre 2026 :
            // « je veux la même tabbar ».
            action=Box::new(|| view! { <BoutonDeRecherche /> }.into_any())
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
