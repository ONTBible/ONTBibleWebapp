//! La navigation de la liseuse — la chrome de l'app, portée.
//!
//! ## La barre ne parle pas en capitales, et elle le disait déjà à moitié
//!
//! Les capitales espacées sont la voix **du site** — les œils-de-bœuf, les
//! titres de section, le fil d'Ariane. La barre d'onglets en était déjà
//! sortie, avec son argument : *« l'app écrit “Bible”, “Lexique”, et c'est ce
//! qu'un lecteur reconnaît d'un appareil à l'autre »*, plus le coût de lecture
//! d'une capitale espacée dans une barre qu'on lit du coin de l'œil.
//!
//! **La barre latérale, elle, était restée en capitales.** Même liste, même
//! fichier, deux casses — et ce n'était pas une décision, c'était un reste.
//! Relevé par la session macOS sur une capture, pas dans le code : les deux
//! classes vivent à quatre cents lignes l'une de l'autre.
//!
//! L'app a d'ailleurs tranché **deux fois**, et la seconde coûte : le style
//! `sidebar` de macOS compose ses en-têtes en capitales, et
//! `BarreLateraleONT.swift` pose `.textCase(nil)` **pour l'en désactiver** —
//! son commentaire dit pourquoi, *« c'était un vrai écart avec l'iPad, où
//! l'en-tête lit “Kenesset” et non “KENESSET” »*. Le web n'avait pas à lutter
//! pour l'obtenir ; il n'a pas non plus de raison d'y renoncer.
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
//! tombe directement sur `/fr/webapp/{livre}/{unité}`, et c'est le cas d'usage
//! qui justifie toute cette route. La barre lui apparaît donc sans qu'il ait
//! rien franchi — et c'est acceptable, parce que pour lui elle n'*apparaît*
//! pas : elle est là depuis le premier pixel de sa première page.
//!
//! ## Quatre destinations, et une absence qui tient
//!
//! L'app en porte cinq — Qahal, Bible, Lexique, Chuqqot, Vous — plus
//! « Reprendre » en barre latérale.
//!
//! Ce paragraphe écartait **Qahal et Chuqqot** ensemble, comme « des
//! fonctionnalités à écrire, et non de la chrome à porter ». C'était juste
//! pour l'une et faux pour l'autre, et le partage se mesure en ouvrant
//! `dist/` plutôt qu'en raisonnant :
//!
//! - **Qahal** n'attend rien. L'app le dit elle-même — *« structure posée,
//!   sans serveur »* : le verset du jour, que le site porte déjà et par la
//!   même fonction de la date, plus l'annonce de ce qui vient. Tout ce qui
//!   suppose d'autres lecteurs y est **annoncé sans être simulé** ;
//! - **Chuqqot attend une validation, pas une rédaction.** Le pipeline émet
//!   son fichier même vide — *« un fichier absent et un fichier sans entrée
//!   ne se distinguent pas côté liseuse, et l'un des deux voudrait dire le
//!   réseau a échoué »* — et il est vide aujourd'hui. Mais **quatorze chuqqot
//!   sont écrites**, retenues dans les brouillons du vault par une règle
//!   arbitrée le 9 septembre 2026 : une unité de traduction voyage marquée
//!   « Brouillon », un **énoncé permanent** « en attente de validation » se
//!   contredit lui-même.
//!
//!   L'onglet serait donc aujourd'hui ce que ce dépôt s'interdit depuis le
//!   badge App Store — *une bannière n'a que deux états justes, et « allumée
//!   vers rien » n'en est pas un* — et il cessera de l'être **d'un coup**, le
//!   jour où l'auteur valide : les quatorze passent en `locked/`, le fichier
//!   se remplit, et le vault n'écrit pas une ligne de plus.
//!
//!   C'est la session du Vault qui l'a corrigé, et l'écart comptait : j'avais
//!   conclu « rien n'est écrit » d'un tableau vide. **Un compteur à zéro ne
//!   dit pas pourquoi il est à zéro** — rien d'écrit et tout d'écrit-mais-
//!   retenu donnent le même chiffre, et n'appellent pas la même préparation.
//!
//! La leçon est celle du §8 quinquies, prise à l'envers : on s'était
//! transmis une contrainte sans la redater.
//!
//! **« Reprendre » y est**, en barre latérale comme chez l'app — et ce
//! paragraphe disait le contraire, au motif que « le site ne suit pas encore
//! la position de lecture ». Il la suit : `api::retenir_la_position` est
//! appelée à l'ouverture de chaque unité depuis que le compte existe, et
//! `api::ma_position` la relit. C'était vrai à l'écriture du §8 nonies, et
//! c'est resté écrit après avoir cessé de l'être.
//!
//! Elle n'est pas une destination de plus : l'app le dit d'un mot — *« les
//! trois suivantes sont des lieux ; celle-ci est un signet »* —, et c'est ce
//! qui lui donne sa section à elle, au-dessus des autres.
//!
//! ## Et la barre latérale porte le corpus
//!
//! Sous les destinations : un rayon par corpus peuplé, dépliable, avec les
//! livres qui ont du texte. Ce n'est **pas** un emprunt à la barre dessinée du
//! Mac — que la session macOS déconseille de copier, et à raison, ses trois
//! contournements n'existant pas sur le web. Son propre commentaire dit que
//! l'iPad montre la même chose : *« Sur l'iPad il est toujours visible »*. Le
//! contournement était le **dessin**, pas le contenu.
//!
//! Le pli passe par `<details>`, qui est la réponse du web à
//! `Section(isExpanded:)` : il replie sans JavaScript, il est au clavier, et
//! il porte son témoin. Le Mac a dû dessiner le sien à la main parce que le
//! style `sidebar` n'en affiche aucun — mesuré chez lui, la section se
//! repliait sans que rien ne dise qu'elle le pouvait.

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
const DESTINATIONS: [Destination; 4] = [
    // **Le Qahal ouvre**, comme chez l'app. L'ordre n'est pas alphabétique et
    // n'est pas une préférence : la Kenesset est le rassemblement des textes,
    // le Qahal celui des lecteurs, et c'est par le second qu'on arrive.
    Destination {
        chemin: "/fr/qahal",
        nom: "Qahal",
        signe: "qahal",
    },
    Destination {
        chemin: "/fr/webapp",
        nom: "Bible",
        signe: "livre",
    },
    Destination {
        chemin: "/fr/lexique",
        nom: "Lexique",
        signe: "lexique",
    },
    // **Chuqqot est là, et son écran dit qu'il attend.**
    //
    // Ce dépôt l'écartait au motif qu'« une bannière n'a que deux états
    // justes, et allumée vers rien n'en est pas un ». L'argument tombe sur un
    // fait qu'il fallait aller chercher : **l'app a l'onglet et montre un
    // écran d'attente**. Ce n'est donc pas une branche qu'aucun état du site
    // ne rend — c'est l'état d'aujourd'hui, visible et éprouvable.
    Destination {
        chemin: "/fr/chuqqot",
        nom: "Chuqqot",
        signe: "strates",
    },
];

