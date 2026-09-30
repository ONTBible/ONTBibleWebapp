use leptos::prelude::*;

use crate::interface::design::{fournir_preferences, LesReglages, PageDeLecture};
use crate::interface::tete::Tete;

/// `/fr/compte/lecture` — les réglages de lecture, en page.
///
/// ## Le chemin qui manquait
///
/// Ils ne vivaient que dans la feuille « aA », montée sur un **passage**
/// seulement. Depuis la Bible, le Lexique, Qahal ou Chuqqot, il n'y avait
/// aucun moyen d'y arriver — et la ligne « Réglages de lecture » de l'onglet
/// « Vous » menait à la Bible, c'est-à-dire à un écran sans ce bouton.
///
/// L'auteur l'a trouvé, et sa question disait le soupçon naturel : *« est-ce
/// que c'est parce que je ne suis pas connecté ? »* Non — tout vit dans
/// `localStorage` depuis toujours. Ce qui manquait était un **chemin**, pas un
/// droit, et c'est le pire des deux : un réglage qu'on croit réservé se cherche
/// une fois, puis plus jamais.
///
/// ## Deux chemins, un seul écran
///
/// C'est ce que fait l'app : `YouTab` pousse `ReadingSettingsSheet` comme une
/// destination, **en plus** de la barre d'outils du chapitre. Le site a donc la
/// feuille dans un chapitre — où l'on décide au milieu d'une lecture, sans la
/// quitter — et cette page ailleurs.
///
/// Un seul jeu de bascules les alimente : les dupliquer aurait fait deux
/// vérités à tenir d'accord.
#[component]
pub fn Reglages() -> impl IntoView {
    let preferences = fournir_preferences();

    view! {
        <Tete
            titre="Réglages de lecture"
            description="Thème, fonte, taille, interligne et niveaux du texte — la liseuse \
                         de La Bible ONT se règle sans compte."
            chemin=crate::domaine::chemins::reglages(crate::interface::arbre::arbre_maintenant())
        />
        // Ce n'est pas une page qu'un moteur doit servir : elle ne porte aucun
        // contenu, seulement des interrupteurs, et elle est vide sans
        // JavaScript.
        <leptos_meta::Meta name="robots" content="noindex, follow" />

        // **Sous « Vous », et non sous la Bible.**
        //
        // Elle était à `/fr/webapp/reglages`, donc l'onglet **Bible**
        // s'allumait en y arrivant — on partait de « Vous » et l'on se
        // retrouvait ailleurs. Et la mémoire de l'onglet Bible retenait cette
        // page comme sa dernière place : y revenir ramenait aux réglages au
        // lieu du corpus.
        //
        // C'est la structure de l'app : `YouTab` pousse `ReadingSettingsSheet`
        // comme une destination **de son onglet**. L'adresse le dit maintenant.
        <PageDeLecture
            liste=true
            fil=vec![(crate::domaine::chemins::compte(crate::interface::arbre::arbre_maintenant()), "Vous".to_string())]
            titre="Lecture"
        >
            <div class="max-w-mesure">
                <LesReglages preferences />
            </div>
        </PageDeLecture>
    }
}
