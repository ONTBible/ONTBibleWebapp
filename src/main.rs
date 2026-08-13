//! La racine de composition — le seul endroit qui assemble les couches.
//!
//! C'est ici, et nulle part ailleurs, qu'on décide quelle réalisation concrète
//! répond à quel port. Une page ne choisit jamais son horloge : elle reçoit
//! celle que cette fonction a fournie. C'est ce qui permet d'en substituer une
//! autre — figée, décalée — sans toucher à une ligne d'affichage.

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::sync::Arc;

    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use ontbible::application::ports::{Horloge, Vivier};
    use ontbible::infrastructure::horloge::HorlogeSysteme;
    use ontbible::infrastructure::vivier::VivierEmbarque;
    use ontbible::interface::app::{shell, App};

    // Le vivier est analysé une fois, au démarrage. Le refaire à chaque
    // requête coûterait 58 Ko de JSON pour un résultat identique.
    //
    // Et l'échec est fatal, délibérément : un site qui démarre sans vivier
    // servirait une page d'accueil muette, et on l'apprendrait par un lecteur.
    let vivier: Arc<dyn Vivier> = Arc::new(
        VivierEmbarque::charger().expect("daily.json illisible — le vivier est embarqué à la compilation"),
    );
    let horloge: Arc<dyn Horloge> = Arc::new(HorlogeSysteme);

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    // Les dépendances entrent par le contexte, à chaque requête. Le même clos
    // sert au rendu des pages et aux fonctions serveur — `leptos_routes_with_context`
    // enregistre les deux, donc il n'existe qu'un seul point d'injection.
    let dependances = {
        let vivier = vivier.clone();
        let horloge = horloge.clone();
        move || {
            provide_context(vivier.clone());
            provide_context(horloge.clone());
        }
    };

    // Le plan de site, composé depuis la liste des pages plutôt qu'écrit à la
    // main : un fichier statique se périme au premier ajout de route, et
    // personne ne s'en aperçoit avant de constater qu'une page n'est pas
    // indexée.
    let plan = {
        use ontbible::interface::tete::{ORIGINE, PAGES};
        let entrees: String = PAGES
            .iter()
            .map(|chemin| format!("<url><loc>{ORIGINE}{chemin}</loc></url>"))
            .collect();
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{entrees}</urlset>"#
        )
    };

    let app = Router::new()
        .route(
            "/sitemap.xml",
            axum::routing::get(|| async move {
                ([(axum::http::header::CONTENT_TYPE, "application/xml")], plan)
            }),
        )
        // La racine renvoie vers la langue. Une **redirection temporaire** et
        // non permanente : un 301 est mis en cache par le navigateur pour
        // toujours, et le jour où « / » devra choisir la langue du lecteur, on
        // ne pourrait plus reprendre la main sur les visiteurs déjà venus.
        .route(
            "/",
            axum::routing::get(|| async { axum::response::Redirect::temporary("/fr") }),
        )
        .leptos_routes_with_context(&leptos_options, routes, dependances, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("ontbible écoute sur http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

/// Le binaire est aussi compilé pour le navigateur, où il n'a rien à démarrer :
/// l'entrée côté client est `hydrate()`, dans `lib.rs`.
#[cfg(not(feature = "ssr"))]
pub fn main() {}
