use leptos::prelude::*;
use leptos_meta::{provide_meta_context, HashedStylesheet, MetaTags};
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, SsrMode, StaticSegment,
};

use crate::interface::design::{image, Bouton, Hero, PiedDePage};
use crate::interface::pages::{
    Accueil, Application, Assistance, Compte, Conditions, Confidentialite, Fiche, Lexique, Lire,
    Livre, Negations, Passage, Pourquoi, Recherche,
};
use crate::interface::tete::{Tete, ORIGINE};

/// L'enveloppe HTML rendue par le serveur.
///
/// `lang="fr"` n'est pas décoratif : il décide de la césure, des guillemets et
/// de la voix qu'emploiera un lecteur d'écran.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="fr">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
                // La barre du navigateur sur mobile prend le fond de la page —
                // sinon le chrome coupe la nuit en deux.
                <meta name="theme-color" content="#18090D" />
                <meta name="apple-mobile-web-app-title" content="La Bible ONT" />

                // Deux favicons, et l'ordre compte : un navigateur qui comprend
                // le SVG prend le vecteur, net à toute densité ; les autres
                // retombent sur le PNG.
                <link rel="icon" href=image("logomark.svg") type="image/svg+xml" />
                <link rel="icon" href=image("montagne-512.png") sizes="512x512" />
                // L'icône d'écran d'accueil est **opaque** : iOS ne gère pas la
                // transparence d'une icône, il la remplit de noir. La montagne
                // dorée sur transparence y deviendrait une tache sur un carré
                // noir.
                <link rel="apple-touch-icon" href=image("touch-icon.png") />

                // Le manifeste : c'est lui qui donne à Android le nom, l'icône
                // et la couleur de l'app installée. Sans lui, « Ajouter à
                // l'écran d'accueil » ne pose qu'un raccourci de navigateur,
                // sans nom propre et ouvert dans un onglet.
                <link rel="manifest" href="/manifest.webmanifest" />

                // Les fontes du corps sont demandées dès le premier octet du
                // HTML plutôt qu'à la découverte de la feuille de style : sans
                // ça, le texte s'affiche en fonte de repli puis saute.
                //
                // `r#as` et non `as_`. Sur un élément écrit en clair dans
                // `view!`, Leptos recopie le nom de l'attribut **tel quel** : il
                // ne traduit pas le souligné final. La balise sortait donc avec
                // un `as_="font"` que le navigateur ignore, et il le disait —
                // « <link rel=preload> cannot have the empty string as `as`
                // value ». Le preload ne servait à rien depuis le premier jour,
                // ce qui est précisément le contraire de ce que ces trois
                // lignes de commentaire promettent.
                //
                // `attr:as` ne marche pas davantage : il sort littéralement
                // lui aussi. `as` est un mot-clé de Rust, et l'échappement est
                // celui de Rust — `r#`.
                <link
                    rel="preload"
                    href="/fontes/Jost-Regular.woff2"
                    // `attr:as` et non `as_` : `as` est un mot-clé de Rust, et
                    // le raccourci `as_` n'est **pas** traduit par le macro —
                    // il sort tel quel dans le HTML. Le navigateur voyait donc
                    // un preload sans type de ressource et le refusait :
                    // « <link rel=preload> cannot have the empty string as
                    // `as` value ». La fonte n'était pas préchargée du tout,
                    // c'est-à-dire exactement ce que cette balise existe pour
                    // faire.
                    r#as="font"
                    type="font/woff2"
                    crossorigin="anonymous"
                />

                // La feuille de style est déclarée **ici**, dans l'enveloppe, et
                // non dans `App` : `HashedStylesheet` a besoin des options, qui
                // n'existent que de ce côté. Elle y écrit le nom avec son
                // empreinte — `ontbible.<empreinte>.css` — au lieu d'un nom fixe
                // qu'un navigateur garderait en cache par-dessus une refonte.
                <HashedStylesheet options=options.clone() id="leptos" />

                <FicheStructuree />
                <PeauAvantLePremierRendu />

                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="grain-page min-h-screen">
                <App />
            </body>
        </html>
    }
}

