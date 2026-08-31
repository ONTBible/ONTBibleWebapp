use leptos::prelude::*;

use crate::interface::design::verset::composer;
use leptos_meta::{Link, Meta, Title};

use crate::interface::design::image;

/// L'origine publique du site.
///
/// Les adresses canoniques et Open Graph doivent être **absolues** : une
/// messagerie qui prépare un aperçu ne connaît pas le chemin d'où vient la
/// page, elle ne connaît que ce qu'on lui donne.
pub const ORIGINE: &str = "https://ontbible.com";

/// Les pages du site, pour le plan de site.
///
/// Cette liste est **la** source des pages **fixes** : `main.rs` la lit pour
/// composer `/sitemap.xml`. Quand une route s'ajoute dans `app.rs`, elle
/// s'ajoute ici — sinon un moteur ne la trouvera jamais, et rien ne le
/// signalera.
///
/// Les pages du corpus et du lexique n'y sont pas, et ne doivent pas y être :
/// elles sont calculées depuis le pipeline au démarrage du serveur. Les écrire
/// ici les figerait au premier livre traduit.
///
/// La page d'erreur n'y figure pas : elle porte un `noindex`. Celle de l'auteur
/// non plus : sa route est retirée jusqu'à sa relecture.
pub const PAGES: &[&str] = &[
    "/fr",
    "/fr/lire",
    "/fr/lexique",
    "/fr/rechercher",
    "/fr/le-pourquoi",
    "/fr/ce-que-l-ont-n-est-pas",
    "/fr/l-app",
    "/fr/assistance",
    "/fr/confidentialite",
    "/fr/conditions",
];

/// L'identifiant App Store de l'app iOS, quand elle en a un.
///
/// ## À quoi il sert
///
/// Il allume la **bannière d'app** de Safari sur iPhone et iPad : le bandeau
/// qui se pose en haut de la page avec l'icône, le nom, et un bouton
/// « OUVRIR » — « VOIR » si l'app n'est pas installée, auquel cas il mène à
/// l'App Store. C'est Safari qui la dessine ; le site ne fait que la déclarer.
///
/// ## D'où il vient
///
/// De la page « Informations sur l'app » d'App Store Connect, champ `Apple ID`.
/// Il est attribué à la **création de la fiche**, bien avant toute publication —
/// c'est pourquoi la bannière peut exister avant que l'app ne soit sur l'App
/// Store : elle mènera alors à une page « bientôt disponible ».
///
/// Tant qu'il vaut `None`, la balise n'est pas écrite. Une bannière déclarée
/// avec un identifiant faux ne produit rien de visible, mais elle laisse croire
/// que le travail est fait.
///
/// ## Pourquoi il est resté `None` pendant toute la bêta
///
/// L'identifiant existait depuis le premier jour — il naît avec la fiche — et
/// la bannière fonctionnait donc déjà. C'était le problème : elle menait à une
/// fiche App Store qui ne répondait pas, en haut de **toutes** les pages du
/// site, sur le seul appareil où l'app compte.
///
/// Et elle ne sait pas mener ailleurs. `apple-itunes-app` ne prend qu'un
/// `app-id` d'App Store ; TestFlight n'a aucune balise équivalente, aucun
/// bandeau que Safari saurait dessiner. La bêta se rejoignait par un lien ou
/// par un QR, sur `/fr/l-app`, et par rien d'autre.
///
/// Elle est allumée depuis le 18 août 2026, jour où Apple a approuvé la 1.0, et
/// en même temps que `pages::application::PUBLIEE` : les deux disent la même
/// chose, et laisser l'une en arrière rendrait le site incohérent avec
/// lui-même.
///
/// ## Ce qu'elle ne fait pas
///
/// Elle ne remplace pas les **liens universels** — voir
/// [`crate::interface::association`]. Ceux-là ouvrent l'app *directement*, sans
/// bandeau, sur les seuls chemins `/fr/lire/*`. La bannière, elle, s'adresse
/// surtout à qui n'a pas encore l'app : c'est le chemin d'acquisition, pas
/// celui du lien partagé.
///
/// Et elle n'existe que dans Safari sur iOS et iPadOS. Sur macOS, le bandeau
/// « Ouvrir dans la web app » que propose Safari relève d'une autre mécanique —
/// le lecteur a lui-même ajouté le site au Dock, et aucune balise ne la
/// déclenche.
pub const IDENTIFIANT_APP_STORE: Option<&str> = Some("6801192372");

