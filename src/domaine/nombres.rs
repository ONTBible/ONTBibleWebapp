//! Écrire un nombre en toutes lettres.
//!
//! ## Pourquoi ça existe
//!
//! Trois pages du site annonçaient « **Trois** livres sur soixante-dix » en
//! dur, alors que le vault en porte cinq. Le compte était faux sur ce que le
//! lecteur lit en premier — et `design/chiffres.rs` portait déjà
//! l'avertissement, à quelques lignes de là : *« un site qui annonce trois
//! livres quand le vault en a cinq ment sans que personne ne le remarque »*.
//!
//! **La règle était écrite, la garde existait, et le texte est resté en dur.**
//! Parce qu'on ne savait pas écrire un nombre variable en prose : `Chiffres`
//! rend des chiffres dans une grille, ce qui est juste pour un tableau de bord
//! et faux au milieu d'une phrase. Écrire « 5 livres sur 70 » aurait corrigé
//! le mensonge en abîmant la composition, et personne n'a tranché.
//!
//! Il manquait donc **vingt lignes**, pas une décision.

/// Le nombre en toutes lettres, de zéro à quatre-vingt-dix-neuf.
///
/// ## Les trois irrégularités du français, et elles sont toutes là
///
/// - **soixante-dix** et non « septante » : c'est la langue de France, celle
///   du vault ;
/// - **quatre-vingts** prend son `s` **seul**, jamais suivi d'un autre nombre
///   — « quatre-vingts » mais « quatre-vingt-un ». L'accord se fait sur la
///   multiplication, pas sur l'addition ;
/// - **et** remplace le trait d'union à 21, 31, 41, 51, 61 et **71**, et à
///   nulle autre place. « Soixante et onze », mais « quatre-vingt-un ».
///
/// Au-delà de 99 le site n'en a pas besoin — le corpus compte soixante-dix
/// livres — et rendre les chiffres est alors plus honnête qu'une règle qu'on
/// n'aurait jamais l'occasion d'éprouver.
pub fn en_lettres(n: u32) -> String {
    const UNITES: [&str; 20] = [
        "zéro", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf", "dix",
        "onze", "douze", "treize", "quatorze", "quinze", "seize", "dix-sept", "dix-huit",
        "dix-neuf",
    ];
    const DIZAINES: [&str; 8] = [
        "",
        "dix",
        "vingt",
        "trente",
        "quarante",
        "cinquante",
        "soixante",
        "soixante",
    ];

    if n >= 100 {
        return n.to_string();
    }
    if n < 20 {
        return UNITES[n as usize].to_string();
    }

    // Soixante-dix et quatre-vingt-dix comptent par vingt : leur reste va
    // jusqu'à dix-neuf, pas jusqu'à neuf.
    let (base, reste) = match n {
        70..=79 => (60, n - 60),
        80..=99 => (80, n - 80),
        _ => (n / 10 * 10, n % 10),
    };

    let tete = if base == 80 {
        // Le `s` seulement quand il est seul.
        if reste == 0 {
            "quatre-vingts".to_string()
        } else {
            "quatre-vingt".to_string()
        }
    } else {
        DIZAINES[(base / 10) as usize].to_string()
    };

    if reste == 0 {
        return tete;
    }
    // « et » à 21, 31, 41, 51, 61, 71 — et nulle part ailleurs.
    let liaison = if reste == 1 && base <= 60 {
        " et "
    } else if reste == 11 && base == 60 {
        " et "
    } else {
        "-"
    };
    format!("{tete}{liaison}{}", UNITES[reste as usize])
}

/// Le même, capitale en tête — pour ouvrir une phrase.
pub fn en_lettres_capitale(n: u32) -> String {
    let mot = en_lettres(n);
    let mut lettres = mot.chars();
    match lettres.next() {
        Some(premiere) => premiere.to_uppercase().collect::<String>() + lettres.as_str(),
        None => mot,
    }
}

#[cfg(test)]
mod tests {
    use super::{en_lettres, en_lettres_capitale};

    #[test]
    fn les_petits_nombres() {
        for (n, mot) in [
            (0, "zéro"),
            (1, "un"),
            (3, "trois"),
            (5, "cinq"),
            (16, "seize"),
            (17, "dix-sept"),
            (19, "dix-neuf"),
            (20, "vingt"),
        ] {
            assert_eq!(en_lettres(n), mot, "{n}");
        }
    }

