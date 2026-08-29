//! Le design system — la forme, sans le propos.
//!
//! Un composant par fichier, et un fichier par composant. Chacun ignore ce
//! qu'il affiche : `Bloc` ne sait pas ce qu'il contient, `Portrait` ne sait pas
//! qui il montre. C'est ce qui permet de les composer sans les modifier — et
//! c'est la seule discipline qui empêche un design system de devenir une
//! collection de cas particuliers.
//!
//! ## Rien ne reste ici qui ne serve
//!
//! Un composant que plus aucune page n'emploie est retiré, pas conservé « au
//! cas où ». Sinon quelqu'un le retrouve six mois plus tard, le croit partie du
//! système, et bâtit dessus. `TitreDePage`, `Filet` et `Mention` sont sortis
//! comme ça, chacun après une refonte qui les avait rendus inutiles.
//!
//! Depuis Tailwind, la forme d'un composant vit **dans son fichier**, à côté de
//! son balisage : il n'y a plus de feuille parallèle qui puisse diverger. Les
//! valeurs, elles, ne s'écrivent jamais ici — couleurs, fontes, échelle et
//! mesure viennent des jetons de `style/main.css`.
//!
//! ## La mise en page
//!
//! `Hero` ouvre, `Bloc` fait tout le reste. Une seule primitive de mise en
//! page : deux finissent toujours par diverger sur un espacement, et personne
//! ne sait plus laquelle fait foi.

mod bloc;
mod blocs;
mod bouton;
mod carte_verset;
mod chiffres;
mod chronologie;
mod citation;
mod comparaison;
mod correspondances;
mod entete;
mod exergue;
mod hero;
mod image;
mod legende_niveaux;
mod lien;
mod liste_affirmations;
mod liste_unites;
mod marques;
mod mention_brouillon;
mod occurrences;
mod page_de_lecture;
mod page_legale;
mod pied;
mod porte;
mod portrait;
mod principe;
mod reglages_de_lecture;
pub mod selection_de_versets;
mod sommaire;
mod titre_de_section;
pub mod verset;

pub use bloc::Bloc;
pub use blocs::Blocs;
pub use bouton::Bouton;
pub use carte_verset::CarteVersetDuJour;
pub use chiffres::Chiffres;
pub use chronologie::{Chronologie, Jalon, Titre};
pub use citation::Citation;
pub use comparaison::Comparaison;
pub use correspondances::{Correspondance, Correspondances};
pub use entete::Entete;
pub use exergue::Exergue;
pub use hero::Hero;
pub use image::image;
pub use legende_niveaux::LegendeNiveaux;
pub use lien::Lien;
pub use liste_affirmations::ListeAffirmations;
pub use liste_unites::{nom_d_unite, ListeDUnites};
pub use marques::{Nom, Terme};
pub use mention_brouillon::MentionBrouillon;
pub use occurrences::Occurrences;
pub use page_de_lecture::PageDeLecture;
pub use page_legale::PageLegale;
pub use pied::PiedDePage;
pub use porte::{traverser, Porte};
pub use portrait::Portrait;
pub use principe::Principe;
pub use reglages_de_lecture::{fournir_preferences, preferences, ReglagesDeLecture};
pub use selection_de_versets::{
    basculer, couleur_du_verset, fournir_marques, fournir_selection, marques, renvoi, selection,
    BarreDeSelection, Marques, Selection,
};
pub use sommaire::Sommaire;
pub use titre_de_section::TitreDeSection;
pub use verset::Verset;