/// Les métadonnées d'une page.
///
/// Un seul endroit pour le titre, la description, le lien canonique et les
/// balises d'aperçu. Écrites page par page, elles finiraient dépareillées —
/// et une page sans aperçu partagée dans une conversation ne montre rien.
///
/// Le `hreflang` annonce le français et le désigne comme défaut. Le jour d'une
/// édition anglaise, c'est ici qu'une seconde ligne apparaîtra, et nulle part
/// ailleurs.
#[component]
pub fn Tete(
    /// Le titre de la page, sans le nom du site — il est ajouté ici.
    #[prop(into)]
    titre: String,
    /// Ce que montrent un moteur de recherche et l'aperçu d'une messagerie.
    #[prop(into)]
    description: String,
    /// Le chemin de la page, à partir de la racine — « /fr/l-auteur ».
    #[prop(into)]
    chemin: String,
) -> impl IntoView {
    // Le suffixe n'est ajouté qu'aux pages qui ne portent pas déjà le nom.
    //
    // L'accueil le porte, parce que son titre est une **phrase** et non une
    // étiquette : « La Bible ONT » seul faisait douze caractères sur les
    // soixante qu'un moteur affiche, et aucun des mots qu'on tape pour trouver
    // ce projet. Un titre qui ne contient pas la requête ne dit pas au lecteur
    // que le résultat est pour lui, même quand il l'est.
    //
    // Le test porte sur le **début** et non sur l'égalité : une page qui
    // s'annonce déjà comme La Bible ONT n'a pas à le redire, quelle que soit
    // la suite de sa phrase.
    let complet = if titre.starts_with("La Bible ONT") {
        titre.clone()
    } else {
        format!("{titre} — La Bible ONT")
    };

    // Le titre et la description sont du **français**, donc ils prennent la
    // composition du site.
    //
    // Le §8 bis pose la règle pour le corpus, et le §8 octies l'a apprise à ses
    // dépens : le titre d'une fiche est devenu « bara (hébreu) : orchestrer »,
    // avec une espace ordinaire devant le deux-points. `verifier-composition.py`
    // l'a refusé, et il avait raison — une espace ordinaire est un point de
    // coupure, donc un navigateur peut renvoyer le deux-points à la ligne
    // suivante dans un onglet étroit, et un moteur peut le faire dans un
    // résultat.
    //
    // La règle est posée **ici** plutôt que dans chaque page, pour la même
    // raison qu'elle l'est dans `Verset` plutôt que dans chaque appelant :
    // une règle typographique qu'il faut penser à appliquer est une règle qui
    // manquera quelque part. `composer` est la seule qui existe, et c'est ce
    // qui la garde vraie.
    let complet = composer(&complet);
    let description = composer(&description);
    let canonique = format!("{ORIGINE}{chemin}");

    // La bannière d'app, quand l'identifiant est connu.
    //
    // `app-argument` porte l'adresse de la page **courante**, pas la racine :
    // c'est elle que l'app reçoit à l'ouverture. Sans elle, quelqu'un qui lit
    // un passage et tape « OUVRIR » se retrouve sur l'écran d'accueil de l'app,
    // et doit retrouver seul l'endroit d'où il vient.
    let banniere = IDENTIFIANT_APP_STORE.map(|identifiant| {
        view! {
            <Meta
                name="apple-itunes-app"
                content=format!("app-id={identifiant}, app-argument={canonique}")
            />
        }
    });

    view! {
        <Title text=complet.clone() />
        <Meta name="description" content=description.clone() />
        <Link rel="canonical" href=canonique.clone() />
        {banniere}

        <Link rel="alternate" hreflang="fr" href=canonique.clone() />
        <Link rel="alternate" hreflang="x-default" href=canonique.clone() />

        <Meta name="robots" content="index, follow, max-image-preview:large" />

        <Meta property="og:type" content="website" />
        <Meta property="og:site_name" content="La Bible ONT" />
        <Meta property="og:locale" content="fr_FR" />
        <Meta property="og:url" content=canonique />
        <Meta property="og:title" content=complet.clone() />
        <Meta property="og:description" content=description.clone() />
        // Un PNG de 1200 × 630, composé par `scripts/images-sociales.py` :
        // c'est le format qu'attendent les messageries. Le site servait la
        // montagne seule en 512 × 512 — un carré dans un cadre paysage, que la
        // plupart rognent ou entourent de blanc. Et jamais le SVG : aucune
        // messagerie ne rend un vecteur.
        <Meta property="og:image" content=format!("{ORIGINE}{}", image("apercu.png")) />
        <Meta property="og:image:type" content="image/png" />
        <Meta property="og:image:width" content="1200" />
        <Meta property="og:image:height" content="630" />
        <Meta
            property="og:image:alt"
            content="La Bible ONT — מקרא הקדם, sur une montagne d'aubergine"
        />

        <Meta name="twitter:card" content="summary_large_image" />
        <Meta name="twitter:image" content=format!("{ORIGINE}{}", image("apercu.png")) />
        <Meta name="twitter:title" content=complet />
        <Meta name="twitter:description" content=description />
    }
}