    /// **Le « et » ne vaut qu'à six places.**
    #[test]
    fn la_liaison_est_un_mot_ou_un_trait() {
        for (n, mot) in [
            (21, "vingt et un"),
            (22, "vingt-deux"),
            (31, "trente et un"),
            (61, "soixante et un"),
            (71, "soixante et onze"),
            (81, "quatre-vingt-un"),
            (91, "quatre-vingt-onze"),
        ] {
            assert_eq!(en_lettres(n), mot, "{n}");
        }
    }

    /// **Quatre-vingts prend son `s` seul**, et soixante-dix compte par vingt.
    #[test]
    fn les_deux_vigesimales() {
        for (n, mot) in [
            (70, "soixante-dix"),
            (72, "soixante-douze"),
            (79, "soixante-dix-neuf"),
            (80, "quatre-vingts"),
            (82, "quatre-vingt-deux"),
            (90, "quatre-vingt-dix"),
            (99, "quatre-vingt-dix-neuf"),
        ] {
            assert_eq!(en_lettres(n), mot, "{n}");
        }
    }

    /// Au-delà, on rend les chiffres plutôt qu'une règle non éprouvée.
    #[test]
    fn au_dela_de_cent_ce_sont_des_chiffres() {
        assert_eq!(en_lettres(100), "100");
        assert_eq!(en_lettres(163), "163");
    }

    #[test]
    fn la_capitale_ouvre_une_phrase() {
        assert_eq!(en_lettres_capitale(5), "Cinq");
        assert_eq!(en_lettres_capitale(70), "Soixante-dix");
    }
}

#[cfg(all(test, feature = "ssr"))]
mod garde {
    /// ## Aucune page n'écrit le compte du corpus à la main
    ///
    /// C'est la garde qui manquait. La **règle** était écrite — trois fois,
    /// dans `chiffres.rs`, `build.rs` et `api.rs` — et le texte est resté en
    /// dur quand même, sur la phrase que le lecteur voit en premier.
    ///
    /// Une règle qu'on énonce sans la tenir est pire qu'une règle absente :
    /// elle donne le sentiment d'avoir décidé. C'est le §5 sur le portail, et
    /// c'est ce qui s'est produit ici.
    ///
    /// Elle cherche les **nombres écrits en lettres** à côté du mot « livres »
    /// : c'est la forme exacte du défaut, et la seule qui ne lève pas de faux
    /// positifs sur les chiffres légitimes d'une glose ou d'un renvoi.
    #[test]
    fn aucune_page_ne_code_le_compte_des_livres() {
        const EN_LETTRES: [&str; 12] = [
            "un",
            "deux",
            "trois",
            "quatre",
            "cinq",
            "six",
            "sept",
            "huit",
            "neuf",
            "dix",
            "soixante-dix",
            "quatre-vingts",
        ];

        let pages = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/interface");
        let mut fautes = Vec::new();
        parcourir(&pages, &mut |chemin, source| {
            for (rang, ligne) in source.lines().enumerate() {
                let nu = ligne.trim_start();
                // Les commentaires racontent le défaut ; ils ne le commettent
                // pas. Sans cette exception, la garde accuserait sa propre
                // explication.
                if nu.starts_with("//") {
                    continue;
                }
                let bas = ligne.to_lowercase();
                if !bas.contains("livres") {
                    continue;
                }
                for mot in EN_LETTRES {
                    if bas.contains(&format!("{mot} livres")) || bas.contains(&format!("sur {mot}"))
                    {
                        fautes.push(format!(
                            "{}:{} — « {} »",
                            chemin.file_name().unwrap_or_default().to_string_lossy(),
                            rang + 1,
                            ligne.trim(),
                        ));
                    }
                }
            }
        });

        assert!(
            fautes.is_empty(),
            "le compte du corpus est écrit à la main :\n  {}\n\n\
             Il vient du pipeline, figé par `build.rs` : \
             `env!(\"CORPUS_LIVRES_ECRITS\")` et `env!(\"CORPUS_LIVRES\")`, \
             écrits en prose par `domaine::nombres::en_lettres`. \
             Un site qui annonce trois livres quand le vault en a cinq ment \
             sans que personne ne le remarque.",
            fautes.join("\n  "),
        );
    }

    fn parcourir(dossier: &std::path::Path, faire: &mut impl FnMut(&std::path::Path, &str)) {
        let Ok(entrees) = std::fs::read_dir(dossier) else {
            return;
        };
        for entree in entrees.flatten() {
            let chemin = entree.path();
            if chemin.is_dir() {
                parcourir(&chemin, faire);
            } else if chemin.extension().is_some_and(|e| e == "rs") {
                if let Ok(source) = std::fs::read_to_string(&chemin) {
                    faire(&chemin, &source);
                }
            }
        }
    }
}
