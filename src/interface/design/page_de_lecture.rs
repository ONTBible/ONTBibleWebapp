use leptos::prelude::*;

use crate::interface::design::Bloc;

/// Le gabarit des pages de la liseuse et du lexique.
///
/// ## Pourquoi elles n'ont pas d'ouverture
///
/// Le reste du site s'ouvre sur un `Hero` qui remplit l'écran : on y arrive
/// sans rien savoir, et l'ouverture est ce qui dit où l'on est. Ces pages-ci
/// sont l'inverse — **on y arrive en sachant**, par un lien partagé, par un mot
/// d'or, par le sommaire. Leur poser un écran d'ouverture mettrait un obstacle
/// d'une hauteur de fenêtre entre le lecteur et le texte qu'il est venu lire.
///
/// C'est la même logique que les pages légales, qui posent leur en-tête
/// elles-mêmes. Il n'y a jamais deux `<header>` sur une page.
///
/// ## Le fil
///
/// Un retour nommé, et non un « ← Retour » générique : ces pages sont
/// **profondes** — un verset est à trois niveaux du sommaire — et quelqu'un qui
/// arrive par un lien partagé n'a pas d'historique à remonter. Le fil est sa
/// seule façon de savoir dans quel livre il se trouve.
#[component]
pub fn PageDeLecture(
    /// Le fil d'Ariane, du plus général au plus précis. La page courante n'y
    /// figure pas : c'est le titre qui la nomme.
    #[prop(optional)]
    fil: Vec<(String, String)>,
    /// La ligne au-dessus du titre — « Torah », « Intraduisible ».
    #[prop(optional, into)]
    rappel: Option<String>,
    /// Le titre de la page.
    ///
    /// **Un `Signal` et non une `String`**, et il l'a fallu : le nom d'une
    /// unité dépend du registre de lecture — « Chapitre 2 » en français reçu,
    /// « Parashah 2 » en glose ONT — que le lecteur bascule sans recharger.
    /// Un prop de type texte fermait la porte à ce calcul, et la page de
    /// lecture affichait le nom ONT pendant que le sommaire affichait l'autre.
    ///
    /// Les appels qui passent une `String` continuent de marcher : `into`
    /// enveloppe la valeur dans un signal constant.
    #[prop(into)]
    titre: Signal<String>,
    /// Ce qui se lit sous le titre, avant le corps — un renvoi, une mention.
    ///
    /// **`optional_no_strip`**, pour que l'appelant puisse en poser *un ou
    /// aucun* selon l'arbre. Avec `optional` seul, le prop devient un `Children`
    /// et l'absence ne s'exprime plus : il faudrait passer une fermeture qui ne
    /// rend rien — et le conteneur qui l'enveloppe, lui, rendrait quand même sa
    /// marge de trois rem et demi. ==Un vide qui occupe la place d'un contenu
    /// n'est pas une absence.==
    #[prop(optional_no_strip)]
    chapeau: Option<Children>,
    children: Children,
    /// **Un écran de liste, et non de lecture.**
    ///
    /// L'app a deux registres d'écran, et ils ne se composent pas pareil : une
    /// **liste** s'ouvre sur un grand titre serré à gauche ; une **lecture**
    /// s'ouvre sur un titre d'unité, avec son rappel et son chapeau.
    ///
    /// Le site les confondait — le sommaire portait un œil-de-bœuf, un titre
    /// d'affiche et un paragraphe d'introduction, c'est-à-dire l'appareil d'une
    /// page d'édition. Une liste ne s'introduit pas : on y revient, on ne la
    /// découvre pas.
    #[prop(optional)]
    liste: bool,
    /// La barre du haut d'un écran de lecture — la pastille et « aA ».
    ///
    /// **Elle prend la place du fil**, elle ne s'y ajoute pas : les deux
    /// disent « tu es dans Bereshit, au 3 », et les empiler donnerait deux
    /// appareils de navigation pour un écran. Les écrans de liste gardent le
    /// fil, n'ayant pas de pastille.
    ///
    /// Elle est rendue **hors de `.liseuse`**, comme le titre : c'est de la
    /// chrome, et une chrome qui enfle avec le réglage du corps mange la place
    /// du texte.
    ///
    /// **`optional_no_strip`** : elle n'existe que sous l'arbre de l'app, et
    /// l'édition doit pouvoir n'en poser aucune. Voir `chapeau`.
    #[prop(optional_no_strip)]
    barre: Option<Children>,
    /// Ce qui se pose **en haut à droite** d'un écran de liste.
    ///
    /// C'est la place de `ONTPlacement.principale` chez l'app — le bouton de
    /// recherche de la Bible, et rien d'autre aujourd'hui. Un seul objet : une
    /// barre d'outils qui en porte trois cesse d'être une barre d'outils.
    ///
    /// **Distinct de `barre`**, qui appartient à un écran de *lecture* et
    /// porte sa pastille. Les deux ne coexistent jamais : une liste n'a pas de
    /// renvoi à afficher.
    ///
    /// **`optional_no_strip`**, pour la même raison que `barre` : sous l'édition,
    /// c'est elle qui porte le « aA » d'un écran de lecture, et sous l'app c'est
    /// la barre. Chaque arbre en pose une et pas l'autre.
    #[prop(optional_no_strip)]
    action: Option<Children>,
    /// **Cette page porte-t-elle du texte à lire ?**
    ///
    /// Vrai pour un passage, une fiche, la feuille de prononciation et les
    /// extraits d'une recherche : leur corps est du corpus, et le curseur de
    /// taille doit l'agrandir. Faux partout ailleurs — un sommaire, le lexique,
    /// « Vous », les réglages portent des **noms** et des commandes, pas du
    /// texte qu'on lit au long.
    ///
    /// Le défaut est `false`, et c'est voulu : ==un réglage qui agrandit
    /// l'interface se remarque tout de suite ; un corpus qui n'a pas grandi se
    /// remarque aussi, et l'on sait alors quoi corriger.== L'oubli dans ce sens
    /// se voit ; dans l'autre, il passe pour une mise en page.
    #[prop(optional)]
    corpus: bool,
) -> impl IntoView {
    let chemin = leptos_router::hooks::use_location().pathname;
    // Relevé avant que `barre` ne soit consommée par le rendu — `Children` est
    // une `FnOnce`, donc l'appeler la prend.
    let barre_posee = barre.is_some();

    // **Sous quel arbre.** Lu une fois : une page ne change pas d'arbre sans se
    // remonter, et s'abonner ici recalculerait toute la page à chaque
    // navigation pour une valeur qui n'aura pas bougé.
    let sous_l_app =
        crate::interface::arbre::arbre_maintenant() == crate::domaine::lecture::Arbre::Webapp;

    // **`liste` est un registre de l'app, et il n'a pas cours ailleurs.**
    //
    // L'édition n'a qu'une composition : celle de `main`, avec sa voûte, sa
    // mesure de 38 rem et son titre d'affiche. Un sommaire n'y est pas dessiné
    // autrement qu'un chapitre — c'est l'app qui distingue une **liste** d'une
    // **lecture**, parce qu'une liste d'application se parcourt au pouce.
    //
    // Éteint ici et une seule fois, plutôt que `liste=!edition` chez les neuf
    // appelants : ==un drapeau qui ne vaut que sous une condition se borne là où
    // il est lu, jamais chez ceux qui le posent.== Sinon la dixième page
    // l'oublie, et rien ne le dit.
    let liste = liste && sous_l_app;

    view! {
        // **La peau du lecteur est montée ici**, et c'est ce qui la borne à la
        // liseuse : ce composant est la page de corpus, les cinq qui en
        // portent l'emploient, et aucune page d'édition ne l'emploie. La règle
        // est donc structurelle, pas une table de chemins à tenir d'accord
        // avec les routes — et elle suit la vue, puisqu'un montage suit la vue.
        //
        // La recherche la monte séparément : elle rend du corpus sans passer
        // par ce cadre-ci.
        // **L'ouverture, et elle est dans le HTML du serveur.**
        //
        // Montée côté navigateur après vérification, elle arriverait *après* la
        // page — une ouverture qui ouvre ce qui est déjà ouvert. Le script de
        // l'en-tête la retire quand la session l'a déjà vue, avant la première
        // peinture.
        //
        // Elle ne retarde rien : `fixed` par-dessus, la page est rendue,
        // lisible et indexable dessous pendant qu'elle joue.
        // **Seulement à l'entrée de la liseuse**, jamais sur un passage.
        //
        // L'app ouvre au lancement. L'équivalent du lancement, ici, est
        // d'arriver sur la Bible — pas d'atterrir sur un verset.
        //
        // Un lien partagé depuis l'app mène à `/fr/webapp/{livre}/{unité}?v=1-3`,
        // et c'est **la raison d'être de cette route** (§4). Couvrir ce verset
        // cinq secondes et demie parce que le lecteur découvre le site serait
        // exactement l'inverse du service rendu : il n'a pas demandé le site,
        // il a demandé un verset.
        // **L'entrée du corpus, dans l'arbre où l'on est.** Une égalité
        // stricte : l'`Ouverture` salue l'arrivée dans la liseuse, elle n'a
        // rien à faire sur un livre ni sur une unité.
        {(chemin.get_untracked().trim_end_matches('/')
            == crate::domaine::chemins::bible(crate::interface::arbre::arbre_maintenant()))
            .then(|| view! { <crate::interface::design::Ouverture /> })}
        <crate::interface::design::PeauDeLaLiseuse />
        // **La chrome de l'app, et elle remplace celle de l'édition.**
        //
        // `Entete` n'est plus rendu ici : il porte la navigation d'un site —
        // cinq entrées en capitales au-dessus de tout, qui disent « voici les
        // pages ». La liseuse est un lieu où l'on revient, pas une page qu'on
        // lit une fois, et sa navigation est celle de l'app.
        // ## Deux leçons d'hydratation, payées le 30 septembre 2026
        //
        // Elles ont survécu au montage qui les a produites, et elles valent pour
        // tout ce qu'on posera ici.
        //
        // **Aucune enveloppe autour de ce composant.** Un `<div>` posé autour de
        // lui tue l'hydratation : il rend deux barres *frères* — la latérale et
        // les onglets —, et Leptos hydrate un fragment en comptant des marqueurs
        // qui tombent alors ailleurs.
        //
        // ```text
        // A hydration error occurred … at navigation_de_la_liseuse.rs:312
        // the framework expected a marker node, but found [object HTMLElement]
        // panicked at tachys/src/hydration.rs:216
        // ```
        //
        // Le WASM meurt au démarrage, la page reste belle, et **plus rien ne
        // répond au doigt** — le symptôme du §7 bis, pour la quatrième fois dans
        // ce dépôt et avec une quatrième cause.
        //
        // **Et `Entete` / `PiedDePage` ne se rendent pas ici.** Deuxième mesure
        // du même soir, `bloc.rs:106`, sans enveloppe — donc ce n'est pas la même
        // cause. `App` rend **déjà** le pied sous `<Show when=…>`, et la condition
        // ne vaut pas la même chose des deux côtés au moment où elle est lue.
        //
        // ==Un second rendu d'un composant que la racine gouverne déjà est un
        // désaccord qui attend son tour.==
        //
        // Trouvé par `banc-erreurs.html` en trois passes, et la première a failli
        // tromper : retirer `Entete` et `PiedDePage` n'avait rien changé, ce qui
        // les innocentait — à tort. Leur effet était masqué par la première
        // cause, qui tombait plus tôt dans l'arbre. *Un retrait qui ne change
        // rien ne dit que « ce n'est pas la première cause ».*
        // **Les barres ne se rendent que sous leur arbre.** Un chrome par
        // adresse, décidé côté serveur : rien de mort dans le document, et la
        // barre latérale ne lance pas ses deux requêtes pour une page qui ne
        // la montrera jamais.
        {sous_l_app.then(|| view! { <crate::interface::design::NavigationDeLaLiseuse /> })}
        // La barre latérale est en `fixed` : elle ne pousse rien, donc le
        // contenu se décale lui-même au-delà de `lg`. Et le bas respire de la
        // hauteur de la barre d'onglets, sans quoi la dernière ligne du
        // chapitre se lirait derrière elle.
        // **Le sens de la navigation porte sur l'écran entier**, pas sur ses
        // pièces : un empilement déplace la page, une cascade la remplit. Les
        // deux se composent — le titre et le corps gardent leur arrivée en
        // rang à l'intérieur de l'écran qui glisse.
        <div class=move || {
            format!(
                // Les marges servent les barres : la latérale pousse par la
                // gauche au-delà de `lg`, les onglets réservent le bas. Sans
                // barres, elles laisseraient un couloir vide et une bande sous
                // le dernier verset.
                // **La classe d'écran, et la feuille descend.** Les
                // composants profonds — `Groupe`, `Ligne`, `EnteteDeSection` —
                // n'ont rien à savoir de l'arbre : ils rendent une fois, et la
                // CSS les habille.
                //
                // Ce n'est pas la double peinture retirée le 30 septembre.
                // Celle-là mettait **deux chromes dans un même document** —
                // cent soixante-six kilo-octets de DOM mort et deux requêtes
                // par page. Ici il y a un seul rendu et deux feuilles.
                //
                // ==Dupliquer un rendu coûte ; le restyler ne coûte rien.==
                "{} {} {}",
                if sous_l_app { "ecran-app" } else { "ecran-edition" },
                if sous_l_app { "pb-24 lg:ps-[16.5rem] lg:pb-0" } else { "" },
                // **Le sens de la navigation est une affaire d'app.** Il rejoue
                // `ONTApparition` : l'écran glisse dans la direction d'où l'on
                // vient, ce qui donne à une pile de vues la profondeur qu'une
                // barre d'onglets ne dit pas.
                //
                // Une édition ne se feuillette pas, elle se charge. `main` n'avait
                // aucune animation d'entrée, et c'est juste : un mouvement sur une
                // page de texte se lit comme un chargement qui n'en finit pas.
                if sous_l_app {
                    crate::interface::design::sens().get().classe()
                } else {
                    ""
                },
            )
        }>
        // **Nu sous l'app, habillé sous l'édition**, et c'est ce qui sépare
        // un écran d'application d'une section de page.
        //
        // L'app peint un fond plat — `ontScreen()` — et rien d'autre : ni
        // voûte d'aubergine, ni filet de section, et sa mesure est celle d'une
        // liste (46 rem), où l'œil saute d'un intitulé à sa valeur.
        //
        // L'édition reprend ce que `main` rendait : `<Bloc>` **sans aucun
        // prop**, donc la voûte, le filet, et la mesure d'une phrase (38 rem).
        // C'est le défaut du composant, et il était juste — c'est de l'avoir
        // écrasé pour les deux arbres qui ne l'était pas.
        <Bloc page=sous_l_app && liste nu=sous_l_app>
            {barre.map(|barre| barre())}

            // **L'action seule, alignée à droite**, et `sticky` comme la barre
            // d'un écran de lecture : sur un sommaire de soixante-dix livres,
            // un bouton qui défile avec le titre n'est plus atteignable au
            // trentième.
            //
            // `pointer-events-none` sur la rangée : sans ça, la largeur vide à
            // gauche du bouton prendrait les clics sur toute la colonne, et la
            // première ligne de la liste deviendrait inatteignable.
            {action.map(|action| {
                view! {
                    // `pointer-events-none` pour que la largeur vide à gauche
                    // du bouton ne prenne pas les clics de toute la colonne, et
                    // la première ligne de la liste avec. Ce qu'on pose dedans
                    // rétablit le pointeur sur **lui-même** — `BoutonDeRecherche`
                    // porte `pointer-events-auto`.
                    //
                    // ## Et surtout pas `[&>*]:pointer-events-auto`
                    //
                    // Le réflexe est de le rendre à tous les enfants d'un coup,
                    // pour que la règle ne dépende pas de la discipline de
                    // chacun. Essayé le 2 octobre 2026, et **c'est une panne** :
                    // tout n'est pas un bouton. `ReglagesDeLecture` rend trois
                    // frères — un voile plein écran, le bouton, la feuille — et
                    // le voile **reste monté en permanence**, pour qu'on puisse
                    // animer sa fermeture. Fermé, il ne vit que par son
                    // `pointer-events-none`.
                    //
                    // Les deux utilitaires ont la même spécificité, donc c'est
                    // l'ordre de la feuille qui tranche : le voile reprenait le
                    // pointeur et avalait les clics de la page entière,
                    // invisible. Le piège de `Bloc` et `max-w-mesure`, un étage
                    // plus bas et avec un coût bien pire.
                    //
                    // ==Rendre en bloc ce qu'on a annulé en bloc suppose que
                    // tous les enfants voulaient la même chose.== Celui-ci
                    // comptait sur l'annulation.
                    <div class="pointer-events-none sticky top-0 z-30 -mx-1 mb-2 flex justify-end py-2">
                        {action()}
                    </div>
                }
            })}

            // Le fil **cède la place** à la pastille quand il y en a une — et
            // il n'y en a que sous l'app. Sous l'édition il est la seule
            // remontée : le retirer laisserait le lecteur sans chemin vers le
            // livre qu'il vient de quitter.
            {(!fil.is_empty() && !(sous_l_app && barre_posee))
                .then(|| {
                    view! {
                        <nav
                            aria-label="Fil d'Ariane"
                            class="mb-8 flex flex-wrap items-center gap-x-2.5 gap-y-1 text-sm uppercase tracking-capitales text-encre-douce"
                        >
                            {fil
                                .into_iter()
                                .map(|(chemin, nom)| {
                                    view! {
                                        <>
                                            <a href=chemin class="no-underline hover:text-encre">
                                                {nom}
                                            </a>
                                            // Le séparateur est décoratif : à
                                            // l'oreille, une barre oblique entre
                                            // deux liens n'est que du bruit.
                                            //
                                            // `last:hidden` retire celui du bout.
                                            // Il **sépare**, donc il n'a rien à
                                            // faire après le dernier maillon —
                                            // « Lire / Bereshit / » se lisait
                                            // comme un fil coupé.
                                            <span
                                                aria-hidden="true"
                                                class="opacity-40 last:hidden"
                                            >"/"</span>
                                        </>
                                    }
                                })
                                .collect_view()}
                        </nav>
                    }
                })}

            // **Vide vaut absent.** `optional` déshabille l'`Option` d'un prop :
            // le rappel arrive en `String`, et une page qui n'en veut pas selon
            // l'arbre ne peut pas passer `None` — elle passe la chaîne vide.
            // Sans ce filtre, l'édition gagnerait un paragraphe vide portant sa
            // marge, c'est-à-dire un blanc que rien ne justifie sous le titre.
            {rappel
                .filter(|rappel| !rappel.is_empty())
                .map(|rappel| {
                    view! {
                        <p class="mb-3 text-sm uppercase tracking-capitales text-accent">{rappel}</p>
                    }
                })}

            // **L'arrivée en cascade**, comme `ONTApparition`.
            //
            // Le titre part au rang 0, le chapeau au 1, le corps au 2 : trois
            // objets qui se suivent d'un souffle. *Sans mouvement d'entrée, un
            // écran apparaît ; avec, il arrive.*
            //
            // Le rang est porté par une propriété et non par une classe : une
            // classe par rang ferait douze classes dans la feuille pour ce que
            // `calc` fait en une ligne.
            <h1
                class="mt-0 text-balance"
                class=("arrivee", sous_l_app)
                style:--rang="0"
                // **Le titre d'unité suit le corps, comme chez l'app.**
                //
                // Il prenait `--text-3xl`, le palier d'édition — calibré pour
                // le titre d'une page d'essai, où il ouvre un écran entier.
                // Mesuré dans la page : **56,9 px contre 21 de corps, soit
                // 2,71 fois**. `ONTTypography.display` donne **1,7**.
                //
                // Ce n'est pas un écart de goût : à 2,71 le titre pèse plus
                // que les trois premières lignes du texte qu'il annonce, et le
                // chapitre commence par son propre nom au lieu de commencer.
                //
                // En `calc` et non en `em` : ce titre est **hors** de
                // `.liseuse`, parce qu'une chrome ne doit pas enfler avec le
                // réglage du corps. Il lit donc `--lecture` lui-même, ce qui
                // le fait suivre le curseur sans faire suivre le fil d'Ariane.
                //
                // **Sous l'édition, rien de tout cela.** `main` rendait
                // `<h1 class="mt-0 mb-4 text-balance">` et s'en remettait à
                // l'échelle typographique du §5, dont chaque palier a été
                // mesuré. Un titre de page d'édition ouvre un écran ; il n'a
                // pas à suivre le curseur du corps.
                style:font-size=move || {
                    (sous_l_app && !liste)
                        .then_some("calc(var(--text-base) * var(--lecture, 1) * 1.7)")
                }
                class=("mb-4", !sous_l_app || !liste)
                // Le titre d'une liste est celui de l'app : serré, à gauche, et
                // sans l'air d'une affiche. Celui d'une lecture ne bouge pas.
                class=("mb-6", sous_l_app && liste)
                class=("text-2xl", sous_l_app && liste)
                class=("leading-none", sous_l_app && liste)
            >
                {move || titre.get()}
            </h1>

            {chapeau
                .map(|chapeau| {
                    view! {
                        <div
                            class="mb-14"
                            class=("arrivee", sous_l_app)
                            style:--rang="1"
                        >
                            {chapeau()}
                        </div>
                    }
                })}

            // **La seconde échelle.** Tout ce que la liseuse contient hérite
            // de cette taille, donc tout suit le réglage du lecteur — une
            // glose, une fiche, un verset en lecture suivie. Au facteur 1,
            // c'est exactement ce dont la page héritait déjà.
            //
            // Elle est posée **ici et pas sur le `Bloc`** : le fil d'Ariane et
            // le titre sont de la chrome, et ils ne doivent pas enfler quand on
            // monte le corps — c'est la règle de l'app, et sa raison est
            // qu'une chrome qui grandit mange la place du texte.
            //
            // **Et elle ne vaut que pour les pages qui portent du corpus.
            // Corrigé le 2 octobre 2026**, sur une observation de l'auteur :
            // *« le modificateur de taille de texte ne doit pas impacter
            // l'interface, seulement le corps du texte — j'ai vu que quand
            // j'augmentais le texte, les boutons OAuth grossissaient aussi ».*
            //
            // `.liseuse` pose une **taille de police**, donc tout ce qu'elle
            // contient en hérite. Posée sur les douze pages du gabarit, elle
            // faisait enfler les boutons de connexion, les cartes de « Vous »,
            // les rangées de réglages — c'est-à-dire exactement la chrome que
            // le commentaire ci-dessus dit de protéger.
            //
            // ==Une règle qu'on énonce pour un conteneur ne vaut que pour ce
            // qu'on met dedans.== Le gabarit la posait au bon endroit ; ce
            // qu'il y mettait n'était pas toujours du texte à lire.
            //
            // Le §8 undecies l'avait écrit et c'est resté vrai à moitié : *« il
            // n'est lu que par `.liseuse`. Ni la navigation, ni le fil
            // d'Ariane, ni le panneau lui-même ne bougent — c'est la moitié du
            // sujet, et c'est la moitié qu'on oublie. »*
            <div
                class=("liseuse", corpus)
                class=("arrivee", sous_l_app)
                style:--rang="2"
            >
                {children()}
            </div>
        </Bloc>
        </div>
        // Le pied du site, que `App` ne rend pas dans la liseuse : il compte
        // les pages de liseuse à l'écran et s'efface tant qu'il y en a une.
        // Sous l'édition il fait partie de l'habillage, donc il se pose ici.
    }
}