/// Pose la peau du lecteur **avant** que la page n'existe.
///
/// ## Pourquoi un script, et pourquoi dans l'en-tête
///
/// Le thème vit dans le stockage du navigateur, que le serveur ne voit pas :
/// il rend donc toujours `mystique`, le défaut du site. Un lecteur qui a choisi
/// `parchemin` recevrait une nuit d'aubergine, puis du papier une fois le WASM
/// hydraté — un éclair de noir sur une page claire, sur **chaque** navigation.
///
/// Un script synchrone dans l'en-tête bloque l'analyse du document : il
/// s'exécute avant que `<body>` n'existe, donc avant la première peinture.
/// L'attribut est posé, la feuille s'applique dessus, et rien ne clignote.
///
/// C'est le seul script en clair de tout le site, et il ne fait que ça —
/// quatorze lignes, aucune dépendance, aucune requête.
///
/// ## Un cookie aurait été l'autre voie, et on l'a écartée
///
/// Lu côté serveur, il rendrait le bon thème dès le premier octet, script
/// compris. Il coûte une écriture d'en-tête sur une réponse que les pages ne
/// composent pas, un aller-retour de plus pour le poser, et une donnée du
/// lecteur qui voyage à chaque requête. Le reste des réglages est déjà dans
/// `localStorage` (§8 bis) ; une seconde mémoire pour un seul champ aurait
/// deux états à tenir d'accord.
///
/// ## Les quatre noms viennent du type, pas d'une liste écrite ici
///
/// `Theme::attribut` est la seule table. Une liste recopiée dans ce script
/// laisserait passer un thème renommé — l'attribut ne correspondrait plus, le
/// filtre le refuserait, et le lecteur retomberait sur le défaut sans qu'aucune
/// erreur ne le dise.
#[component]
fn PeauAvantLePremierRendu() -> impl IntoView {
    view! { <script inner_html=script_de_la_peau()></script> }
}

/// Le script lui-même, séparé du composant pour qu'une épreuve puisse le lire.
///
/// Les deux tables qu'il porte — les quatre thèmes, les trois préfixes de la
/// liseuse — sont **engendrées** depuis `Theme::TOUS` et `LA_LISEUSE`. Une
/// entrée ajoutée d'un côté ne peut pas manquer de l'autre.
///
/// Ce qui *peut* diverger est la **façon de comparer** : la condition de chemin
/// est écrite deux fois, ici en JavaScript et dans `c_est_la_liseuse` en Rust,
/// faute de pouvoir appeler la seconde depuis l'en-tête d'une page. C'est la
/// seule duplication de tout ce mécanisme, et `le_script_reprend_les_trois_
/// clauses_de_la_regle` en garde la forme.
fn script_de_la_peau() -> String {
    use crate::domaine::lecture::{Fonte, Theme, LA_LISEUSE};

    let connus = Theme::TOUS
        .iter()
        .map(|theme| format!("'{}'", theme.attribut()))
        .collect::<Vec<_>>()
        .join(",");
    let liseuse = LA_LISEUSE
        .iter()
        .map(|prefixe| format!("'{prefixe}'"))
        .collect::<Vec<_>>()
        .join(",");
    let fontes = Fonte::TOUTES
        .iter()
        .map(|fonte| format!("'{}'", fonte.attribut()))
        .collect::<Vec<_>>()
        .join(",");
    let (bas, haut, defaut) = (
        Theme::CORPS_MINIMUM,
        Theme::CORPS_MAXIMUM,
        Theme::CORPS_PAR_DEFAUT,
    );

    // `try` sur tout : `localStorage` **lève** quand le site est bloqué —
    // navigation privée stricte, cookies refusés — et une exception ici
    // arrêterait l'analyse de l'en-tête. La page partirait sans sa feuille.
    //
    // **Les deux réglages ne se posent pas pareil, et ce n'est pas une
    // inattention.** La peau est bornée à la liseuse : posée ailleurs, elle
    // mettrait du parchemin sous un massif d'aubergine. La taille, elle, se
    // pose partout — `--lecture` n'est lue que par `.liseuse`, qui n'existe
    // que dans la liseuse. Une valeur inerte hors de son lieu n'a pas besoin
    // d'être bornée, et la borner coûterait une seconde condition à tenir
    // d'accord avec la première.
    format!(
        "try{{var o=JSON.parse(localStorage.getItem('ont.lecture')||'{{}}'),\
         r=document.documentElement,n=+o.corps;\
         if(n>={bas}&&n<={haut})r.style.setProperty('--lecture',n/{defaut});\
         var p=location.pathname.replace(/\\/+$/,''),L=[{liseuse}],d=0;\
         for(var i=0;i<L.length;i++)if(p===L[i]||p.indexOf(L[i]+'/')===0)d=1;\
         if(d){{var c=[{connus}];\
         if(c.indexOf(o.theme)>=0)r.setAttribute('data-theme',o.theme);\
         var f=[{fontes}];\
         if(f.indexOf(o.fonte)>=0)r.setAttribute('data-fonte',o.fonte);}}\
         }}catch(e){{}}"
    )
}

