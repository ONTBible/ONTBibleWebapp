//! La racine de composition — le seul endroit qui assemble les couches.
//!
//! C'est ici, et nulle part ailleurs, qu'on décide quelle implémentation
//! concrète répond à quel besoin. Une page ne choisit jamais son dépôt : elle
//! reçoit ce que cette fonction lui a donné.

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use ontbible::interface::app::{shell, App};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
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