#[cfg(test)]
mod tests {
    /// La luminance relative d'une couleur, au sens de WCAG 2.
    fn luminance((r, g, b): (f64, f64, f64)) -> f64 {
        fn canal(c: f64) -> f64 {
            let c = c / 255.0;
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        0.2126 * canal(r) + 0.7152 * canal(g) + 0.0722 * canal(b)
    }

    fn contraste(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
        let (x, y) = (luminance(a), luminance(b));
        (x.max(y) + 0.05) / (x.min(y) + 0.05)
    }

    /// Relève un jeton `--color-<nom>: #rrggbb;` dans la feuille.
    ///
    /// Volontairement littéral : il **ne suit pas** un `color-mix` ni une
    /// variable qui en référencerait une autre. Un jeton écrit autrement n'est
    /// pas mesuré en silence — il fait échouer le relevé, et c'est ce qu'on
    /// veut d'une garde.
    fn jeton(feuille: &str, nom: &str) -> (f64, f64, f64) {
        let motif = format!("--color-{nom}:");
        let ligne = feuille
            .lines()
            .find(|l| l.trim_start().starts_with(&motif))
            .unwrap_or_else(|| panic!("le jeton --color-{nom} n'existe pas dans style/main.css"));
        let valeur = ligne
            .split_once(':')
            .expect("un jeton porte un deux-points")
            .1
            // Le commentaire de fin de ligne fait partie de la ligne : sans
            // cette coupe, la valeur relevée serait « #cfc5b9; /* le corps … */ ».
            .split(';')
            .next()
            .expect("une ligne non vide rend au moins un fragment")
            .trim();
        let hexa = valeur.strip_prefix('#').unwrap_or_else(|| {
            panic!("--color-{nom} vaut « {valeur} » — la garde ne lit qu'un littéral hexadécimal")
        });
        assert_eq!(
            hexa.len(),
            6,
            "--color-{nom} doit s'écrire sur six chiffres"
        );
        let c = |i: usize| i64::from_str_radix(&hexa[i..i + 2], 16).expect("hexadécimal") as f64;
        (c(0), c(2), c(4))
    }

    /// ## Aucune couleur de texte ne descend sous le plancher de la rampe
    ///
    /// Le site tient **6,5:1** — c'est la valeur de `encre-douce` et celle de
    /// `accentuation`, et le commentaire de la feuille dit que la seconde a été
    /// « remontée en clarté à teinte constante » jusque-là. Ce n'est pas AA,
    /// qui s'arrête à 4,5 : c'est le plancher que ce site s'est donné.
    ///
    /// **Rien ne le tenait.** La couche des Shemot est arrivée avec `#A3704D`,
    /// une valeur mesurée sur le fond sombre de l'app — juste là-bas, et fausse
    /// ici : 4,60:1 sur cette nuit d'aubergine, et 4,27:1 sur une `surface`,
    /// donc sous AA même. Une couleur se relève au calcul, pas à l'œil, et une
    /// valeur qui traverse un dépôt doit être remesurée sur le fond d'arrivée.
    ///
    /// Les deux fonds comptent, et il faut les deux : la page se lit sur
    /// `nuit`, mais un verset **désigné** par un lien partagé prend `surface`.
    /// Une couleur qui ne tiendrait que sur le premier serait juste partout
    /// sauf sur la page qu'on partage.
    #[test]
    fn aucune_couleur_de_texte_ne_descend_sous_le_plancher_de_la_rampe() {
        const FEUILLE: &str = include_str!("../../../style/main.css");
        const PLANCHER: f64 = 6.4;
        const PLANCHER_SUR_SURFACE: f64 = 4.5;

        let nuit = jeton(FEUILLE, "nuit");
        let surface = jeton(FEUILLE, "surface");

        let mut fautes = Vec::new();
        for nom in [
            "encre",
            "encre-vive",
            "encre-douce",
            "accentuation",
            "accent",
            "or",
            "shem",
        ] {
            let teinte = jeton(FEUILLE, nom);
            let sur_nuit = contraste(teinte, nuit);
            let sur_surface = contraste(teinte, surface);
            if sur_nuit < PLANCHER {
                fautes.push(format!(
                    "  {nom} : {sur_nuit:.2}:1 sur la nuit, plancher {PLANCHER}"
                ));
            }
            if sur_surface < PLANCHER_SUR_SURFACE {
                fautes.push(format!(
                    "  {nom} : {sur_surface:.2}:1 sur une surface, plancher {PLANCHER_SUR_SURFACE}"
                ));
            }
        }
        assert!(
            fautes.is_empty(),
            "des couleurs de texte passent sous le plancher :\n{}",
            fautes.join("\n")
        );
    }
}
