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
    #[prop(optional)]
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
    #[prop(optional)]
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
    #[prop(optional)]
    action: Option<Children>,
) -> impl IntoView {
    let chemin = leptos_router::hooks::use_location().pathname;
    // Relevé avant que `barre` ne soit consommée par le rendu — `Children` est
    // une `FnOnce`, donc l'appeler la prend.
    let barre_posee = barre.is_some();

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
        {(chemin.get_untracked().trim_end_matches('/') == "/fr/webapp")
            .then(|| view! { <crate::interface::design::Ouverture /> })}
        <crate::interface::design::PeauDeLaLiseuse />
        // **La chrome de l'app, et elle remplace celle de l'édition.**
        //
        // `Entete` n'est plus rendu ici : il porte la navigation d'un site —
        // cinq entrées en capitales au-dessus de tout, qui disent « voici les
        // pages ». La liseuse est un lieu où l'on revient, pas une page qu'on
        // lit une fois, et sa navigation est celle de l'app.
        // ## Les deux habillages sont dans le document, et la CSS en peint un
        //
        // L'auteur a demandé le 30 septembre 2026 de garder **les deux** — les
        // barres de l'app et l'édition d'avant — avec une bascule dans « Vous ».
        //
        // Le rendu ne tranche pas : il pose les deux enveloppes, et
        // `data-habillage` décide laquelle se peint. Trois raisons :
        //
        // - **pas de cookie.** Un rendu conditionnel côté serveur demanderait
        //   d'écrire un en-tête sur des réponses que les pages ne composent
        //   pas, et une seconde mémoire à tenir d'accord avec `ont.lecture` ;
        // - **pas de saut.** Le script de l'en-tête pose l'attribut avant la
        //   première peinture, donc on ne voit jamais l'un puis l'autre ;
        // - **une seule adresse.** Deux jeux auraient dédoublé les cent
        //   soixante-trois pages du corpus dans les index, et forcé chaque
        //   lien partagé à trancher pour son destinataire.
        //
        // Le prix est l'en-tête du site et son pied rendus en pure perte quand
        // on lit dans l'app — quelques centaines d'octets, contre les cent
        // soixante-dix kilo-octets que la barre latérale du corpus pèse déjà.
        // ==C'est le petit des deux coûts, et c'est le seul qu'on ait choisi.==
        // **Sans enveloppe**, et c'est une leçon payée : un `<div>` posé autour
        // de ce composant tue l'hydratation. Il rend deux barres *frères* — la
        // latérale et les onglets —, et Leptos hydrate un fragment en comptant
        // des marqueurs qui tombent alors ailleurs :
        //
        // ```text
        // A hydration error occurred … at navigation_de_la_liseuse.rs:312
        // the framework expected a marker node, but found [object HTMLElement]
        // panicked at tachys/src/hydration.rs:216
        // ```
        //
        // Le WASM meurt au démarrage, la page reste belle, et **plus rien ne
        // répond au doigt** — le symptôme du §7 bis, pour la quatrième fois
        // dans ce dépôt et avec une quatrième cause.
        //
        // ## Et l'en-tête du site ne peut pas être rendu ici
        //
        // Deuxième mesure du même soir : `Entete` et `PiedDePage` posés dans ce
        // composant rompent l'hydratation à leur tour — `bloc.rs:106`, sur un
        // `SVGCircleElement`. Sans enveloppe, donc ce n'est pas la même cause.
        //
        // `App` rend **déjà** le pied, sous `<Show when=dans_la_liseuse()==0>`,
        // et ce compteur ne vaut pas la même chose des deux côtés au moment où
        // la condition est lue : le serveur et le client ne s'accordent alors
        // plus sur ce qu'il y a à cet endroit de l'arbre.
        //
        // ==Un second rendu d'un composant que la racine gouverne déjà est un
        // désaccord qui attend son tour.== L'habillage « édition » se limite
        // donc pour l'instant à retirer les barres, la voûte et les marges de
        // l'app. L'en-tête et le pied demandent de passer par `App`, où la
        // condition vit — c'est un second temps, et il se mesure au même banc.
        //
        // ==Un habillage se pose sur l'élément, jamais autour de lui.== Les
        // deux barres portent donc `habillage-app` dans leur propre `class`, et
        // rien ne s'interpose entre le composant et son parent.
        //
        // Trouvé par `banc-erreurs.html` en deux passes : retirer `Entete` et
        // `PiedDePage` n'a rien changé — ce qui les a innocentés —, retirer
        // cette enveloppe a tout rendu. *Retirer le suspect est plus court que
        // de raisonner sur lui.*
        <crate::interface::design::NavigationDeLaLiseuse />
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
                // `voute` et le filet sont ceux du `Bloc` d'avant le chantier :
                // `nu=true` les lui retire, et ils reviennent ici pour
                // l'édition. Portés par l'enveloppe plutôt que par la section,
                // ils se neutralisent d'une règle sous l'app au lieu de
                // demander un second prop à `Bloc`.
                "liseuse-corps voute border-t border-filet/50 \
                 pb-24 lg:ps-[16.5rem] lg:pb-0 {}",
                crate::interface::design::sens().get().classe(),
            )
        }>
        // **Nu**, et c'est ce qui sépare un écran d'app d'une section de
        // page : ni voûte d'aubergine, ni filet de section. L'app peint un
        // fond plat — `ontScreen()` — et rien d'autre.
        <Bloc page=liste nu=true>
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
                    // **`habillage-app`**, et c'est mesuré plutôt que supposé :
                    // sous l'édition, `Entete` porte déjà un lien vers
                    // `/fr/rechercher` et `PiedDePage` un second. Le bouton y
                    // ferait un troisième accès à la même page, posé en
                    // flottant au-dessus du texte — c'est-à-dire du bruit là où
                    // l'habillage entier existe pour n'en pas avoir.
                    <div class="habillage-app pointer-events-none sticky top-0 z-30 -mx-1 mb-2 flex justify-end py-2">
                        {action()}
                    </div>
                }
            })}

            // Le fil **cède la place** à la pastille quand il y en a une.
            {(!fil.is_empty() && !barre_posee)
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

            {rappel
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
                class="arrivee mt-0 text-balance"
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
                style:font-size=move || {
                    (!liste).then_some("calc(var(--text-base) * var(--lecture, 1) * 1.7)")
                }
                class=("mb-4", !liste)
                // Le titre d'une liste est celui de l'app : serré, à gauche, et
                // sans l'air d'une affiche. Celui d'une lecture ne bouge pas.
                class=("mb-6", liste)
                class=("text-2xl", liste)
                class=("leading-none", liste)
            >
                {move || titre.get()}
            </h1>

            {chapeau.map(|chapeau| view! { <div class="arrivee mb-14" style:--rang="1">{chapeau()}</div> })}

            // **La seconde échelle.** Tout ce que la liseuse contient hérite
            // de cette taille, donc tout suit le réglage du lecteur — une
            // glose, une fiche, un verset en lecture suivie. Au facteur 1,
            // c'est exactement ce dont la page héritait déjà.
            //
            // Elle est posée **ici et pas sur le `Bloc`** : le fil d'Ariane et
            // le titre sont de la chrome, et ils ne doivent pas enfler quand on
            // monte le corps — c'est la règle de l'app, et sa raison est
            // qu'une chrome qui grandit mange la place du texte.
            <div class="arrivee liseuse" style:--rang="2">{children()}</div>
        </Bloc>
        </div>
        // Le pied du site, que `App` ne rend pas dans la liseuse : il compte
        // les pages de liseuse à l'écran et s'efface tant qu'il y en a une.
        // Sous l'édition il fait partie de l'habillage, donc il se pose ici.
    }
}
