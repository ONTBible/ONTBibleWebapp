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

    // Les redirections du lexique — **déduites du glossaire**, jamais écrites.
    //
    // Bâties une fois au démarrage : la table ne dépend que du glossaire, qui est
    // embarqué à la compilation. La recalculer par requête serait un parcours de
    // cent cinquante-huit lemmes pour un résultat invariant.
    //
    // Elle est **vide aujourd'hui**, et c'est voulu — voir
    // `redirections_du_lexique`. Elle s'allumera d'elle-même quand le pipeline
    // portera les demi-anneaux jusqu'à la branche que la CI clone.
    let redirections: Arc<std::collections::HashMap<String, String>> = Arc::new(
        ontbible::domaine::corpus::redirections_du_lexique(
            &lexique
                .entrees()
                .iter()
                .map(|e| e.lemme.clone())
                .collect::<Vec<_>>(),
        )
        .into_iter()
        .collect(),
    );
    println!("  {} redirection(s) de lexique", redirections.len());

    // Le compte, joint chez le backend de l'app.
    //
    // La racine se lit dans l'environnement plutôt qu'en dur : c'est
    // aujourd'hui l'adresse `execute-api`, ce sera `api.ontbible.com` le jour où
    // elle servira — et un identifiant `execute-api` change si l'API est
    // recréée. À défaut, celle qui répond aujourd'hui, pour qu'un lancement
    // local sans variable ne soit pas muet.
    let comptes: Arc<dyn ontbible::application::ports::Comptes> =
        Arc::new(ontbible::infrastructure::comptes::ComptesDuBackend::new(
            std::env::var("ONT_API").unwrap_or_else(|_| {
                "https://j451hq8d3k.execute-api.eu-west-3.amazonaws.com".to_string()
            }),
        ));

    let synchronisation: Arc<dyn ontbible::application::ports::Synchronisation> =
        Arc::new(ontbible::infrastructure::comptes::SyncDuBackend::new(
            std::env::var("ONT_API").unwrap_or_else(|_| {
                "https://j451hq8d3k.execute-api.eu-west-3.amazonaws.com".to_string()
            }),
        ));

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    // Les dépendances entrent par le contexte, à chaque requête. Le même clos
    // sert au rendu des pages et aux fonctions serveur — `leptos_routes_with_context`
    // enregistre les deux, donc il n'existe qu'un seul point d'injection.
    let dependances = {
        let comptes_pour_pages = comptes.clone();
        let sync_pour_pages = synchronisation.clone();
        let vivier = vivier.clone();
        let horloge = horloge.clone();
        let corpus = corpus.clone();
        let lexique = lexique.clone();
        move || {
            provide_context(vivier.clone());
            provide_context(horloge.clone());
            provide_context(corpus.clone());
            provide_context(lexique.clone());
            provide_context(comptes_pour_pages.clone());
            provide_context(sync_pour_pages.clone());
        }
    };

    // Le plan de site, composé depuis la liste des pages plutôt qu'écrit à la
    // main : un fichier statique se périme au premier ajout de route, et
    // personne ne s'en aperçoit avant de constater qu'une page n'est pas
    // indexée.
    let plan = {
        use ontbible::domaine::chemins as adresses;
        use ontbible::domaine::lecture::Arbre;
        use ontbible::interface::tete::{pages_d_un_arbre, ORIGINE, PAGES};

        // Les pages hors arbre, puis celles de **l'arbre canonique seul**. Les
        // deux arbres servent le même texte : les déclarer tous deux ferait du
        // contenu dupliqué, et les moteurs trancheraient eux-mêmes laquelle
        // montrer. Le `rel="canonical"` de chaque page dit déjà laquelle fait
        // foi ; le plan du site le répète au lieu de le contredire.
        let mut chemins: Vec<String> = PAGES.iter().map(|c| c.to_string()).collect();
        chemins.extend(pages_d_un_arbre(Arbre::CANONIQUE));

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
                chemins.push(adresses::livre(Arbre::CANONIQUE, &entree.id));
                if let Some(ouvrage) = corpus.livre(&entree.id) {
                    for unite in ouvrage.intro.iter().chain(ouvrage.chapitres.iter()) {
                        chemins.push(adresses::unite(Arbre::CANONIQUE, &entree.id, &unite.id));
                    }
                }
            }
        }
        for entree in lexique.entrees() {
            chemins.push(adresses::fiche(Arbre::CANONIQUE, &entree.lemme));
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
        // L'autorisation donnée à l'app **Android** d'ouvrir les liens du
        // domaine. Mêmes contraintes que celle d'Apple, en plus sévère : depuis
        // Android 12, un lien non vérifié ne propose plus de sélecteur, il part
        // droit au navigateur — et rien ne dit pourquoi.
        .route(
            "/.well-known/assetlinks.json",
            axum::routing::get(|| async {
                (
                    [(axum::http::header::CONTENT_TYPE, "application/json")],
                    ontbible::interface::association::assetlinks(),
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
        // Le compte du lecteur — trois routes qui ne rendent aucune page.
        //
        // Elles posent des cookies et redirigent, donc elles vivent ici et non
        // dans le routeur de Leptos : une fonction serveur ne peut pas écrire
        // d'en-tête `Set-Cookie` sur une réponse qu'elle ne compose pas.
        //
        // Le `State` leur est donné à part, puis effacé par `with_state` : le
        // reste du routeur attend `LeptosOptions`, et mélanger les deux ferait
        // un état composite dont chaque route n'utiliserait qu'une moitié.
        .merge(
            axum::Router::new()
                .route(
                    "/fr/compte/aller/{fournisseur}",
                    axum::routing::get(ontbible::interface::compte::aller),
                )
                .route(
                    "/fr/compte/retour",
                    axum::routing::get(ontbible::interface::compte::retour),
                )
                .route(
                    "/fr/compte/partir",
                    axum::routing::get(ontbible::interface::compte::partir),
                )
                .with_state(comptes.clone()),
        )
        // La clé IndexNow — ce qui prouve à Bing que nous tenons ce domaine.
        //
        // IndexNow est le seul chemin qui **n'exige aucun compte** : on pose une
        // clé à la racine, on POSTe la liste des adresses, et Bing, Yandex,
        // Naver et Seznam se la partagent. C'est ce qui débloque la recherche de
        // ChatGPT, qui lit l'index de Bing.
        //
        // Google, lui, **refuse le protocole** depuis 2021 et n'a pas changé
        // d'avis. Son index — donc Gemini — passe obligatoirement par la Search
        // Console, qui demande le compte de Gloire. Voir le §8 septies.
        //
        // **Ce n'est pas un secret**, et il ne faut pas la traiter comme tel :
        // sa publication *est* sa fonction. Elle ne donne aucun droit sur le
        // site ; elle atteste seulement que celui qui soumet des adresses est
        // celui qui sert le domaine.
        //
        // Le corps est la clé **nue** — pas de saut de ligne final, pas
        // d'espace. Bing compare l'octet, et un `\n` de trop rend 403 sans
        // dire lequel des deux fichiers il n'a pas aimé.
        .route(
            "/7162566e429a0cd82fba80f161e32026.txt",
            axum::routing::get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "text/plain; charset=utf-8",
                    )],
                    include_str!("../public/7162566e429a0cd82fba80f161e32026.txt"),
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
        // Les anciennes adresses du lexique.
        //
        // **Posée après les routes**, et c'est tout le sujet : une `layer`
        // d'Axum n'enveloppe que ce qui est déclaré **avant** elle. Placée
        // plus haut — le réflexe, puisqu'elle doit agir « avant » Leptos —
        // elle ne voyait aucune route du routeur, et les 114 redirections
        // engendrées rendaient toutes 404.
        //
        // « Avant » décrit l'ordre d'exécution d'une requête, « après »
        // l'ordre d'écriture des routes. Les deux mots disent la même
        // chose et se contredisent à la lecture.
        //
        // Une couche et non une route : une route `/fr/lexique/{lemme}` capterait
        // *toutes* les fiches, et il faudrait lui faire rendre la main pour
        // celles qui existent. Une couche regarde, agit quand elle reconnaît, et
        // passe la requête sinon — c'est exactement ce qu'on demande.
        //
        // La comparaison se fait sur le chemin **décodé**. `ʾ` est U+02BE et
        // s'écrit `%CA%BE` dans une adresse : les deux formes arrivent, et une
        // table qui compare la chaîne brute échoue sur celle qui est encodée.
        // C'est la famille du `+` et du `/` de PKCE, un cran plus bas — deux
        // représentations d'une même chaîne, dont une seule est celle qu'on a en
        // tête en écrivant la comparaison.
        .layer(axum::middleware::from_fn({
            let redirections = redirections.clone();
            move |requete: axum::extract::Request, suite: axum::middleware::Next| {
                let redirections = redirections.clone();
                async move {
                    let chemin = requete.uri().path();
                    let entetes = requete.headers();

                    // **L'ancienne adresse de la liseuse.**
                    //
                    // Elle a déménagé de `/fr/lire` à `/fr/webapp` le
                    // 29 septembre 2026. Tout ce qui pointe l'ancienne doit
                    // continuer d'arriver — et ce n'est pas une politesse :
                    // `/fr/lire/{livre}/{unité}?v=1-3` est **la route des
                    // liens partagés depuis l'app** (§4), la raison d'être de
                    // toute cette page. Un lien envoyé hier dans une
                    // conversation doit ouvrir le passage, pas un 404.
                    //
                    // **Permanente, contrairement à la racine** : la cible
                    // est définitive, et l'on *veut* que le navigateur et les
                    // moteurs l'apprennent — un renvoi temporaire laisserait
                    // les index pointer l'ancienne adresse indéfiniment.
                    //
                    // `Redirect::permanent` rend un **308**, pas un 301, et
                    // la différence compte ici : un 301 autorise le client à
                    // retomber en `GET`, le 308 **préserve la méthode**. Sur
                    // une page ça ne change rien ; sur les fonctions serveur
                    // de `/api/`, qui sont des `POST`, un 301 transformerait
                    // l'appel en `GET` et rendrait une erreur qui ne
                    // nommerait pas sa cause.
                    //
                    // La requête garde sa **chaîne de requête** : `?v=1-3`
                    // désigne les versets, et la perdre rendrait le passage
                    // entier là où le lien désignait trois lignes.
                    // ## Les anciennes adresses, et il y en a trois âges
                    //
                    // `/fr/lire/*` est la forme d'origine, celle des liens
                    // partagés depuis l'app avant le 30 septembre. `/fr/webapp/*`
                    // sans `bible` est celle d'un seul jour — le 30 —, mais elle
                    // a circulé. Et les cinq onglets vivaient à la racine.
                    //
                    // ==Une adresse qu'on a servie une heure doit être renvoyée
                    // pour toujours.== On ne sait pas qui l'a copiée.
                    //
                    // Toutes vers l'arbre **canonique** : c'est l'adresse
                    // officielle du texte, et un lien reçu doit mener à la forme
                    // qui s'indexe. La préférence du lecteur corrigera ensuite,
                    // à l'arrivée.
                    // ## L'adresse s'aligne sur la préférence, côté serveur
                    //
                    // Le cookie est posé par le script de l'en-tête, avant la
                    // première peinture. Le serveur le lit et sert **le bon
                    // arbre d'emblée** : une seule requête, un seul chrome,
                    // aucune correction après coup.
                    //
                    // **302 et jamais 308.** La cible dépend du lecteur : un
                    // renvoi permanent serait mis en cache par le navigateur et
                    // par CloudFront, et le suivant hériterait du choix du
                    // précédent. C'est l'inverse exact de la règle des
                    // anciennes adresses, dont la cible ne dépend de personne.
                    if let Some(cible) = alignement(chemin, entetes) {
                        let cible = match requete.uri().query() {
                            Some(q) => format!("{cible}?{q}"),
                            None => cible,
                        };
                        return axum::response::IntoResponse::into_response(
                            axum::response::Redirect::temporary(&cible),
                        );
                    }

                    if let Some(cible) = ancienne_adresse(chemin) {
                        let cible = match requete.uri().query() {
                            Some(q) => format!("{cible}?{q}"),
                            None => cible,
                        };
                        return axum::response::IntoResponse::into_response(
                            axum::response::Redirect::permanent(&cible),
                        );
                    }

                    if let Some(lemme) = chemin.strip_prefix("/fr/lexique/") {
                        let decode = percent_encoding::percent_decode_str(lemme)
                            .decode_utf8_lossy()
                            .into_owned();
                        if let Some(vers) = redirections.get(&decode) {
                            // **301 et non 302** — l'ancienne adresse ne
                            // reviendra pas, et c'est la décision de l'auteur :
                            // un moteur qui la détient doit transférer son
                            // ancienneté à la nouvelle, ce qu'un 302 ne fait pas.
                            let cible = format!(
                                "/fr/lexique/{}",
                                percent_encoding::utf8_percent_encode(
                                    vers,
                                    percent_encoding::NON_ALPHANUMERIC,
                                )
                            );
                            return axum::response::IntoResponse::into_response(
                                axum::response::Redirect::permanent(&cible),
                            );
                        }
                    }
                    suite.run(requete).await
                }
            }
        }))
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

/// Où mener une requête dont l'arbre diverge de la préférence du lecteur.
///
/// Rend `None` quand tout est en place — le cas courant.
///
/// ## Pourquoi le serveur, et pas le navigateur
///
/// Le script de l'en-tête sait déjà tout : il lit la préférence et la largeur
/// de l'écran avant la première peinture. Il peut donc corriger l'adresse — mais
/// seulement l'**adresse** : `history.replaceState` ne re-rend rien, et le
/// document servi reste celui de l'arbre demandé.
///
/// Mesuré au banc le 1er octobre 2026, dans un cadre de 390 px :
///
/// ```text
/// url après chargement : /fr/webapp/bible     ← corrigée
/// barre latérale       : absente              ← pas re-rendue
/// en-tête du site      : présent              ← celui de la liseuse
/// ```
///
/// ==Une adresse corrigée sans que le rendu suive est le mensonge qu'on
/// voulait éviter, dans l'autre sens.== Le serveur, lui, tranche avant de
/// composer : un seul chrome, et il correspond.
fn alignement(chemin: &str, entetes: &axum::http::HeaderMap) -> Option<String> {
    use ontbible::domaine::chemins as adresses;
    use ontbible::domaine::lecture::Arbre;

    // Les routes du compte agissent : elles ne se renvoient nulle part.
    if chemin.starts_with("/fr/compte/") {
        return None;
    }

    let actuel = Arbre::du_chemin(chemin)?;
    let voulu = entetes
        .get(axum::http::header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .filter_map(|paire| paire.trim().strip_prefix("ont.habillage="))
        .find_map(|valeur| {
            Arbre::TOUS
                .into_iter()
                .find(|arbre| arbre.segment() == valeur.trim())
        })?;

    (voulu != actuel).then(|| adresses::dans(voulu, chemin))
}

/// Où mène une adresse d'avant les deux arbres, s'il y en a une.
///
/// Rend `None` quand le chemin est déjà à sa place — le cas courant, et celui
/// qui doit coûter le moins.
///
/// ## Trois âges d'adresses, et aucun ne se périme
///
/// ```text
/// /fr/lire/{livre}/{unité}      la forme d'origine, dans le fichier
///                               d'association d'iOS
/// /fr/webapp/{livre}/{unité}    la forme du 30 septembre, sans `bible`
/// /fr/qahal, /fr/lexique, …     les cinq onglets, quand ils vivaient
///                               à la racine
/// ```
///
/// Toutes mènent à l'arbre **canonique**. Un lien reçu doit ouvrir la forme qui
/// s'indexe ; ce que le lecteur préfère se règle à l'arrivée, pas dans le lien
/// qu'on lui a envoyé.
fn ancienne_adresse(chemin: &str) -> Option<String> {
    use ontbible::domaine::chemins as adresses;
    use ontbible::domaine::lecture::Arbre;

    // ## Les trois routes du compte ne se renvoient nulle part
    //
    // **Et il a fallu le mesurer.** Le commentaire d'à côté affirmait qu'elles
    // « sont servies avant ce middleware, donc n'arrivent jamais ici ». C'était
    // faux : `from_fn` est posée après elles dans le code, mais elle **enveloppe
    // le routeur entier** — mergées avant ou après, toutes les requêtes la
    // traversent.
    //
    // ```text
    // /fr/compte/aller/google   308 /fr/liseuse/compte/aller/google
    // /fr/compte/partir         308 /fr/liseuse/compte/partir
    // ```
    //
    // Le lecteur partait chez Google, revenait, et tombait sur une adresse que
    // rien ne sert. ==Une exemption qu'on affirme sans la mesurer est une
    // exemption qui n'existe pas.==
    //
    // Elles restent où elles sont pour une raison qui ne se déplace pas :
    // `/fr/compte/retour` est l'adresse enregistrée chez Google et chez GitHub.
    const OAUTH: [&str; 3] = [
        "/fr/compte/aller/",
        "/fr/compte/retour",
        "/fr/compte/partir",
    ];
    if OAUTH
        .iter()
        .any(|route| chemin == route.trim_end_matches('/') || chemin.starts_with(route))
    {
        return None;
    }

    let racine = Arbre::CANONIQUE.racine();

    // Les deux formes du corpus. `/fr/lire` d'abord, puis `/fr/webapp` sans
    // `bible` — et l'ordre compte : la seconde est un préfixe des adresses
    // vivantes, donc on vérifie d'abord qu'on n'y est pas déjà.
    for ancienne in ["/fr/lire", "/fr/webapp"] {
        let Some(suite) = chemin
            .strip_prefix(ancienne)
            .filter(|s| s.is_empty() || s.starts_with('/'))
        else {
            continue;
        };
        // `/fr/webapp/bible…`, `/fr/webapp/compte…` : déjà en place.
        if ancienne == "/fr/webapp"
            && [
                "/bible",
                "/lexique",
                "/qahal",
                "/chuqqot",
                "/compte",
                "/rechercher",
            ]
            .iter()
            .any(|vivant| suite == *vivant || suite.starts_with(&format!("{vivant}/")))
        {
            return None;
        }
        return Some(format!("{racine}/bible{suite}"));
    }

    // Les cinq onglets, quand ils étaient à la racine de `/fr`.
    for (ancien, vers) in [
        ("/fr/qahal", adresses::qahal(Arbre::CANONIQUE)),
        ("/fr/chuqqot", adresses::chuqqot(Arbre::CANONIQUE)),
        ("/fr/lexique", adresses::lexique(Arbre::CANONIQUE)),
        ("/fr/compte", adresses::compte(Arbre::CANONIQUE)),
        ("/fr/rechercher", adresses::rechercher(Arbre::CANONIQUE)),
    ] {
        if chemin == ancien {
            return Some(vers);
        }
        // `/fr/lexique/{lemme}` et `/fr/compte/lecture` gardent leur suite.
        if let Some(suite) = chemin.strip_prefix(&format!("{ancien}/")) {
            return Some(format!("{vers}/{suite}"));
        }
    }

    None
}

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

#[cfg(all(test, feature = "ssr"))]
mod epreuves_des_renvois {
    use super::ancienne_adresse;
    use ontbible::domaine::chemins as adresses;
    use ontbible::domaine::lecture::Arbre;

    /// **Les trois routes du compte ne se renvoient nulle part.**
    ///
    /// Ce sont les seules qui *agissent* — elles écrivent des cookies —, et
    /// `/fr/compte/retour` est l'adresse enregistrée chez Google et chez
    /// GitHub. Les renvoyer casse la connexion : le lecteur part chez le
    /// fournisseur, revient, et tombe sur une adresse que rien ne sert.
    ///
    /// L'épreuve existe parce que le commentaire qui la remplaçait était faux.
    /// Il affirmait que ces routes n'atteignent jamais le middleware, étant
    /// déclarées avant lui — or `from_fn` enveloppe le routeur entier. Mesuré :
    /// elles rendaient `308 /fr/liseuse/compte/aller/google`.
    ///
    /// ==Une exemption qu'on affirme sans la mesurer est une exemption qui
    /// n'existe pas.==
    #[test]
    fn les_routes_du_compte_ne_se_renvoient_nulle_part() {
        for route in [
            "/fr/compte/aller/google",
            "/fr/compte/aller/apple",
            "/fr/compte/aller/github",
            "/fr/compte/retour",
            "/fr/compte/retour?code=abc&state=xyz",
            "/fr/compte/partir",
        ] {
            assert_eq!(
                ancienne_adresse(route.split('?').next().unwrap_or(route)),
                None,
                "{route} est renvoyée — la connexion casse"
            );
        }
    }

    /// **L'adresse s'aligne sur le cookie, dans les deux sens.**
    ///
    /// Et elle ne bouge pas quand les deux s'accordent — sans quoi le serveur
    /// se renverrait à lui-même indéfiniment.
    #[test]
    fn l_adresse_s_aligne_sur_la_preference() {
        use axum::http::{header::COOKIE, HeaderMap, HeaderValue};

        let avec = |valeur: &str| {
            let mut e = HeaderMap::new();
            e.insert(
                COOKIE,
                HeaderValue::from_str(valeur).expect("un cookie lisible"),
            );
            e
        };

        for arbre in Arbre::TOUS {
            let ici = adresses::bible(arbre);
            // Accord : rien ne bouge.
            assert_eq!(
                super::alignement(&ici, &avec(&format!("ont.habillage={}", arbre.segment()))),
                None,
                "{ici} bouge alors que la préférence s'y accorde"
            );
            // Divergence : on s'aligne sur la préférence.
            let autre = arbre.autre();
            assert_eq!(
                super::alignement(&ici, &avec(&format!("ont.habillage={}", autre.segment()))),
                Some(adresses::bible(autre)),
                "{ici} ne s'aligne pas sur {autre:?}"
            );
        }

        // Sans cookie, sans arbre, ou sur une valeur inconnue : rien ne bouge.
        assert_eq!(
            super::alignement("/fr/liseuse/bible", &HeaderMap::new()),
            None
        );
        assert_eq!(
            super::alignement("/fr/l-app", &avec("ont.habillage=webapp")),
            None
        );
        assert_eq!(
            super::alignement("/fr/liseuse/bible", &avec("ont.habillage=autre-chose")),
            None,
            "une valeur inconnue doit être ignorée, pas devinée"
        );
        // Les routes du compte agissent : elles ne s'alignent jamais.
        assert_eq!(
            super::alignement("/fr/compte/retour", &avec("ont.habillage=webapp")),
            None
        );
    }

    /// Les trois âges d'adresses mènent tous à l'arbre canonique.
    #[test]
    fn les_anciennes_adresses_menent_au_canonique() {
        let k = Arbre::CANONIQUE;
        for (depuis, attendu) in [
            ("/fr/lire", adresses::bible(k)),
            (
                "/fr/lire/bereshit/bereshit-1",
                adresses::unite(k, "bereshit", "bereshit-1"),
            ),
            ("/fr/webapp", adresses::bible(k)),
            ("/fr/webapp/bereshit", adresses::livre(k, "bereshit")),
            ("/fr/qahal", adresses::qahal(k)),
            ("/fr/chuqqot", adresses::chuqqot(k)),
            ("/fr/lexique", adresses::lexique(k)),
            ("/fr/lexique/bara", adresses::fiche(k, "bara")),
            ("/fr/compte", adresses::compte(k)),
            ("/fr/compte/lecture", adresses::reglages(k)),
            ("/fr/rechercher", adresses::rechercher(k)),
        ] {
            assert_eq!(
                ancienne_adresse(depuis),
                Some(attendu.clone()),
                "{depuis} ne mène pas à {attendu}"
            );
        }
    }

    /// **Une adresse vivante ne se renvoie pas.**
    ///
    /// C'est le piège du préfixe : `/fr/webapp` est à la fois une ancienne
    /// adresse et la racine d'un arbre vivant. Sans la garde, `/fr/webapp/bible`
    /// deviendrait `/fr/liseuse/bible/bible`.
    #[test]
    fn une_adresse_vivante_ne_se_renvoie_pas() {
        for arbre in Arbre::TOUS {
            for vivante in [
                adresses::bible(arbre),
                adresses::livre(arbre, "bereshit"),
                adresses::unite(arbre, "bereshit", "bereshit-1"),
                adresses::partie(arbre, "torah"),
                adresses::lexique(arbre),
                adresses::fiche(arbre, "bara"),
                adresses::prononciation(arbre),
                adresses::qahal(arbre),
                adresses::chuqqot(arbre),
                adresses::compte(arbre),
                adresses::reglages(arbre),
                adresses::rechercher(arbre),
            ] {
                assert_eq!(
                    ancienne_adresse(&vivante),
                    None,
                    "{vivante} est renvoyée alors qu'elle est à sa place"
                );
            }
        }
    }

    /// Les pages hors arbre ne bougent pas non plus.
    #[test]
    fn les_pages_hors_arbre_ne_bougent_pas() {
        for fixe in [
            "/fr",
            "/fr/l-app",
            "/fr/le-pourquoi",
            "/fr/ce-que-l-ont-n-est-pas",
            "/fr/confidentialite",
            "/fr/conditions",
            "/fr/assistance",
        ] {
            assert_eq!(ancienne_adresse(fixe), None, "{fixe} est renvoyée");
        }
    }
}