// **La recherche n'est pas une destination, et c'est l'app qui le dit.**
//
// Elle était le cinquième onglet ici, dans le créneau que Chuqqot laissait
// libre. `BibleTab` la pose ailleurs :
//
//     ToolbarItem(placement: ONTPlacement.principale) {
//         Button("Rechercher", systemImage: "magnifyingglass") { … }
//     }
//
// En haut à droite, **et sur la Bible seulement** — pas au Lexique, qui a son
// propre rail de lettres et ses segments.
//
// L'arbitrage avait été pris dans l'autre sens ici, sur l'argument de la
// session iOS : sa barre de navigation est une contrainte de plateforme
// (cinq onglets, Chuqqot prend le cinquième), donc le site, qui n'a pas la
// contrainte, pouvait garder son onglet. **L'auteur a tranché autrement** le
// 29 septembre 2026 : « je veux la même tabbar ». Une contrainte qui produit
// le bon dessin reste le bon dessin.

/// Un symbole, à la taille d'une ligne.
#[component]
fn Signe(
    #[prop(into)] nom: String,
    /// Le plein plutôt que le contour.
    ///
    /// ## Toutes les destinations le portent, actives ou non
    ///
    /// C'était le premier réflexe de le réserver à l'onglet courant — un
    /// contour qui se remplit dit l'état sans couleur, ce qu'un lecteur
    /// daltonien peut lire. Mis côte à côte avec l'app, c'est faux : **elle
    /// emploie `.fill` sur les cinq**, et ne distingue l'actif que par sa
    /// capsule et son encre de marque.
    ///
    /// Ça se tient, et le site le tenait déjà sans le savoir : *« ici c'est le
    /// fond qui tient le rôle du semi-gras »*. La capsule n'est pas une
    /// couleur, c'est une forme — elle reste lisible sans distinguer les
    /// teintes.
    ///
    /// Le prop reste, et il sert : le livre d'un rayon est en **contour**,
    /// parce qu'il n'est pas une destination de la barre mais une entrée de
    /// liste, et cinq livres pleins feraient une colonne de taches.
    ///
    /// Le site ne pouvait rien de tout ça tant qu'il dessinait ses symboles à
    /// la main : un contour et son plein sont deux dessins, pas un réglage.
    #[prop(optional, into)]
    plein: Signal<bool>,
) -> impl IntoView {
    view! {
        // **Un aplat sur une grille de 256**, et non un trait sur 24. C'est la
        // façon dont les SF Symbols sont faits, et Phosphor les suit : un
        // symbole est une silhouette, pas un fil de fer. Les tracés dessinés à
        // la main ici étaient des traits de 1,6 — plus maigres que ceux de
        // l'app, et chacun avec sa propre épaisseur.
        <svg
            aria-hidden="true"
            viewBox="0 0 256 256"
            fill="currentColor"
            class="signe size-[1.6em] shrink-0"
        >
            <path d=move || crate::interface::design::symboles::trace(&nom, plein.get()) />
        </svg>
    }
}

