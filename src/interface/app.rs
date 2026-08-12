use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet};
use leptos_router::{
    components::{Route, Router, Routes},
    SsrMode, StaticSegment,
};

use crate::interface::design::{Entete, PiedDePage, Section};
use crate::interface::pages::{Accueil, Auteur, Conditions, Confidentialite, Negations, Pourquoi};
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
                <link rel="apple-touch-icon" href="/images/montagne-512.png" />

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
        <Stylesheet id="leptos" href="/pkg/ontbible.css" />

        <Router>
            // Le segment de langue est délibéré (§4) : il épargne une migration
            // le jour d'une édition anglaise, et il ne coûte que trois
            // caractères. C'est `main.rs` qui envoie « / » vers « /fr ».
            <Entete />
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
                    <Route
                        path=(StaticSegment("fr"), StaticSegment("ce-que-l-ont-n-est-pas"))
                        view=Negations
                    />
                    <Route path=(StaticSegment("fr"), StaticSegment("l-auteur")) view=Auteur />
                    <Route
                        path=(StaticSegment("fr"), StaticSegment("confidentialite"))
                        view=Confidentialite
                    />
                    <Route
                        path=(StaticSegment("fr"), StaticSegment("conditions"))
                        view=Conditions
                    />
                </Routes>
            </main>
            <PiedDePage />
        </Router>
    }
}

/// La page absente.
///
/// Elle ne s'excuse pas et ne propose pas un plan du site : elle ramène à
/// l'accueil, qui est la seule chose utile à quelqu'un qui s'est perdu. Et
/// elle demande à ne pas être indexée — une page d'erreur dans les résultats
/// d'un moteur ne sert personne.
#[component]
fn Introuvable() -> impl IntoView {
    view! {
        <Tete
            titre="Page introuvable"
            description="Cette page n'existe pas."
            chemin="/fr"
        />
        <leptos_meta::Meta name="robots" content="noindex, follow" />

        <Section>
            <h1>"Cette page n'existe pas."</h1>
            <p><a href="/fr">"Revenir à l'accueil."</a></p>
        </Section>
    }
}