/// Un titre et une description tiennent dans ce qu'un moteur affiche.
///
/// ## Pourquoi c'est un test et non une consigne
///
/// Un dépassement ne casse rien. La page s'affiche, se partage, s'indexe — elle
/// est seulement **coupée** dans les résultats, au milieu d'un mot, et ce qui
/// tombe est toujours la fin : c'est-à-dire l'argument, puisqu'on écrit
/// l'essentiel en dernier.
///
/// La page « Le pourquoi » portait trois cent soixante-sept signes de
/// description. La Septante et l'hellénisation — ce que la page démontre — ne
/// sont jamais arrivées à l'écran de personne. Rien ne le signalait, et
/// personne ne relit une balise qu'aucun écran ne montre.
///
/// ## Les bornes, et d'où elles viennent
///
/// Google coupe autour de **six cents pixels**, pas à un nombre de signes — la
/// mesure exacte dépend de la fonte et des lettres. Soixante signes de titre et
/// cent soixante de description sont les seuils que la profession retient, et
/// ils sont pris ici comme des **maxima** : on ne cherche pas à les atteindre.
///
/// Le titre est mesuré **suffixe compris**, puisque c'est la chaîne entière qui
/// s'affiche. C'est ce qui a fait retirer « et pourquoi elle change tout » d'un
/// titre qui, seul, paraissait court.
///
/// Le test ne voit que les littéraux. Les descriptions composées — celles d'un
/// passage, d'un livre, d'une fiche — passent par `tronquer`, qui borne à deux
/// cents signes ; c'est leur garde à elles, et elle vit dans leur code.
#[cfg(all(test, feature = "ssr"))]
mod tests {
    /// Ce qu'un moteur affiche d'un titre, suffixe du site compris.
    const TITRE_MAX: usize = 60;
    /// Ce qu'un moteur affiche d'une description.
    const DESCRIPTION_MAX: usize = 160;
    /// En deçà, une description ne dit rien qu'un moteur puisse préférer au
    /// texte de la page — il la remplace alors par un extrait pris au hasard.
    const DESCRIPTION_MIN: usize = 70;

    /// Relève la valeur d'un attribut littéral, continuations de ligne comprises.
    ///
    /// Un littéral Rust coupé par un `\` en fin de ligne reprend après
    /// l'indentation : recoller les morceaux sans replier ces blancs donnerait
    /// une longueur fausse — plus grande que la vraie, donc un test qui échoue
    /// sur des pages correctes.
    fn valeur(source: &str, attribut: &str) -> Option<String> {
        let debut = source.find(&format!("{attribut}=\""))? + attribut.len() + 2;
        let reste = &source[debut..];

        let mut valeur = String::new();
        let mut caracteres = reste.chars().peekable();
        while let Some(c) = caracteres.next() {
            match c {
                '"' => return Some(valeur),
                '\\' => {
                    // Une continuation : on saute le retour et l'indentation.
                    if caracteres.peek() == Some(&'\n') {
                        caracteres.next();
                        while caracteres.peek().is_some_and(|c| *c == ' ') {
                            caracteres.next();
                        }
                    } else if let Some(echappe) = caracteres.next() {
                        valeur.push(echappe);
                    }
                }
                _ => valeur.push(c),
            }
        }
        None
    }

    #[test]
    fn les_titres_et_descriptions_tiennent_dans_un_resultat() {
        let pages = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/interface/pages");
        let mut vus = 0;

        for entree in std::fs::read_dir(&pages).expect("le dossier des pages") {
            let chemin = entree.expect("une entrée").path();
            if chemin.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let source = std::fs::read_to_string(&chemin).expect("une page");
            let nom = chemin.file_name().unwrap().to_string_lossy().to_string();

            for morceau in source.split("<Tete").skip(1) {
                let bloc = morceau.split("/>").next().unwrap_or(morceau);

                // Une page qui se retire des index n'a rien à calibrer.
                //
                // Les pages d'absence — « Fiche introuvable », « Passage
                // introuvable » — portent `noindex` : leur description ne
                // paraîtra devant personne, et lui imposer soixante-dix signes
                // reviendrait à étoffer une phrase que le test seul lira. Le
                // `noindex` se pose juste après le `<Tete>`, d'où le regard sur
                // ce qui suit plutôt que sur le bloc lui-même.
                let suite: String = morceau.chars().take(400).collect();
                if suite.contains("noindex") {
                    continue;
                }

                if let Some(titre) = valeur(bloc, "titre") {
                    vus += 1;
                    // Le suffixe n'est pas ajouté quand le titre porte déjà le nom.
                    let complet = if titre.starts_with("La Bible ONT") {
                        titre.chars().count()
                    } else {
                        titre.chars().count() + " — La Bible ONT".chars().count()
                    };
                    assert!(
                        complet <= TITRE_MAX,
                        "{nom} : le titre « {titre} » fait {complet} signes une fois suffixé, \
                         donc au-delà de {TITRE_MAX} — un moteur le coupera, et c'est la fin \
                         qui tombe"
                    );
                }

                if let Some(description) = valeur(bloc, "description") {
                    let long = description.chars().count();
                    assert!(
                        long <= DESCRIPTION_MAX,
                        "{nom} : la description fait {long} signes, au-delà de \
                         {DESCRIPTION_MAX} — la fin sera coupée, et c'est là qu'on écrit \
                         l'argument"
                    );
                    assert!(
                        long >= DESCRIPTION_MIN,
                        "{nom} : la description fait {long} signes, en deçà de \
                         {DESCRIPTION_MIN} — un moteur la jugera trop maigre et lui \
                         préférera un extrait pris dans la page"
                    );
                }
            }
        }

        assert!(
            vus >= 8,
            "seulement {vus} titres relevés — le relevé est cassé"
        );
    }
}