/// La navigation de la liseuse, dans ses deux formes.
///
/// Une seule déclaration rend les deux : la même liste, deux habillages, choisis
/// par une requête média. Deux composants auraient deux listes à tenir
/// d'accord — et c'est la forme de défaut que ce dépôt a payée le plus souvent.
#[component]
pub fn NavigationDeLaLiseuse() -> impl IntoView {
    view! {
        <BarreLaterale />
        <BarreDOnglets />
    }
}

/// Vrai quand ce chemin est dans cette destination.
///
/// Le préfixe suivi d'une barre, ou le chemin exact — la même règle que
/// `c_est_la_liseuse`, et pour la même raison : `/fr/webappnt-ils` commencerait
/// par `/fr/webapp`.
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
fn BarreLaterale() -> impl IntoView {
    // **Le chemin se lit ici, il ne se reçoit pas.**
    //
    // Il arrivait en prop, depuis `PageDeLecture`. Avec `#[prop(into)]`, la
    // conversion crée un signal **possédé par la portée de l'appelant** — donc
    // par la page. À la navigation, la page est détruite, et les closures
    // d'attribut de la barre le relisent : panic, WASM mort, plus un onglet ne
    // répond.
    //
    // C'est ce que l'auteur a vu — « au bout d'un moment la nav de la tabbar se
    // fige » —, et « au bout d'un moment » voulait dire *à la quatrième
    // navigation*. Le banc l'a nommé en une ligne : *« you tried to access a
    // reactive value … but it has already been disposed »*, défini à
    // `into_reactive_value.rs:17`.
    //
    // `use_location()` rend le mémo du **routeur**, qui vit aussi longtemps que
    // l'application. Rien n'est créé, donc rien ne peut être détruit.
    //
    // > Un signal traverse mal une frontière de composant quand les deux n'ont
    // > pas la même durée de vie. La barre survit aux pages ; son chemin doit
    // > venir de ce qui survit aussi.
    let chemin = leptos_router::hooks::use_location().pathname;
    // Chaque navigation dépose la place courante sous son onglet.
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| retenir_la_place(&chemin.get()));
    // **Deux ressources et non une.** Le plan est le même pour tout le monde
    // et se met en cache au bord ; la position appartient au lecteur et ne
    // doit jamais y entrer. Les fondre rendrait le plan incachable pour gagner
    // un aller-retour.
    //
    // Ni l'une ni l'autre n'est `blocking` : la barre n'est pas ce qu'on vient
    // chercher, et retarder le premier octet du texte pour peindre une
    // navigation inverserait les priorités.
    let plan = Resource::new(|| (), |_| async { crate::api::sommaire().await });
    let position = Resource::new(|| (), |_| async { crate::api::ma_position().await });

    view! {
        <nav
            aria-label="La liseuse"
            class="habillage-app verre fixed inset-y-0 start-0 z-40 hidden w-[16.5rem] flex-col border-e border-filet px-3 py-5 lg:flex"
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

            // **Le signet, avant les lieux.** Il se tait sans compte et sans
            // rien de lu — il n'y a alors rien à reprendre, et le dire serait
            // un reproche.
            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    match position.await {
                        Ok(Some(p)) => {
                            let ou = format!("{}:{}", p.chapter_title, p.verse);
                            view! {
                                <A
                                    href=format!(
                                        "/fr/webapp/{}/{}?v={}",
                                        p.book_id,
                                        p.chapter_id,
                                        p.verse,
                                    )
                                    attr:class="presse--ligne survol mb-3 flex items-center gap-3 rounded-full px-3 py-2 font-titre text-sm text-encre-douce no-underline hover:text-encre"
                                >
                                    <Signe nom="signet" plein=true />
                                    <span class="flex-1 truncate">"Reprendre"</span>
                                    <span class="chiffres-tableau text-[0.7rem] opacity-70">
                                        {ou}
                                    </span>
                                </A>
                            }
                                .into_any()
                        }
                        _ => ().into_any(),
                    }
                })}
            </Suspense>

            <ul class="m-0 flex list-none flex-col gap-0.5 p-0">
                {DESTINATIONS
                    .iter()
                    .map(|destination| {
                        let ici = destination.chemin;
                        // **Pas de `Signal::derive` ici**, et ça a coûté la
                        // navigation entière.
                        //
                        // Il en portait un, créé **dans la boucle de rendu** :
                        // il appartenait donc à la portée réactive du moment.
                        // À la navigation, cette portée est détruite — et les
                        // closures d'attribut, elles, sont réévaluées. Elles
                        // lisaient alors un signal disposé :
                        //
                        //     panicked at reactive_graph/traits.rs:394
                        //     you tried to access a reactive value … but it
                        //     has already been disposed
                        //     RuntimeError: Unreachable code should not be
                        //     executed
                        //
                        // Le WASM meurt, et **plus aucun onglet ne répond**.
                        // C'est ce que l'auteur a vu — « au bout d'un moment
                        // la nav de la tabbar se fige » — et « au bout d'un
                        // moment » veut dire *à la quatrième navigation*.
                        //
                        // `chemin` vient du routeur et vit aussi longtemps que
                        // l'application : le relire directement dans chaque
                        // closure ne crée rien qui puisse être détruit. Un
                        // signal intermédiaire n'économisait qu'un appel de
                        // fonction sur une comparaison de chaînes.
                        let actif = move || on_y_est(&chemin.get(), ici);
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
                                    // **La place retenue, pas la racine.**
                                    // Un onglet qu'on retrouve doit rendre ce
                                    // qu'on y avait laissé — c'est ce que fait
                                    // une `NavigationStack` par onglet, et
                                    // c'est la moitié qu'un navigateur laisse
                                    // reproduire.
                                    //
                                    // Lue à chaque rendu, donc à chaque
                                    // navigation : une valeur figée au montage
                                    // renverrait à la place d'il y a trois
                                    // écrans.
                                    href=move || place_retenue(ici)
                                    attr:aria-current=move || actif().then_some("page")
                                    attr:class=move || {
                                        // `presse--ligne` et non `presse` :
                                        // une ligne pleine largeur qui cède de
                                        // trois pour cent est une embardée.
                                        // C'est `ONTPresse(echelle: 0.985)`,
                                        // que l'app nomme `.ontLigne`.
                                        let base = "presse--ligne survol flex items-center \
                                                    gap-3 rounded-full px-3 py-2 font-titre text-sm \
                                                    no-underline";
                                        if actif() {
                                            format!("{base} bg-accent/15 ring-1 ring-accent/30 text-accent")
                                        } else {
                                            format!("{base} text-encre-douce hover:text-encre")
                                        }
                                    }
                                >
                                    <Signe nom=destination.signe plein=true />
                                    {destination.nom}
                                </A>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>

            // **Le corpus, en rayons dépliables.** Il défile, lui — et c'est
            // tout l'enjeu du compte épinglé juste en dessous : sur soixante-
            // dix livres, une dernière section mettrait « Vous » hors de vue.
            //
            // `min-h-0` sur la boîte qui défile : dans une colonne flex, un
            // enfant ne descend pas sous la taille de son contenu sans ça, et
            // la barre déborderait au lieu de défiler.
            <div class="-me-1 mt-5 min-h-0 flex-1 overflow-y-auto pe-1">
                <Suspense fallback=|| ()>
                    {move || Suspend::new(async move {
                        let Ok(ensembles) = plan.await else {
                            return ().into_any();
                        };
                        ensembles
                            .into_iter()
                            .filter_map(|ensemble| {
                                // **Seulement les corpus qui ont un livre à
                                // proposer**, mot pour mot la règle de l'app :
                                // un en-tête « Berit Hadashah » suivi de rien
                                // annoncerait un rayon vide.
                                let livres: Vec<_> = ensemble
                                    .sections
                                    .iter()
                                    .flat_map(|s| s.livres.iter())
                                    .filter(|l| l.ecrit)
                                    .map(|l| (l.id.clone(), l.titre.clone()))
                                    .collect();
                                if livres.is_empty() {
                                    return None;
                                }
                                Some(
                                    view! {
                                        <details open class="mb-2 group">
                                            // L'en-tête est **nettement plus
                                            // petit que ses lignes**, et c'est
                                            // une correction que l'app a payée :
                                            // au corps de ses lignes, il cessait
                                            // d'être un en-tête — « Kenesset » se
                                            // lisait comme un livre de plus.
                                            <summary class="survol flex cursor-pointer list-none items-center gap-2 rounded-full px-3 py-1.5 font-titre text-[0.72rem] text-encre-douce/70 hover:text-encre-douce marker:content-['']">
                                                <span class="flex-1 truncate">{ensemble.titre}</span>
                                                // Le témoin est **toujours
                                                // visible**, comme sur l'iPad —
                                                // le Mac ne l'a que parce qu'il
                                                // l'a dessiné.
                                                <svg
                                                    aria-hidden="true"
                                                    viewBox="0 0 12 12"
                                                    class="size-2.5 shrink-0 transition-transform duration-200 ease-out group-open:rotate-0 -rotate-90 motion-reduce:transition-none"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2.2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                >
                                                    <path d="M2.5 4.5 6 8l3.5-3.5" />
                                                </svg>
                                            </summary>
                                            <ul class="m-0 mt-0.5 flex list-none flex-col p-0">
                                                {livres
                                                    .into_iter()
                                                    .map(|(id, titre)| {
                                                        view! {
                                                            <li>
                                                                // **L'icône tient
                                                                // l'aplomb**, et ce
                                                                // n'est pas qu'un
                                                                // ornement porté de
                                                                // l'app.
                                                                //
                                                                // Sans elle, le titre
                                                                // d'un livre commence
                                                                // là où commence
                                                                // l'**icône** des
                                                                // destinations, donc
                                                                // à gauche de leurs
                                                                // libellés : un
                                                                // sous-niveau
                                                                // paraissait moins
                                                                // en retrait que son
                                                                // rayon. Relevé en
                                                                // regardant les bords
                                                                // l'un sous l'autre
                                                                // sur une capture —
                                                                // rien dans le code
                                                                // ne le dit, et la
                                                                // session macOS avait
                                                                // prévenu que c'est
                                                                // le seul contrôle
                                                                // qui l'attrape.
                                                                <A
                                                                    href=format!("/fr/webapp/{id}")
                                                                    attr:class="presse--ligne survol flex items-center gap-3 rounded-full py-1.5 px-3 font-titre text-sm text-encre-douce no-underline hover:text-encre"
                                                                >
                                                                    <Signe nom="feuillets" />
                                                                    <span class="truncate">{titre}</span>
                                                                </A>
                                                            </li>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </ul>
                                        </details>
                                    }
                                    .into_any(),
                                )
                            })
                            .collect_view()
                            .into_any()
                    })}
                </Suspense>
            </div>

            // **Le compte va en bas, épinglé, hors du défilement.**
            //
            // C'est la place qu'Apple Music lui donne, et la session macOS a
            // donné la raison : *le compte n'est pas une destination parmi les
            // livres, c'est qui regarde.* Sur un corpus complet, une dernière
            // section le mettrait à soixante-dix livres de là.
            // `mt-auto` a disparu : le corpus au-dessus prend désormais la
            // place restante, donc il n'y a plus rien à pousser vers le bas.
            // Le laisser ferait deux prétendants à l'espace libre.
            <div class="border-t border-filet/60 pt-4 mt-4">
                <A
                    href="/fr/compte"
                    attr:class="presse--ligne survol flex items-center gap-3 rounded-full border border-filet px-3 py-2 font-titre text-sm text-encre-douce no-underline hover:border-or/50 hover:text-encre"
                >
                    <Signe nom="compte" plein=true />
                    "Vous"
                </A>
            </div>
        </nav>
    }
}

/// **Où l'on en était dans chaque onglet.**
///
/// ## Ce qu'un navigateur n'a pas
///
/// iOS donne une `NavigationStack` **par onglet** : on descend Bible → Bereshit
/// → chapitre, on passe au Lexique, on revient à la Bible — et l'on est encore
/// dans le chapitre. Un navigateur n'a qu'**une** pile, partagée, et un onglet
/// y ramène toujours à sa racine.
///
/// On ne peut pas lui rendre de vraies piles, et il ne faut pas essayer :
/// rejouer une pile par-dessus celle du navigateur ferait deux histoires, dont
/// le bouton « précédent » ne saurait plus laquelle dérouler.
///
/// ## Ce qui se reproduit, et c'est la moitié qui compte
///
/// **Revenir où l'on était.** L'onglet mène au dernier chemin visité sous lui,
/// pas à sa racine — c'est le comportement qu'on remarque, et son absence est
/// ce que l'auteur a senti en demandant « des stacks par tab ».
///
/// Ce qu'on ne reproduit pas, et il faut le dire : le « précédent » du
/// navigateur reste **chronologique**, il ne remonte pas la pile de l'onglet
/// courant. Sur iOS ce geste est un chevron dans la barre ; ici c'est le fil
/// d'Ariane et la pastille de renvoi, qui remontent bien l'arborescence.
///
/// ## `sessionStorage` et non `localStorage`
///
/// Une position dans un onglet vaut pour **une session de lecture**. Retrouver
/// au réveil le chapitre où l'on était il y a trois jours serait du « Reprendre »
/// — qui existe, qui est explicite, et qui passe par le compte.
#[cfg(feature = "hydrate")]
fn retenir_la_place(chemin: &str) {
    let Some(racine) = DESTINATIONS
        .iter()
        .map(|d| d.chemin)
        .find(|racine| on_y_est(chemin, racine))
    else {
        return;
    };
    if let Some(magasin) = web_sys::window().and_then(|f| f.session_storage().ok().flatten()) {
        let _ = magasin.set_item(&format!("ont.onglet.{racine}"), chemin);
    }
}

/// Le dernier chemin visité sous cet onglet, ou sa racine.
#[cfg(feature = "hydrate")]
fn place_retenue(racine: &str) -> String {
    web_sys::window()
        .and_then(|f| f.session_storage().ok().flatten())
        .and_then(|m| m.get_item(&format!("ont.onglet.{racine}")).ok().flatten())
        .filter(|chemin| chemin.starts_with(racine))
        .unwrap_or_else(|| racine.to_string())
}

/// Côté serveur, un onglet mène à sa racine : il n'y a pas de session.
#[cfg(not(feature = "hydrate"))]
fn place_retenue(racine: &str) -> String {
    racine.to_string()
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
fn BarreDOnglets() -> impl IntoView {
    // **Le chemin se lit ici, il ne se reçoit pas.**
    //
    // Il arrivait en prop, depuis `PageDeLecture`. Avec `#[prop(into)]`, la
    // conversion crée un signal **possédé par la portée de l'appelant** — donc
    // par la page. À la navigation, la page est détruite, et les closures
    // d'attribut de la barre le relisent : panic, WASM mort, plus un onglet ne
    // répond.
    //
    // C'est ce que l'auteur a vu — « au bout d'un moment la nav de la tabbar se
    // fige » —, et « au bout d'un moment » voulait dire *à la quatrième
    // navigation*. Le banc l'a nommé en une ligne : *« you tried to access a
    // reactive value … but it has already been disposed »*, défini à
    // `into_reactive_value.rs:17`.
    //
    // `use_location()` rend le mémo du **routeur**, qui vit aussi longtemps que
    // l'application. Rien n'est créé, donc rien ne peut être détruit.
    //
    // > Un signal traverse mal une frontière de composant quand les deux n'ont
    // > pas la même durée de vie. La barre survit aux pages ; son chemin doit
    // > venir de ce qui survit aussi.
    let chemin = leptos_router::hooks::use_location().pathname;
    // Chaque navigation dépose la place courante sous son onglet.
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| retenir_la_place(&chemin.get()));
    view! {
        <nav
            aria-label="La liseuse"
            class="habillage-app barre-d-onglets pointer-events-none fixed inset-x-0 bottom-0 z-40 flex justify-center px-4 pb-3 lg:hidden"
            style="padding-bottom: calc(0.75rem + env(safe-area-inset-bottom))"
        >
            <ul class="verre pointer-events-auto m-0 flex w-full max-w-md list-none items-stretch justify-around gap-1 rounded-full border border-filet/50 p-1">
                {DESTINATIONS
                    .iter()
                    .chain(std::iter::once(&COMPTE))
                    .map(|destination| {
                        let ici = destination.chemin;
                        // Même raison qu'en barre latérale, quatre cents lignes
                        // plus haut : rien de réactif ne se crée dans une
                        // boucle de rendu, sans quoi la navigation le détruit
                        // et les attributs le relisent.
                        let actif = move || on_y_est(&chemin.get(), ici);
                        view! {
                            <li class="flex-1">
                                <A
                                    // **La place retenue, pas la racine.**
                                    // Un onglet qu'on retrouve doit rendre ce
                                    // qu'on y avait laissé — c'est ce que fait
                                    // une `NavigationStack` par onglet, et
                                    // c'est la moitié qu'un navigateur laisse
                                    // reproduire.
                                    //
                                    // Lue à chaque rendu, donc à chaque
                                    // navigation : une valeur figée au montage
                                    // renverrait à la place d'il y a trois
                                    // écrans.
                                    href=move || place_retenue(ici)
                                    attr:aria-current=move || actif().then_some("page")
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
                                        if actif() {
                                            format!("{base} bg-encre/10 text-marque-encre")
                                        } else {
                                            format!("{base} text-encre-douce")
                                        }
                                    }
                                >
                                    <Signe nom=destination.signe plein=true />
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
            // `/fr/webapp` se déclare `(StaticSegment("fr"), StaticSegment("lire"))`.
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
        assert!(on_y_est("/fr/webapp/bereshit", "/fr/webapp"));
        assert!(on_y_est("/fr/webapp", "/fr/webapp"));
        assert!(!on_y_est("/fr/webappnt-ils", "/fr/webapp"));
        assert!(!on_y_est("/fr/lexique", "/fr/webapp"));
    }
}