/// Ce qu'un moteur de recherche comprend du site sans le lire./// Ce qu'un moteur de recherche comprend du site sans le lire.
///
/// `Book` et non `WebSite` : l'objet de ce domaine est une traduction, pas une
/// entreprise. C'est ce qui permet à un moteur de la relier à son auteur et à
/// son corpus public plutôt que de la classer comme une page parmi d'autres.
#[component]
fn FicheStructuree() -> impl IntoView {
    let fiche = format!(
        r#"{{"@context":"https://schema.org","@type":"Book",
"name":"La Bible ONT","alternateName":"מקרא הקדם",
"inLanguage":"fr","bookFormat":"https://schema.org/EBook","url":"{ORIGINE}",
"author":{{"@type":"Person","name":"Gloire Bikouta"}},
"about":"Restitution française du corpus hébreu et araméen antique fondée sur l'ontologie hébraïque fonctionnelle.",
"sameAs":["https://github.com/ONTBible/ONTBibleTranslation","https://github.com/ONTBible"]}}"#
    );

    view! { <script type="application/ld+json" inner_html=fiche></script> }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // **Ici et pas dans les pages**, bien que la peau ne vaille que sur la
    // liseuse : c'est `PeauDeLaLiseuse` — montée par `PageDeLecture` — qui la
    // borne, et elle a besoin que le signal existe déjà quand elle se monte.
    //
    // Les pages de corpus continuent de l'appeler pour leurs propres niveaux de
    // texte ; la fonction est idempotente et leur rend ce signal-ci.
    crate::interface::design::fournir_preferences();

    view! {
        <Router>
            // Le segment de langue est délibéré (§4) : il épargne une migration
            // le jour d'une édition anglaise, et il ne coûte que trois
            // caractères. C'est `main.rs` qui envoie « / » vers « /fr ».
            // Pas d'en-tête ici : `Hero` le porte, pour que l'ouverture
            // soit une seule unité qui remplit l'écran. Les pages sans
            // ouverture — les légales, l'erreur — posent le leur.
            <main id="contenu">
                <Routes fallback=Introuvable>
                    // `SsrMode::Async` : le serveur attend le verset du jour et
                    // rend la page entière d'un bloc.
                    //
                    // Par défaut, Leptos diffuse en flux — il envoie la page
                    // sans la carte, puis la pousse dans un `<template>` après
                    // `</html>`, et du JavaScript la remet à sa place. C'est
                    // bon pour une application ; c'est faux ici. Sans
                    // JavaScript, la carte n'existait pas, et un moteur de
                    // recherche ne la voyait pas non plus.
                    //
                    // Le coût est le temps d'analyse du vivier — mesurable en
                    // microsecondes, puisqu'il est en mémoire.
                    <Route path=StaticSegment("fr") view=Accueil ssr=SsrMode::Async />
                    <Route path=(StaticSegment("fr"), StaticSegment("le-pourquoi")) view=Pourquoi />
                    // Le compte. Les trois routes qui *agissent* — aller,
                    // retour, partir — sont posées avant ce routeur, dans
                    // `main.rs` : elles écrivent des cookies, ce qu'une page ne
                    // peut pas faire. Celle-ci ne fait que montrer l'état.
                    <Route path=(StaticSegment("fr"), StaticSegment("compte")) view=Compte />
                    <Route path=(StaticSegment("fr"), StaticSegment("l-app")) view=Application />
                    <Route
                        path=(StaticSegment("fr"), StaticSegment("rechercher"))
                        view=Recherche
                        ssr=SsrMode::Async
                    />
                    <Route
                        path=(StaticSegment("fr"), StaticSegment("ce-que-l-ont-n-est-pas"))
                        view=Negations
                    />
                    // La page de l'auteur est **retirée jusqu'à sa relecture**.
                    //
                    // Son texte est un premier jet écrit d'après une source
                    // privée, et le document de reprise est formel : rien de
                    // cette page ne doit être mis en ligne avant qu'il l'ait
                    // relue. Le composant reste dans `pages/auteur.rs` — c'est
                    // la route qui manque, pas le travail.
                    <Route
                        path=(StaticSegment("fr"), StaticSegment("confidentialite"))
                        view=Confidentialite
                    />
                    <Route
                        path=(StaticSegment("fr"), StaticSegment("conditions"))
                        view=Conditions
                    />
                    // L'adresse d'assistance qu'exige la fiche App Store, et
                    // qu'Apple refuse quand elle pointe une page d'accueil.
                    <Route
                        path=(StaticSegment("fr"), StaticSegment("assistance"))
                        view=Assistance
                    />

                    // ── La liseuse ────────────────────────────────────────
                    //
                    // Toutes en `SsrMode::Async`, et c'est la même raison que
                    // pour le verset du jour : ces pages **sont** leurs
                    // données. En flux, le serveur enverrait une page vide
                    // suivie du texte dans un `<template>` — invisible sans
                    // JavaScript, et invisible pour la messagerie qui prépare
                    // l'aperçu d'un lien partagé.
                    //
                    // L'ordre compte : Leptos apparie la première route qui
                    // convient, donc la plus précise passe avant la plus
                    // générale.
                    <Route
                        path=(StaticSegment("fr"), StaticSegment("lire"))
                        view=Lire
                        ssr=SsrMode::Async
                    />
                    <Route
                        path=(StaticSegment("fr"), StaticSegment("lire"), ParamSegment("livre"))
                        view=Livre
                        ssr=SsrMode::Async
                    />
                    // La route des liens partagés depuis l'app, et la seule que
                    // l'association d'app réserve à iOS. Voir
                    // `interface::association`.
                    <Route
                        path=(
                            StaticSegment("fr"),
                            StaticSegment("lire"),
                            ParamSegment("livre"),
                            ParamSegment("unite"),
                        )
                        view=Passage
                        ssr=SsrMode::Async
                    />

                    // ── Le lexique ────────────────────────────────────────
                    //
                    // Ce que promet chaque mot d'or du corpus.
                    <Route
                        path=(StaticSegment("fr"), StaticSegment("lexique"))
                        view=Lexique
                        ssr=SsrMode::Async
                    />
                    <Route
                        path=(StaticSegment("fr"), StaticSegment("lexique"), ParamSegment("lemme"))
                        view=Fiche
                        ssr=SsrMode::Async
                    />
                </Routes>
            </main>
            <PiedDePage />
        </Router>
    }
}

