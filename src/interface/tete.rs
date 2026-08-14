use leptos::prelude::*;
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
    "/fr/le-pourquoi",
    "/fr/ce-que-l-ont-n-est-pas",
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
    let complet = if titre == "La Bible ONT" {
        titre.clone()
    } else {
        format!("{titre} — La Bible ONT")
    };
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
