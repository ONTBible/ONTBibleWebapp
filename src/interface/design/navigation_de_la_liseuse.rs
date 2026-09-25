//! La navigation de la liseuse — la chrome de l'app, portée.
//!
//! ## Deux formes pour un même objet, et c'est l'app qui les donne
//!
//! Sur iPhone, une **barre d'onglets en bas** ; sur iPad et Mac, une **barre
//! latérale**. Ce n'est pas un choix de mise en page : c'est la même
//! distinction que `readingWidth` contre `pageWidth`, un étage plus haut.
//!
//! Le site n'avait ni l'une ni l'autre — il portait la navigation d'une
//! **édition** : cinq entrées en capitales espacées, au-dessus de tout, qui
//! disent « voici les pages de ce site ». Juste pour qui découvre l'ONT,
//! inadapté pour qui revient lire.
//!
//! ## Là où elle vaut, et là où elle ne vaut pas
//!
//! Dans la liseuse seulement. Arbitré avec la session macOS, le 21 septembre
//! 2026, et sa réserve est ce qui a fixé la règle :
//!
//! > Ma barre est **toujours là**, et c'est cette constance qui la rend
//! > invisible. Une barre qui apparaît et disparaît selon la section devient au
//! > contraire une chose qu'on **surveille** — on se demande où elle est
//! > passée, donc on la regarde au lieu de regarder le texte.
//!
//! D'où la règle qui en sort, et qui n'était pas dans la question posée :
//!
//! > **Le passage est un acte du lecteur, pas une conséquence de l'URL.**
//!
//! Le site a déjà cet acte, et c'est sa pièce la plus travaillée : le portail
//! de l'accueil est un seuil qu'on franchit. On entre dans la liseuse par la
//! porte ; la barre naît de ce franchissement, et sa venue est lisible.
//!
//! **Le lecteur qui arrive par un lien partagé n'a franchi aucune porte.** Il
//! tombe directement sur `/fr/lire/{livre}/{unité}`, et c'est le cas d'usage
//! qui justifie toute cette route. La barre lui apparaît donc sans qu'il ait
//! rien franchi — et c'est acceptable, parce que pour lui elle n'*apparaît*
//! pas : elle est là depuis le premier pixel de sa première page.
//!
//! ## Trois destinations, et deux absences assumées
//!
//! L'app en porte cinq — Qahal, Bible, Lexique, Chuqqot, Vous — plus
//! « Reprendre » en barre latérale. Le site n'a de pages que pour trois.
//!
//! **Qahal et Chuqqot ne sont pas de la chrome à porter, ce sont des
//! fonctionnalités à écrire.** Les poser en onglets vides ferait exactement ce
//! que ce dépôt s'interdit depuis le badge App Store : *une bannière n'a que
//! deux états justes, et « allumée vers rien » n'en est pas un.*
//!
//! « Reprendre » manque pour une autre raison : la position de lecture existe
//! côté backend et le site ne la suit pas encore (§8 nonies). Elle viendra avec
//! elle, pas avant.

use leptos::prelude::*;

use crate::interface::design::image;

/// Une destination de la liseuse.
struct Destination {
    chemin: &'static str,
    nom: &'static str,
    /// Le symbole, en SVG inline — voir `signe`.
    signe: &'static str,
}

/// Les destinations que le site peut tenir aujourd'hui.
///
/// Dans l'ordre de l'app : ce qui se lit à gauche, ce qui vous appartient à
/// droite. Le compte n'y figure pas — il est **épinglé en bas**, hors du
/// défilement, et `Compte` le rend à part.
const DESTINATIONS: [Destination; 3] = [
    Destination {
        chemin: "/fr/lire",
        nom: "Bible",
        signe: "livre",
    },
    Destination {
        chemin: "/fr/lexique",
        nom: "Lexique",
        signe: "lexique",
    },
    Destination {
        chemin: "/fr/rechercher",
        nom: "Chercher",
        signe: "loupe",
    },
];

/// Le tracé d'un symbole, en coordonnées de `viewBox="0 0 24 24"`.
///
/// **Dessinés ici et non chargés**, contrairement aux images de la marque : ce
/// sont quatre traits, ils changent de couleur avec l'état, et un fichier par
/// symbole coûterait quatre requêtes pour trois cents octets de tracé.
///
/// Ils citent les symboles SF de l'app — `book.closed.fill`,
/// `character.book.closed.fill`, `magnifyingglass`, `person.crop.circle.fill` —
/// sans les recopier : les SF sont sous licence Apple et ne sortent pas d'une
/// app. On en garde la **silhouette**, qui est ce que le lecteur reconnaît.
fn signe(nom: &str) -> &'static str {
    match nom {
        "livre" => "M6 4h11a2 2 0 0 1 2 2v14H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2Zm0 12h13",
        "lexique" => "M6 4h11a2 2 0 0 1 2 2v14H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2Zm4.2 11.5 2.3-6.4 2.3 6.4m-3.8-2h3",
        "loupe" => "M10.5 4a6.5 6.5 0 1 1 0 13 6.5 6.5 0 0 1 0-13Zm5 11.5L20 20",
        _ => "M12 3a9 9 0 1 1 0 18 9 9 0 0 1 0-18Zm0 4.5a3 3 0 1 1 0 6 3 3 0 0 1 0-6Zm-6.2 10a7.4 7.4 0 0 1 12.4 0",
    }
}

