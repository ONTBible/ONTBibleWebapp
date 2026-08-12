use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

/// L'origine publique du site.
///
/// Les adresses canoniques et Open Graph doivent être **absolues** : une
/// messagerie qui prépare un aperçu ne connaît pas le chemin d'où vient la
/// page, elle ne connaît que ce qu'on lui donne.
pub const ORIGINE: &str = "https://ontbible.com";

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
        // Un PNG, jamais le SVG : aucune messagerie ne rend un vecteur dans un
        // aperçu — elles affichent un cadre vide, ce qui est pire que pas
        // d'image du tout.
        <Meta property="og:image" content=format!("{ORIGINE}/images/montagne-512.png") />

        // `summary_large_image` sans image de bonne taille donne une carte
        // vide. Tant qu'il n'y a pas d'image d'aperçu dessinée pour 1200 × 630,
        // la carte réduite est la seule qui rende quelque chose de propre.
        <Meta name="twitter:card" content="summary" />
        <Meta name="twitter:title" content=complet />
        <Meta name="twitter:description" content=description />
    }
}
