use leptos::prelude::*;

use crate::interface::design::PageDeLecture;
use crate::interface::tete::Tete;

/// `/fr/chuqqot` — חֻקּוֹת, ce qui est gravé et qui demeure.
///
/// ## Le nom, et pourquoi ce n'est pas « Khuqqot »
///
/// Deux pluriels existent et ne disent pas la même chose : חֹק → חֻקִּים,
/// masculin, tire vers la **prescription** — ce qu'on ordonne de faire ;
/// חֻקָּה → חֻקּוֹת, féminin, tire vers la **disposition permanente** — ce qui
/// est établi et tient. Ce que cet onglet portera n'oblige personne à agir :
/// ce sont des régularités énoncées comme des nécessités.
///
/// Et l'initiale est `ch`, jamais `kh` : le §2.9 du vault fixe que **ח**
/// (*het*) se rend `ch`, `kh` étant réservé à **כ** (*khaf*). חֻקָּה commence
/// par un het — la même correction que `Khanokh → Chanokh`.
///
/// ## Pourquoi l'onglet existe alors qu'il ne montre rien
///
/// Ce dépôt l'écartait, et l'argument avait l'air solide : *une bannière n'a
/// que deux états justes, et « allumée vers rien » n'en est pas un.* Il tombe
/// sur un fait qu'il fallait aller chercher — **l'app a l'onglet et montre un
/// écran d'attente**. Ce n'est donc pas une branche qu'aucun état du site ne
/// rend : c'est l'état d'aujourd'hui, visible, et le seul qu'on puisse
/// éprouver.
///
/// L'auteur a tranché le 29 septembre 2026 : « je veux la même tabbar ».
#[component]
pub fn Chuqqot() -> impl IntoView {
    view! {
        <Tete
            titre="Chuqqot — ce qui est gravé et demeure"
            description="Les chuqqot de La Bible ONT — les régularités de l'ontologie \
                         hébraïque, énoncées comme des nécessités."
            chemin=crate::domaine::chemins::chuqqot(crate::interface::arbre::arbre_maintenant())
        />

        <PageDeLecture liste=true titre="Chuqqot">
            <EnAttenteDeValidation />
        </PageDeLecture>
    }
}

/// L'état vide — et il doit dire la **vérité**.
///
/// ## Le texte d'avant était faux, pas seulement vide
///
/// L'app a payé exactement ce défaut : son écran disait « ils ne sont pas
/// encore écrits » alors que sept l'étaient, dans les brouillons du vault, et
/// que le pipeline en produisait déjà le fichier. *« Le défaut n'était pas
/// l'écran vide, c'était l'écran qui mentait. »*
///
/// **Ce dépôt allait refaire la même faute** : j'avais relevé `entries: []` et
/// conclu que le vault n'avait rien écrit. Il y en a **quatorze**. Un compteur
/// à zéro ne dit pas pourquoi il est à zéro.
///
/// ## Il ne compte pas les chuqqot en attente
///
/// « Quatorze sont écrites » se périmerait à la première validation, et
/// personne ne vient corriger un état vide — c'est l'écran qu'on regarde le
/// moins. Le compte exact vit là où il se recalcule seul, `dist/report.md`.
///
/// ## Et il ne montre pas les brouillons
///
/// La tentation est réelle : les textes existent, on pourrait les marquer
/// « brouillon » comme un chapitre en cours. La décision de l'auteur du
/// 9 septembre 2026 dit l'inverse, et sa raison tient en une phrase — **un
/// énoncé permanent « en attente de validation » se contredit lui-même.** Un
/// chapitre qui s'améliore sous les yeux du lecteur reste honnête ; une
/// chuqqah, non.
///
/// La garde vit dans le pipeline : le site n'a rien à filtrer, et surtout rien
/// à contourner en allant chercher les brouillons par un autre chemin.
#[component]
fn EnAttenteDeValidation() -> impl IntoView {
    view! {
        // **En haut de la page, jamais centré.** Centré verticalement, un état
        // vide se lit comme une alerte au milieu d'un écran — alors qu'il
        // décrit une situation parfaitement normale. C'est la note de l'app,
        // et elle vaut ici où `Bloc` centre son contenu par défaut.
        <section class="max-w-mesure">
            <span aria-hidden="true" class="mb-4 block text-marque-encre">
                <svg
                aria-hidden="true"
                viewBox="0 0 256 256"
                fill="currentColor"
                class="size-8"
            >
                <path d=crate::interface::design::symboles::trace("strates", false) />
            </svg>
            </span>
            <h2 class="m-0 mb-3 font-titre text-[1.15em] font-semibold text-encre-vive">
                "Rien à lire pour l'instant"
            </h2>
            <p class="m-0 mb-4 text-encre-douce">
                "Les chuqqot sont écrites et attendent leur validation. "
                "Elles paraîtront ici une à une, dès qu'elles seront arrêtées."
            </p>
            <p class="m-0 text-sm text-encre-douce/80">
                "Un énoncé permanent ne se publie pas « en attente » : "
                "il se contredirait lui-même."
            </p>
        </section>
    }
}