/// Un symbole, à la taille d'une ligne.
#[component]
fn Signe(#[prop(into)] nom: String) -> impl IntoView {
    view! {
        <svg
            aria-hidden="true"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.6"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="size-[1.35em] shrink-0"
        >
            <path d=signe(&nom) />
        </svg>
    }
}

/// La navigation de la liseuse, dans ses deux formes.
///
/// Une seule déclaration rend les deux : la même liste, deux habillages, choisis
/// par une requête média. Deux composants auraient deux listes à tenir
/// d'accord — et c'est la forme de défaut que ce dépôt a payée le plus souvent.
#[component]
pub fn NavigationDeLaLiseuse(
    /// Le chemin courant, pour désigner la destination où l'on est.
    #[prop(into)]
    chemin: Signal<String>,
) -> impl IntoView {
    view! {
        <BarreLaterale chemin />
        <BarreDOnglets chemin />
    }
}

/// Vrai quand ce chemin est dans cette destination.
///
/// Le préfixe suivi d'une barre, ou le chemin exact — la même règle que
/// `c_est_la_liseuse`, et pour la même raison : `/fr/lirent-ils` commencerait
/// par `/fr/lire`.
fn on_y_est(chemin: &str, destination: &str) -> bool {
    let chemin = chemin.trim_end_matches('/');
    chemin == destination || chemin.starts_with(&format!("{destination}/"))
}

