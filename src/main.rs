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
    use ontbible::application::ports::{Corpus, Horloge, Lexique, Vivier};
    use ontbible::infrastructure::corpus::{CorpusEmbarque, LexiqueEmbarque};
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

    // Le corpus et le lexique, mêmes règles : analysés une fois, et l'échec est
    // fatal. Les livres, eux, ne sont analysés qu'à la première visite — c'est
    // ce qui garde le démarrage à froid court quand le vault passera de trois
    // livres à soixante-dix.
    let corpus: Arc<dyn Corpus> = Arc::new(
        CorpusEmbarque::charger().expect("corpus.json illisible — le plan est embarqué à la compilation"),
    );
    let lexique: Arc<dyn Lexique> = Arc::new(
        LexiqueEmbarque::charger().expect("glossary.json illisible — le lexique est embarqué à la compilation"),
    );

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
        let corpus = corpus.clone();
        let lexique = lexique.clone();
        move || {
            provide_context(vivier.clone());
            provide_context(horloge.clone());
            provide_context(corpus.clone());
            provide_context(lexique.clone());
        }
    };

    // Le plan de site, composé depuis la liste des pages plutôt qu'écrit à la
    // main : un fichier statique se périme au premier ajout de route, et
    // personne ne s'en aperçoit avant de constater qu'une page n'est pas
    // indexée.
    let plan = {
        use ontbible::interface::tete::{ORIGINE, PAGES};

        let mut chemins: Vec<String> = PAGES.iter().map(|c| c.to_string()).collect();

        // Le corpus et le lexique s'ajoutent **calculés**, jamais écrits à la
        // main. Un plan de site figé se périme au premier livre traduit, et
        // personne ne s'en aperçoit : les pages existent, elles répondent, et
        // elles ne sont simplement jamais indexées.
        //
        // Les livres non écrits n'y sont pas : leur page dit « pas encore là »
        // et porte un `noindex`. Demander à un moteur de venir la chercher pour
        // qu'elle lui demande de repartir n'a pas de sens.
        for ensemble in corpus.sommaire() {
            for entree in ensemble.livres_ecrits() {
                chemins.push(format!("/fr/lire/{}", entree.id));
                if let Some(ouvrage) = corpus.livre(&entree.id) {
                    for unite in ouvrage.intro.iter().chain(ouvrage.chapitres.iter()) {
                        chemins.push(format!("/fr/lire/{}/{}", entree.id, unite.id));
                    }
                }
            }
        }
        for entree in lexique.entrees() {
            chemins.push(format!("/fr/lexique/{}", entree.lemme));
        }

        let entrees: String = chemins
            .iter()
            .map(|chemin| format!("<url><loc>{ORIGINE}{chemin}</loc></url>"))
            .collect();
        log!("plan de site : {} adresses", chemins.len());
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{entrees}</urlset>"#
        )
    };

    let app = Router::new()
        // L'autorisation donnée à l'app iOS d'ouvrir les liens du domaine.
        //
        // Posée **avant** tout le reste, et sur le chemin exact : Apple ne
        // tolère aucune redirection ici, et le fichier doit sortir en
        // `application/json`. Voir `interface::association` — les trois pièges
        // y sont écrits, ils sont tous silencieux.
        .route(
            "/.well-known/apple-app-site-association",
            axum::routing::get(|| async {
                (
                    [(axum::http::header::CONTENT_TYPE, "application/json")],
                    ontbible::interface::association::corps(),
                )
            }),
        )
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

    // ── Le même binaire des deux côtés ────────────────────────────────────
    //
    // Sur Lambda il n'y a pas de port à ouvrir : le runtime pousse les
    // requêtes par une boucle d'événements, et `lambda_http` traduit chacune en
    // `http::Request` que le routeur axum comprend sans rien savoir de tout ça.
    //
    // La bascule se lit dans l'environnement plutôt que dans un drapeau de
    // compilation : deux binaires divergeraient, et c'est toujours celui qu'on
    // n'essaie pas en local qui casse. C'est le choix du backend de l'app, et
    // pour la même raison.
    if std::env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok() {
        log!("ontbible sur Lambda");
        lambda_http::run(app)
            .await
            .expect("le runtime Lambda s'est arrêté");
    } else {
        log!("ontbible écoute sur http://{addr}");
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}

/// Le binaire est aussi compilé pour le navigateur, où il n'a rien à démarrer :
/// l'entrée côté client est `hydrate()`, dans `lib.rs`.
#[cfg(not(feature = "ssr"))]
pub fn main() {}