/// La page absente.
///
/// Elle s'ouvre comme les autres — c'est la seule chose qui la distingue d'une
/// erreur de serveur. Quelqu'un qui tombe dessus doit reconnaître le site
/// immédiatement, sinon il croit s'être trompé de domaine.
///
/// Elle ne s'excuse pas et ne propose pas un plan du site : elle ramène à
/// l'accueil, qui est la seule chose utile à qui s'est perdu. Et elle demande à
/// ne pas être indexée — une page d'erreur dans les résultats d'un moteur ne
/// sert personne.
#[component]
fn Introuvable() -> impl IntoView {
    view! {
        <Tete
            titre="Page introuvable"
            description="Cette page n'existe pas."
            chemin="/fr"
        />
        <leptos_meta::Meta name="robots" content="noindex, follow" />

        <Hero sobre=true>
            <p class="text-sm uppercase tracking-capitales text-accent">"Introuvable"</p>
            <h1 class="text-balance">"Cette page n'existe pas"</h1>
            <p class="max-w-xl text-encre-douce text-balance">
                "Le lien est peut-être ancien, ou le passage n'a pas encore été traduit."
            </p>
            <Bouton href="/fr" principal=true>"Revenir à l'accueil"</Bouton>
        </Hero>
    }
}

#[cfg(test)]
mod epreuves_de_la_peau {
    use super::script_de_la_peau;
    use crate::domaine::lecture::{c_est_la_liseuse, Theme, LA_LISEUSE};

