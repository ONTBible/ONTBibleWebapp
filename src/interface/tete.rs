use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

/// L'origine publique du site.
///
/// Les adresses canoniques et Open Graph doivent être **absolues** : une
/// messagerie qui prépare un aperçu ne connaît pas le chemin d'où vient la
/// page, elle ne connaît que ce qu'on lui donne.
pub const ORIGINE: &str = "https://ontbible.com";

/// Les pages du site, pour le plan de site.
///
/// Cette liste est **la** source : `main.rs` la lit pour composer
/// `/sitemap.xml`. Quand une route s'ajoute dans `app.rs`, elle s'ajoute ici —
/// sinon un moteur ne la trouvera jamais, et rien ne le signalera.
///
/// La page d'erreur n'y figure pas : elle porte un `noindex`.
pub const PAGES: &[&str] = &[
    "/fr",
    "/fr/le-pourquoi",
    "/fr/ce-que-l-ont-n-est-pas",
    "/fr/l-auteur",
    "/fr/confidentialite",
    "/fr/conditions",
];

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

    view! {
        <Title text=complet.clone() />
        <Meta name="description" content=description.clone() />
        <Link rel="canonical" href=canonique.clone() />

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
        <Meta property="og:image" content=format!("{ORIGINE}/images/apercu.png") />
        <Meta property="og:image:type" content="image/png" />
        <Meta property="og:image:width" content="1200" />
        <Meta property="og:image:height" content="630" />
        <Meta
            property="og:image:alt"
            content="La Bible ONT — מקרא הקדם, sur une montagne d'aubergine"
        />

        <Meta name="twitter:card" content="summary_large_image" />
        <Meta name="twitter:image" content=format!("{ORIGINE}/images/apercu.png") />
        <Meta name="twitter:title" content=complet />
        <Meta name="twitter:description" content=description />
    }
}
