use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::interface::pages::Accueil;

/// L'enveloppe HTML rendue par le serveur.
///
/// `lang="fr"` n'est pas décoratif : il décide de la césure, des guillemets et
/// de la voix qu'emploiera un lecteur d'écran. Le segment de langue des URL
/// (§4) prépare une édition anglaise ; le jour venu, c'est ici que la valeur
/// deviendra dynamique.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="fr">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                // La couleur de la barre du navigateur sur mobile — l'aubergine
                // du logo, pour que le chrome ne coupe pas la page en deux.
                <meta name="theme-color" content="#421B26" />
                // Deux favicons, et l'ordre compte : un navigateur qui
                // comprend le SVG prend le vecteur, net à toute densité ; les
                // autres retombent sur le PNG. Le PNG seul suffirait, mais un
                // écran d'accueil iOS l'affiche en 180 points, où il baverait.
                <link rel="icon" href="/images/logomark.svg" type="image/svg+xml" />
                <link rel="icon" href="/images/montagne-512.png" sizes="512x512" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ontbible.css" />
        <Title text="La Bible ONT" />

        <Router>
            <main>
                <Routes fallback=|| view! { <p>"Cette page n'existe pas."</p> }>
                    <Route path=StaticSegment("") view=Accueil />
                </Routes>
            </main>
        </Router>
    }
}
