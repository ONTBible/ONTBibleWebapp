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
        VivierEmbarque::charger()
            .expect("daily.json illisible — le vivier est embarqué à la compilation"),
    );
    let horloge: Arc<dyn Horloge> = Arc::new(HorlogeSysteme);

    // Le corpus et le lexique, mêmes règles : analysés une fois, et l'échec est
    // fatal. Les livres, eux, ne sont analysés qu'à la première visite — c'est
    // ce qui garde le démarrage à froid court quand le vault passera de trois
    // livres à soixante-dix.
    let corpus: Arc<dyn Corpus> = Arc::new(
        CorpusEmbarque::charger()
            .expect("corpus.json illisible — le plan est embarqué à la compilation"),
    );
    let lexique: Arc<dyn Lexique> = Arc::new(
        LexiqueEmbarque::charger()
            .expect("glossary.json illisible — le lexique est embarqué à la compilation"),
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
        // Le manifeste d'application web — ce qui rend le site installable sur
        // Android, avec son nom, son icône et sa couleur.
        //
        // Servi par une **route** et non depuis `public/`, comme le fichier
        // d'association : le paquet Lambda ne porte que le binaire et
        // `hash.txt`, donc rien de `public/` n'y est joignable. Seuls `/pkg/`,
        // `/images/`, `/fontes/` et `robots.txt` partent vers le seau.
        //
        // Le fichier reste la source — `include_str!` le lit à la compilation.
        // L'écrire en Rust ferait deux endroits à tenir d'accord.
        .route(
            "/manifest.webmanifest",
            axum::routing::get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "application/manifest+json",
                    )],
                    include_str!("../public/manifest.webmanifest"),
                )
            }),
        )
        // La carte du site pour un agent qui le lit.
        //
        // Elle existe pour la raison qu'expose le §I de « Le pourquoi » : une
        // machine entraînée sur l'écrit du monde retombe par défaut dans le
        // cadre grec, et juge alors la restitution fautive. C'est arrivé — le
        // même modèle a changé d'avis quand on lui a demandé de relire depuis
        // l'ontologie hébraïque, sans que rien du site ait bougé.
        //
        // Le site ne pouvait pas le lui dire : le cadre est *dans* une page,
        // qu'il faut avoir trouvée. Ce fichier le pose à la racine, avant toute
        // lecture — c'est la même leçon, portée par l'infrastructure au lieu de
        // la prose.
        //
        // `text/plain` et non `text/markdown` : c'est ce que sert le standard,
        // et c'est ce qu'un outil qui ne le connaît pas sait encore afficher.
        .route(
            "/llms.txt",
            axum::routing::get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "text/plain; charset=utf-8",
                    )],
                    include_str!("../public/llms.txt"),
                )
            }),
        )
        .route(
            "/sitemap.xml",
            axum::routing::get(|| async move {
                (
                    [(axum::http::header::CONTENT_TYPE, "application/xml")],
                    plan,
                )
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
        // Le HTML n'est pas gardé, et il le **dit**.
        //
        // En pratique il ne l'était déjà pas : CloudFront ne le retient pas, et
        // sans `last-modified` un navigateur n'a aucune base pour le retenir non
        // plus. Mais ce silence tenait par accident. Or le déploiement efface
        // les anciens fichiers empreintés — un navigateur qui garderait le HTML
        // réclamerait un WASM supprimé, et la page arriverait morte.
        //
        // `no-cache` et non `no-store` : le premier autorise le retour arrière
        // et la mise en cache mémoire, il exige seulement de revalider avant de
        // réafficher. Le second interdirait jusqu'au bouton « précédent ».
        //
        // Posé ici et non dans CloudFront : c'est l'origine qui sait que la page
        // porte le verset du jour, lequel change à minuit. Une politique de CDN
        // l'imposerait aussi à `/pkg/`, qui veut exactement l'inverse.
        .layer(axum::middleware::map_response(sans_cache))
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

/// Déclare le HTML non gardé, et ne touche à rien d'autre.
///
/// Le filtre porte sur le type de contenu plutôt que sur le chemin : c'est ce
/// que la réponse **est** qui décide, pas l'adresse qui l'a produite. Une route
/// qui rendrait du JSON ou du XML — l'association, le plan de site — garde donc
/// sa propre politique, et une page ajoutée demain hérite de celle-ci sans
/// qu'on ait à y penser.
///
/// Une réponse qui porte déjà un `cache-control` n'est pas touchée.
///
/// ## En développement, **rien** n'est gardé
///
/// Et ce n'est pas un confort, c'est une correction. En mode `watch`,
/// `hash.txt` n'est pas recalculé : la feuille garde son nom empreinté pendant
/// qu'on en réécrit le contenu. Le serveur ne posant aucune politique sur
/// `/pkg/`, un navigateur applique alors son cache heuristique — et Safari
/// resservait sa copie sans même revalider.
///
/// La page capturée au simulateur portait donc le style d'il y a une heure. On
/// mesure un décalage, on cherche la cause dans la règle qu'on vient d'écrire,
/// et la règle n'est jamais arrivée. C'est le §8 ter à l'envers : les
/// empreintes protègent la production précisément parce que le nom change avec
/// le contenu, et en développement il ne change pas.
///
/// La production n'est pas touchée : CloudFront envoie `/pkg/` au seau, jamais
/// à la Lambda, et c'est lui qui pose l'année d'`immutable`.
#[cfg(feature = "ssr")]
async fn sans_cache(mut reponse: axum::response::Response) -> axum::response::Response {
    use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE};

    if reponse.headers().contains_key(CACHE_CONTROL) {
        return reponse;
    }

    // `no-store` et non `no-cache` : on ne veut pas d'une révalidation, on veut
    // qu'il n'y ait rien à révalider. Un `304` sur une feuille dont le nom n'a
    // pas bougé rendrait exactement l'ancienne.
    if cfg!(debug_assertions) {
        reponse.headers_mut().insert(
            CACHE_CONTROL,
            axum::http::HeaderValue::from_static("no-store"),
        );
        return reponse;
    }

    let html = reponse
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|valeur| valeur.to_str().ok())
        .is_some_and(|valeur| valeur.starts_with("text/html"));

    // `no-cache` et non `no-store` : le premier autorise le retour arrière et
    // la mise en cache mémoire, il exige seulement de revalider avant de
    // réafficher. Le second interdirait jusqu'au bouton « précédent ».
    if html {
        reponse.headers_mut().insert(
            CACHE_CONTROL,
            axum::http::HeaderValue::from_static("no-cache"),
        );
    }
    reponse
}
