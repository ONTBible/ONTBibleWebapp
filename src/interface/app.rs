use leptos::prelude::*;
use leptos_meta::{provide_meta_context, HashedStylesheet, MetaTags};
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, SsrMode, StaticSegment,
};

use crate::interface::design::{Bouton, Hero, PiedDePage};
use crate::interface::pages::{Application, 
    Accueil, Assistance, Conditions, Confidentialite, Fiche, Lexique, Lire, Livre, Negations,
    Passage, Pourquoi,
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
                <link rel="icon" href="/images/logomark.svg" type="image/svg+xml" />
                <link rel="icon" href="/images/montagne-512.png" sizes="512x512" />
                // L'icône d'écran d'accueil est **opaque** : iOS ne gère pas la
                // transparence d'une icône, il la remplit de noir. La montagne
                // dorée sur transparence y deviendrait une tache sur un carré
                // noir.
                <link rel="apple-touch-icon" href="/images/touch-icon.png" />

                // Les fontes du corps sont demandées dès le premier octet du
                // HTML plutôt qu'à la découverte de la feuille de style : sans
                // ça, le texte s'affiche en fonte de repli puis saute.
                <link
                    rel="preload"
                    href="/fontes/Jost-Regular.woff2"
                    as_="font"
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

/// Ce qu'un moteur de recherche comprend du site sans le lire.
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
                    <Route path=(StaticSegment("fr"), StaticSegment("l-app")) view=Application />
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