/// La barre latérale — au-delà de `lg`.
///
/// **264 px, la largeur idéale du Mac** (`RacineMac.largeurDeBarreParDefaut`).
/// Elle est en `rem` et non en pixels : sur le web, la taille de police par
/// défaut du lecteur joue le rôle du facteur d'interface, et une barre figée
/// tronquerait « Toledot Adam ve-Chavah » au cran suivant — le défaut que le
/// Mac a payé et corrigé.
#[component]
fn BarreLaterale(chemin: Signal<String>) -> impl IntoView {
    view! {
        <nav
            aria-label="La liseuse"
            class="fixed inset-y-0 start-0 z-40 hidden w-[16.5rem] flex-col border-e border-filet bg-surface/60 px-3 py-5 backdrop-blur-sm lg:flex"
        >
            // La marque en tête, petite : on est dans la liseuse, elle rappelle
            // où l'on est sans se proclamer. Elle mène à l'édition — c'est la
            // porte dans l'autre sens.
            <a href="/fr" class="mb-6 block px-3 no-underline">
                <img
                    src=image("wordmark.svg")
                    alt="La Bible ONT"
                    class="w-36 opacity-80 transition-opacity hover:opacity-100"
                />
            </a>

            <ul class="m-0 flex list-none flex-col gap-0.5 p-0">
                {DESTINATIONS
                    .iter()
                    .map(|destination| {
                        let ici = destination.chemin;
                        let actif = Signal::derive(move || on_y_est(&chemin.get(), ici));
                        view! {
                            <li>
                                <a
                                    href=destination.chemin
                                    aria-current=move || actif.get().then_some("page")
                                    // **La capsule choisie est en accent, pas
                                    // en aubergine.**
                                    //
                                    // Le premier jet la peignait `bg-aubergine`
                                    // — la marque, qui ne suit aucun thème par
                                    // construction. Sur une nuit elle se lisait ;
                                    // sur du parchemin, une capsule aubergine
                                    // sous un libellé doré **disparaissait**.
                                    //
                                    // C'est le défaut du massif sur l'accueil,
                                    // un étage plus bas : une couleur de marque
                                    // posée sur un fond qui, lui, change. Ici on
                                    // n'a pas le choix de borner — la barre est
                                    // *dans* la liseuse. Elle prend donc l'accent
                                    // du thème, qui change avec lui.
                                    class="flex items-center gap-3 rounded-full px-3 py-2 font-titre text-sm uppercase tracking-capitales no-underline transition-colors"
                                    class=("bg-accent/15", move || actif.get())
                                    class=("ring-1", move || actif.get())
                                    class=("ring-accent/30", move || actif.get())
                                    class=("text-accent", move || actif.get())
                                    class=("text-encre-douce", move || !actif.get())
                                    class=("hover:bg-accent/8", move || !actif.get())
                                    class=("hover:text-encre", move || !actif.get())
                                >
                                    <Signe nom=destination.signe />
                                    {destination.nom}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>

            // **Le compte va en bas, épinglé, hors du défilement.**
            //
            // C'est la place qu'Apple Music lui donne, et la session macOS a
            // donné la raison : *le compte n'est pas une destination parmi les
            // livres, c'est qui regarde.* Sur un corpus complet, une dernière
            // section le mettrait à soixante-dix livres de là.
            <div class="mt-auto pt-6">
                <a
                    href="/fr/compte"
                    class="flex items-center gap-3 rounded-full border border-filet px-3 py-2 font-titre text-sm uppercase tracking-capitales text-encre-douce no-underline transition-colors hover:border-or/50 hover:text-encre"
                >
                    <Signe nom="compte" />
                    "Vous"
                </a>
            </div>
        </nav>
    }
}

/// La barre d'onglets — en dessous de `lg`.
///
/// Elle flotte en bas, comme celle de l'app, et pour la même raison que le
/// bouton « aA » : c'est là qu'arrive le pouce.
///
/// `env(safe-area-inset-bottom)` n'est pas une politesse — sans lui, la barre
/// se pose sur la barre d'accueil d'un iPhone, où le geste de retour prend le
/// toucher en premier.
#[component]
fn BarreDOnglets(chemin: Signal<String>) -> impl IntoView {
    view! {
        <nav
            aria-label="La liseuse"
            class="barre-d-onglets fixed inset-x-0 bottom-0 z-40 border-t border-filet bg-surface/80 backdrop-blur-md lg:hidden"
            style="padding-bottom: env(safe-area-inset-bottom)"
        >
            <ul class="m-0 flex list-none items-stretch justify-around p-0">
                {DESTINATIONS
                    .iter()
                    .chain(std::iter::once(&COMPTE))
                    .map(|destination| {
                        let ici = destination.chemin;
                        let actif = Signal::derive(move || on_y_est(&chemin.get(), ici));
                        view! {
                            <li class="flex-1">
                                <a
                                    href=destination.chemin
                                    aria-current=move || actif.get().then_some("page")
                                    // Les libellés sont **sous** les symboles et
                                    // en très petit, comme chez l'app : un
                                    // symbole seul se devine mal, un libellé
                                    // seul prend toute la place.
                                    class="flex flex-col items-center gap-1 px-1 py-2.5 text-[0.7rem] uppercase tracking-[0.1em] no-underline transition-colors"
                                    class=("text-accent", move || actif.get())
                                    class=("text-encre-douce", move || !actif.get())
                                >
                                    <Signe nom=destination.signe />
                                    {destination.nom}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </nav>
    }
}

/// Le compte, qui est une destination de la barre d'onglets et un épinglé de la
/// barre latérale.
///
/// Déclaré à part parce que sa **place** diffère selon la forme — et c'est
/// justement ce que la barre latérale dit de lui.
const COMPTE: Destination = Destination {
    chemin: "/fr/compte",
    nom: "Vous",
    signe: "compte",
};

#[cfg(all(test, feature = "ssr"))]
mod epreuves {
    use super::{on_y_est, COMPTE, DESTINATIONS};

    /// Chaque destination mène à une route qui existe.
    ///
    /// Sans ça, une barre de navigation mène à un 404 — et une barre est la
    /// pièce qu'on touche sans regarder.
    #[test]
    fn chaque_destination_a_sa_route() {
        let app = include_str!("../app.rs");
        for destination in DESTINATIONS.iter().chain(std::iter::once(&COMPTE)) {
            // `/fr/lire` se déclare `(StaticSegment("fr"), StaticSegment("lire"))`.
            let segments: Vec<&str> = destination
                .chemin
                .trim_start_matches('/')
                .split('/')
                .collect();
            let attendu = segments
                .iter()
                .map(|s| format!("StaticSegment(\"{s}\")"))
                .collect::<Vec<_>>()
                .join(", ");
            assert!(
                app.contains(&attendu),
                "la navigation mène à {} — aucune route ne le déclare ({attendu})",
                destination.chemin
            );
        }
    }

    #[test]
    fn un_prefixe_ne_deborde_pas() {
        assert!(on_y_est("/fr/lire/bereshit", "/fr/lire"));
        assert!(on_y_est("/fr/lire", "/fr/lire"));
        assert!(!on_y_est("/fr/lirent-ils", "/fr/lire"));
        assert!(!on_y_est("/fr/lexique", "/fr/lire"));
    }
}