    /// ## Le script porte les deux tables, et rien de plus
    ///
    /// Il les engendre, donc elles ne peuvent pas manquer. Ce que l'épreuve
    /// attrape est l'inverse : un thème ou un préfixe **retiré** des tables
    /// alors que le script continuerait de le nommer — ce qui arriverait si
    /// quelqu'un figeait la chaîne un jour où le `format!` gênerait.
    #[test]
    fn le_script_porte_exactement_les_deux_tables() {
        let script = script_de_la_peau();
        for theme in Theme::TOUS {
            assert!(
                script.contains(&format!("'{}'", theme.attribut())),
                "le script ignore le thème {}",
                theme.attribut()
            );
        }
        for fonte in crate::domaine::lecture::Fonte::TOUTES {
            assert!(
                script.contains(&format!("'{}'", fonte.attribut())),
                "le script ignore la fonte {}",
                fonte.attribut()
            );
        }
        for prefixe in LA_LISEUSE {
            assert!(
                script.contains(&format!("'{prefixe}'")),
                "le script ignore le préfixe {prefixe}"
            );
        }
        // Et rien qui ressemble à un chemin sans être dans la table.
        let chemins: Vec<&str> = script
            .split('\'')
            .filter(|morceau| morceau.starts_with("/fr"))
            .collect();
        assert_eq!(chemins, LA_LISEUSE, "le script nomme un chemin hors table");
    }

    /// ## Les trois clauses de la règle sont dans le script
    ///
    /// **Ce que cette épreuve prouve, et ce qu'elle ne prouve pas.**
    ///
    /// `c_est_la_liseuse` fait trois choses : elle ignore la barre finale, elle
    /// accepte le chemin exact, elle accepte le préfixe suivi d'une barre. Le
    /// script doit faire les trois, et l'épreuve vérifie que les trois **formes
    /// y sont écrites**.
    ///
    /// Elle ne les exécute pas : il n'y a pas de moteur JavaScript ici. C'est
    /// donc une garde de forme, pas de comportement — elle attrape une clause
    /// *supprimée*, pas une clause *fausse*. On la garde parce qu'une clause
    /// supprimée est le mode d'échec réel : on simplifie le script un jour
    /// où il gêne, et `/fr/lire/` cesse silencieusement d'être la liseuse.
    ///
    /// Le comportement, lui, est éprouvé du côté Rust — et les cas de
    /// `epreuves_de_la_liseuse` sont ceux que le script doit reproduire.
    ///
    /// **L'équivalence a été mesurée le 21 septembre 2026**, et la méthode se
    /// rejoue en trois lignes : extraire le script de la page servie, le passer
    /// à `node`, et lui soumettre les cas des épreuves du domaine. Le
    /// JavaScript décide comme `c_est_la_liseuse`.
    ///
    /// Ce n'est pas en CI, et c'est un choix : le coureur devrait lever le
    /// serveur *et* avoir `node`, pour garder une clause de dix caractères.
    /// La mesure est notée ici pour qu'on sache qu'elle a eu lieu et comment
    /// la refaire — une équivalence affirmée sans avoir été mesurée une seule
    /// fois n'est qu'une intention.
    #[test]
    fn le_script_reprend_les_trois_clauses_de_la_regle() {
        let script = script_de_la_peau();
        for (clause, ce_qu_elle_fait) in [
            (r"replace(/\/+$/,'')", "ignorer la barre finale"),
            ("p===L[i]", "accepter le chemin exact"),
            (
                "p.indexOf(L[i]+'/')===0",
                "accepter le préfixe suivi d'une barre",
            ),
        ] {
            assert!(
                script.contains(clause),
                "le script ne sait plus {ce_qu_elle_fait} — `{clause}` a disparu, \
                 alors que `c_est_la_liseuse` le fait toujours"
            );
        }
    }

    /// Le témoin de la règle elle-même, depuis le côté qui l'emploie.
    #[test]
    fn la_regle_borne_bien_la_liseuse() {
        assert!(c_est_la_liseuse("/fr/lire/bereshit/bereshit-1"));
        assert!(!c_est_la_liseuse("/fr"));
    }
}
