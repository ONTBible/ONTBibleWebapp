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
        couleur(feuille, &format!("--color-{nom}"))
    }

    fn couleur(feuille: &str, nom: &str) -> (f64, f64, f64) {
        let motif = format!("{nom}:");
        let ligne = feuille
            .lines()
            .find(|l| l.trim_start().starts_with(&motif))
            .unwrap_or_else(|| panic!("le jeton {nom} n'existe pas dans style/main.css"));
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
            panic!("{nom} vaut « {valeur} » — la garde ne lit qu'un littéral hexadécimal")
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
    /// **Rien ne le tenait.** La couche des Shemot est arrivée avec la couleur
    /// du thème sombre de l'app, `#AA7550` : 4,68:1 sur *son* fond, où elle est
    /// juste, mais 4,95:1 sur cette nuit d'aubergine et 4,60:1 sur une
    /// `surface`. Une valeur qui traverse un dépôt se remesure sur le fond
    /// d'arrivée — un fond sombre n'est pas un autre fond sombre.
    ///
    /// Et elle est arrivée **deux fois**, la première étant `#A3704D`, une
    /// version périmée que l'app avait déjà écartée. Les deux échouaient ici,
    /// donc la mesure a tranché sans qu'on ait eu besoin de savoir laquelle
    /// était la bonne. C'est ce qu'on demande à une garde : rendre le même
    /// verdict que la valeur transmise soit à jour ou non.
    ///
    /// Les deux fonds comptent, et il faut les deux : la page se lit sur
    /// `nuit`, mais un verset **désigné** par un lien partagé prend `surface`.
    /// Une couleur qui ne tiendrait que sur le premier serait juste partout
    /// sauf sur la page qu'on partage.
    ///
    /// ## D'où vient 6,4, et pourquoi ce n'est pas 4,5
    ///
    /// AA demande 4,5 pour du texte courant. Le site en exige 6,4 depuis des
    /// semaines, et **la raison n'était écrite nulle part** — or un plancher
    /// sans raison est un plancher qu'on abaisse le jour où il gêne.
    ///
    /// Elle a un nom : le **kératocône** de l'auteur. La cornée déformée
    /// diffuse la lumière ; les bords se dédoublent, les contours proches se
    /// confondent. La marge au-dessus d'AA n'est pas du zèle, c'est ce qu'il
    /// faut pour que *lui* lise sa propre traduction.
    ///
    /// Ce qui en découle dépasse le chiffre, et vaut pour toute la feuille :
    /// **distinguer deux niveaux de texte par la pente est le pire choix
    /// possible.** Un italique se lit à l'inclinaison des jambages, exactement
    /// ce qu'un halo efface. Ce qui tient est la couleur, la taille, l'air
    /// autour — et c'est mesurable, donc c'est ici que ça se garde.
    ///
    /// Trouvé le 30 août 2026 par la session iOS en portant la liseuse sur
    /// Mac. La règle valait pour les trois dépôts et n'était écrite dans aucun.
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

    /// Compose une couleur sur un fond, à une opacité donnée.
    fn poser(dessus: (f64, f64, f64), dessous: (f64, f64, f64), alpha: f64) -> (f64, f64, f64) {
        let m = |d: f64, s: f64| (d * alpha + s * (1.0 - alpha)).round();
        (
            m(dessus.0, dessous.0),
            m(dessus.1, dessous.1),
            m(dessus.2, dessous.2),
        )
    }

    /// ## Le surlignage enfonce toutes les couleurs, et c'est une dette
    ///
    /// Un verset surligné reçoit une des cinq couleurs à `--surlignage-opacite`,
    /// soit 0,38. Le fond devient alors clair, et **aucune** couleur de texte du
    /// site n'y tient : le corps tombe à 4,01:1, l'or à 3,67, et les trois
    /// marquages du corpus autour de 2,3.
    ///
    /// Ça date des surlignages et non de la couche des Shemot, qui n'a fait que
    /// l'éclairer. La correction n'est pas un jeton à retoucher — ce sont les
    /// six couleurs à la fois, donc soit l'opacité, soit un marquage qui
    /// s'ajusterait au fond réel sous lui. C'est un chantier, pas un correctif.
    ///
    /// **Ce test ne l'exige donc pas résolu ; il l'empêche d'empirer.** Les
    /// valeurs ci-dessous sont des dettes relevées, pas des cibles. Une garde
    /// qui poserait le plancher de la rampe échouerait dès aujourd'hui, et une
    /// garde rouge en permanence cesse d'être lue en une semaine.
    ///
    /// Il serre dans les deux sens : une valeur qui **s'améliore** le fait
    /// échouer aussi, pour que la dette inscrite descende avec le défaut. Sans
    /// ça un cliquet se desserre tout seul — on corrige à moitié, la table garde
    /// l'ancien chiffre, et la moitié gagnée peut être reperdue en silence.
    ///
    /// Le fond retenu est le **pire des deux** : la nuit, et la `surface` d'un
    /// verset désigné, qu'un lien partagé peut cumuler avec un surlignage.
    #[test]
    fn le_surlignage_n_enfonce_pas_les_couleurs_plus_qu_aujourd_hui() {
        const FEUILLE: &str = include_str!("../../../style/main.css");
        /// Ce qu'on tolère d'écart avant de demander la mise à jour de la table.
        const JEU: f64 = 0.05;

        let dettes = [
            ("encre", 4.01),
            ("encre-vive", 5.38),
            ("encre-douce", 2.29),
            ("accentuation", 2.30),
            ("accent", 3.67),
            ("shem", 2.29),
        ];

        let opacite: f64 = FEUILLE
            .lines()
            .find_map(|l| l.trim_start().strip_prefix("--surlignage-opacite:"))
            .expect("--surlignage-opacite doit exister")
            .split(';')
            .next()
            .expect("une ligne non vide rend au moins un fragment")
            .trim()
            .parse()
            .expect("l'opacité doit être un nombre");

        let fonds = [jeton(FEUILLE, "nuit"), jeton(FEUILLE, "surface")];
        let surlignages = ["or", "olive", "ciel", "rose", "violet"]
            .map(|n| couleur(FEUILLE, &format!("--surlignage-{n}")));

        let mut ecarts = Vec::new();
        for (nom, dette) in dettes {
            let teinte = jeton(FEUILLE, nom);
            let pire = surlignages
                .iter()
                .flat_map(|s| {
                    fonds
                        .iter()
                        .map(|f| contraste(teinte, poser(*s, *f, opacite)))
                })
                .fold(f64::INFINITY, f64::min);

            if pire < dette - JEU {
                ecarts.push(format!(
                    "  {nom} : {pire:.2}:1, alors que la dette inscrite est {dette:.2} — ça a empiré"
                ));
            } else if pire > dette + JEU {
                ecarts.push(format!(
                    "  {nom} : {pire:.2}:1, mieux que la dette inscrite ({dette:.2}) — \
                     descends-la à {pire:.2} pour que le cliquet tienne le gain"
                ));
            }
        }
        assert!(
            ecarts.is_empty(),
            "le contraste sous un surlignage a bougé :\n{}",
            ecarts.join("\n")
        );
    }
}
