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
use leptos_router::components::A;

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
            class="signe size-[1.6em] shrink-0"
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
            class="verre fixed inset-y-0 start-0 z-40 hidden w-[16.5rem] flex-col border-e border-filet px-3 py-5 lg:flex"
        >
            // La marque en tête, petite : on est dans la liseuse, elle rappelle
            // où l'on est sans se proclamer. Elle mène à l'édition — c'est la
            // porte dans l'autre sens.
            <A href="/fr" attr:class="mb-6 block px-3 no-underline">
                <img
                    src=image("wordmark.svg")
                    alt="La Bible ONT"
                    class="w-36 opacity-80 transition-opacity hover:opacity-100"
                />
            </A>

            <ul class="m-0 flex list-none flex-col gap-0.5 p-0">
                {DESTINATIONS
                    .iter()
                    .map(|destination| {
                        let ici = destination.chemin;
                        let actif = Signal::derive(move || on_y_est(&chemin.get(), ici));
                        view! {
                            <li>
                                // **La capsule choisie est en accent, pas en
                                // aubergine.** L'aubergine est la marque, qui
                                // ne suit aucun thème par construction : sur
                                // une nuit la capsule se lisait, sur du
                                // parchemin elle disparaissait sous son libellé
                                // doré. C'est le défaut du massif de l'accueil,
                                // un étage plus bas.
                                //
                                // **La classe se calcule en une fois**, et non
                                // par une suite de `class=(…)` : passés à un
                                // composant plutôt qu'à une balise, ces
                                // conditionnels ne s'appliquent pas — ils ne
                                // produisent aucune erreur, seulement une
                                // capsule qui ne se peint jamais.
                                <A
                                    href=destination.chemin
                                    attr:aria-current=move || actif.get().then_some("page")
                                    attr:class=move || {
                                        let base = "flex items-center gap-3 rounded-full px-3 py-2 \
                                                    font-titre text-sm uppercase tracking-capitales \
                                                    no-underline transition-colors";
                                        if actif.get() {
                                            format!("{base} bg-accent/15 ring-1 ring-accent/30 text-accent")
                                        } else {
                                            format!("{base} text-encre-douce hover:bg-accent/8 hover:text-encre")
                                        }
                                    }
                                >
                                    <Signe nom=destination.signe />
                                    {destination.nom}
                                </A>
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
                <A
                    href="/fr/compte"
                    attr:class="flex items-center gap-3 rounded-full border border-filet px-3 py-2 font-titre text-sm uppercase tracking-capitales text-encre-douce no-underline transition-colors hover:border-or/50 hover:text-encre"
                >
                    <Signe nom="compte" />
                    "Vous"
                </A>
            </div>
        </nav>
    }
}

/// La barre d'onglets — en dessous de `lg`.
///
/// ## Une capsule qui flotte, pas une bande collée
///
/// Le premier jet était une bande pleine largeur, bord à bord, comme les
/// barres d'onglets d'avant. Relevé sur la capture de l'app : c'est une
/// **capsule**, encartée de chaque côté, posée au-dessus du contenu, avec un
/// filet très fin et une ombre basse.
///
/// La différence n'est pas cosmétique. Une bande bord à bord **ferme** la page :
/// elle dit que le contenu s'arrête là. Une capsule flotte **sur** le contenu,
/// qui continue de courir dessous — ce qui est vrai, puisqu'on défile encore.
///
/// ## L'onglet choisi porte sa propre capsule
///
/// Et c'est ce qui manquait le plus : la couleur seule ne suffit pas à dire
/// « vous êtes ici » sur cinq entrées de même poids. L'app pose un fond
/// arrondi sous l'onglet actif, et met son symbole **et** son libellé dans
/// l'encre de la marque.
///
/// C'est la règle de l'accentuation, une pièce plus loin : *semi-gras **et**
/// coloré — la couleur seule ne suffit pas, un lecteur daltonien ne verrait
/// rien.* Ici c'est le fond qui tient le rôle du semi-gras.
///
/// ## `env(safe-area-inset-bottom)` n'est pas une politesse
///
/// Sans lui, la capsule se pose sur la barre d'accueil d'un iPhone, où le
/// geste de retour prend le toucher en premier.
#[component]
fn BarreDOnglets(chemin: Signal<String>) -> impl IntoView {
    view! {
        <nav
            aria-label="La liseuse"
            class="barre-d-onglets pointer-events-none fixed inset-x-0 bottom-0 z-40 flex justify-center px-4 pb-3 lg:hidden"
            style="padding-bottom: calc(0.75rem + env(safe-area-inset-bottom))"
        >
            <ul class="verre pointer-events-auto m-0 flex w-full max-w-md list-none items-stretch justify-around gap-1 rounded-full border border-filet/50 p-1">
                {DESTINATIONS
                    .iter()
                    .chain(std::iter::once(&COMPTE))
                    .map(|destination| {
                        let ici = destination.chemin;
                        let actif = Signal::derive(move || on_y_est(&chemin.get(), ici));
                        view! {
                            <li class="flex-1">
                                <A
                                    href=destination.chemin
                                    attr:aria-current=move || actif.get().then_some("page")
                                    // Le libellé est **sous** le symbole et en
                                    // très petit, comme chez l'app : un symbole
                                    // seul se devine mal, un libellé seul prend
                                    // toute la place.
                                    attr:class=move || {
                                        // **En casse normale, pas en capitales
                                        // espacées.** Les capitales sont la
                                        // voix du site — les œils-de-bœuf, les
                                        // titres de section. L'app écrit
                                        // « Bible », « Lexique », et c'est ce
                                        // qu'un lecteur reconnaît d'un
                                        // appareil à l'autre.
                                        //
                                        // Et à cette taille elles coûtent : une
                                        // capitale espacée se lit moins vite
                                        // qu'un bas de casse, et cette barre se
                                        // lit du coin de l'œil.
                                        // `onglet` porte la bascule — voir la
                                        // feuille. Pas de `transition-colors`
                                        // de Tailwind : il poserait sa propre
                                        // durée et sa propre courbe par-dessus
                                        // celles du ressort.
                                        let base = "onglet flex flex-col items-center gap-1 \
                                                    rounded-full px-1 py-1.5 font-titre \
                                                    text-[0.72rem] leading-none no-underline";
                                        if actif.get() {
                                            format!("{base} bg-encre/10 text-marque-encre")
                                        } else {
                                            format!("{base} text-encre-douce")
                                        }
                                    }
                                >
                                    <Signe nom=destination.signe />
                                    {destination.nom}
                                </A>
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

/// L'ouverture — la montagne qu'un trait de lumière révèle.
///
/// ## Elle est rendue par le serveur, et retirée avant d'être peinte
///
/// Le réflexe est de la monter côté navigateur, après avoir vérifié qu'on ne
/// l'a pas déjà vue. Ça ne peut pas marcher : la page s'afficherait d'abord,
/// puis l'ouverture la recouvrirait — une ouverture qui arrive après ce qu'elle
/// ouvre.
///
/// Elle est donc **toujours** dans le HTML du serveur, et le script de
/// l'en-tête la retire immédiatement quand la session l'a déjà vue. Même
/// mécanique que la peau, et pour la même raison : ce qui doit être vrai avant
/// la première peinture ne peut pas attendre l'hydratation.
///
/// ## Une fois par session, pas une fois par visite
///
/// `sessionStorage` et non `localStorage` : l'ouverture dit qu'on entre. Elle a
/// sa place quand on ouvre un onglet, pas quand on revient de la page d'à côté
/// — et elle doit revenir le lendemain, faute de quoi elle n'existerait que
/// pour les lecteurs qui vident leur cache.
///
/// ## Elle ne retarde rien
///
/// Elle est `fixed` par-dessus : la page est rendue, lisible et indexable
/// dessous pendant qu'elle joue. Un moteur de recherche ne la voit pas, et un
/// lecteur qui la traverse trouve la page déjà là.
#[component]
pub fn Ouverture() -> impl IntoView {
    view! {
        <div class="ouverture" aria-label="La Bible ONT" role="img">
            <div class="ouverture-marque">
                // La montagne dans la pénombre — elle est là depuis le premier
                // instant, elle ne s'allume pas. C'est ce qui fait qu'on lit un
                // objet révélé plutôt qu'une forme qui apparaît.
                <div class="ouverture-sombre"></div>
                <div class="ouverture-montee"></div>
                // La partie éclairée : une bande qui traverse, masquée par la
                // même montagne. Sa lueur est son propre `drop-shadow`, donc la
                // lumière quitte la silhouette sans jamais la dupliquer.
                <div class="ouverture-lumiere">
                    <div class="ouverture-bande"></div>
                </div>
            </div>
        </div>
    }
}

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
