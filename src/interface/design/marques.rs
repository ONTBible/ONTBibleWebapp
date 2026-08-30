use leptos::prelude::*;

use crate::interface::design::Lien;

/// Les trois niveaux, **dans la prose du site**.
///
/// ## Pourquoi ces composants existent
///
/// Le corpus porte ses marques dans le `.md` du vault : `**mot**` devient un
/// intraduisible, `==Nom==` un nom propre, et [`crate::interface::design::verset`]
/// les rend en or et en bordeaux. La prose du site, elle, est écrite en
/// littéraux Rust — elle ne traverse **aucun** de ces mécanismes.
///
/// La page « Le pourquoi » l'a montré : on venait de baliser 1 898 noms propres
/// dans le vault, et « Yerushalayim » restait en encre ordinaire sur la page qui
/// explique justement pourquoi il est laissé en hébreu. Les intraduisibles y
/// étaient en `<b>` — du gras, pas de l'or.
///
/// C'est le défaut du §8 bis rejoué à l'identique : `composer` ne voyait que
/// les nœuds du corpus, et la prose du site lui échappait. **Une règle qui ne
/// vaut que pour le corpus ne vaut qu'à moitié**, puisque le lecteur ne sait pas
/// quelle partie de la page vient d'où.
///
/// ## Ils citent le corpus, ils ne le redéfinissent pas
///
/// Les classes sont exactement celles de `verset.rs`. Le jour où l'or change de
/// valeur, les deux changent ensemble — il n'y a pas de seconde table de
/// correspondance à tenir d'accord.
///
/// Un mot d'or **promet une fiche**. `Terme` la tient quand le lemme existe, et
/// se contente de la couleur sinon — `musar` et `paideia` n'ont pas d'entrée au
/// glossaire, et un lien vers une fiche absente vaut moins qu'aucun lien.
#[component]
pub fn Terme(
    /// Le lemme de la fiche. Absent : le mot reste en or, sans lien.
    #[prop(optional, into)]
    lemme: Option<String>,
    children: Children,
) -> impl IntoView {
    match lemme {
        Some(lemme) => view! {
            <Lien href=format!("/fr/lexique/{lemme}")>
                <span class="font-semibold text-accent">{children()}</span>
            </Lien>
        }
        .into_any(),
        None => view! { <b class="font-semibold text-accent">{children()}</b> }.into_any(),
    }
}

/// Un nom propre hébreu — bordeaux, inerte.
///
/// Même rendu que `Noeud::Accentuation`, parce que c'est la même chose : depuis la
/// généralisation du §2.5 bis, le vault balise tout nom propre en `==Nom==`.
///
/// ## Les noms hébreux seulement
///
/// Platon, Jérôme, Alexandre et Ptolémée restent en encre. La marque existe
/// parce qu'un lecteur français ne reconnaît pas Mitsrayim comme un lieu ni
/// Yason comme une personne — elle répond à une difficulté de lecture, pas à
/// une catégorie grammaticale. « Platon » n'en pose aucune, et le colorer
/// diluerait le signal jusqu'à le rendre muet.
#[component]
pub fn Nom(children: Children) -> impl IntoView {
    view! { <b class="font-semibold text-accentuation">{children()}</b> }
}

/// Chaque `lemme` posé dans une page mène à une fiche qui existe.
///
/// `Terme` promet l'or, et l'or promet une fiche — c'est la règle du §5. Un
/// lemme mal orthographié ou retiré du glossaire donnerait un mot d'or menant
/// à un 404 : la page reste belle, le lien reste cliquable, et il ne mène nulle
/// part. Personne ne clique sur les vingt-et-un mots d'or d'un site pour
/// vérifier.
///
/// Le test lit les pages **telles qu'elles sont écrites** plutôt qu'une liste
/// tenue à la main : un lemme ajouté demain y entre sans que personne n'y
/// pense.
#[cfg(all(test, feature = "ssr"))]
mod tests {
    use std::collections::HashSet;

    use crate::application::ports::Lexique;
    use crate::infrastructure::corpus::LexiqueEmbarque;

    #[test]
    fn chaque_lemme_cite_par_une_page_a_sa_fiche() {
        let lexique = LexiqueEmbarque::charger().expect("le lexique s'ouvre");

        let mut cites: HashSet<String> = HashSet::new();
        let pages = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/interface/pages");
        for entree in std::fs::read_dir(&pages).expect("le dossier des pages") {
            let chemin = entree.expect("une entrée").path();
            if chemin.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let source = std::fs::read_to_string(&chemin).expect("une page");
            for morceau in source.split("lemme=\"").skip(1) {
                if let Some(lemme) = morceau.split('"').next() {
                    cites.insert(lemme.to_string());
                }
            }
        }

        assert!(
            !cites.is_empty(),
            "aucun lemme trouvé — le relevé est cassé"
        );

        for lemme in &cites {
            assert!(
                lexique.entree(lemme).is_some(),
                "une page pose <Terme lemme=\"{lemme}\"> — ce lemme n'a pas de fiche, \
                 donc l'or promet une explication qu'il ne tient pas"
            );
        }
    }

    /// Une fiche qui existe **dit** quelque chose.
    ///
    /// ## Ce que le test d'à côté ne demandait pas
    ///
    /// `chaque_lemme_cite_par_une_page_a_sa_fiche` vérifie qu'un mot d'or **mène**
    /// quelque part. Il ne vérifie pas que ce quelque part porte un texte.
    ///
    /// Le trou n'est pas théorique. La session du vault a trouvé le 29 août 2026
    /// **cinq intraduisibles** — `neshamah`, `emunah`, `tsadiq`, `tsedaqah`,
    /// `mabbul` — déclarés, balisés dans tout le corpus, rendus en or et
    /// touchables ici, et **sans aucune définition**. Les trois compteurs du
    /// pipeline restaient au vert : il demande si un terme a une fiche, jamais
    /// si cette fiche a un contenu.
    ///
    /// Trois gardes — les deux du site et celle du pipeline — et aucune ne posait
    /// la bonne question. Ce qui les réunit : **on vérifie l'existence du lien,
    /// jamais la substance de la cible.** C'est plus facile à écrire, et c'est ce
    /// qui reste faux.
    ///
    /// Celle-ci double le contrôle du pipeline par un autre chemin. Deux gardes
    /// indépendantes valent mieux qu'une bonne : elles ne se trompent pas
    /// ensemble.
    #[test]
    fn chaque_fiche_du_lexique_porte_une_definition() {
        let lexique = LexiqueEmbarque::charger().expect("le lexique s'ouvre");

        // **Un relevé vide passe aussi**, et c'est le piège qu'on se renvoie
        // depuis une semaine : sans ce contrôle, un lexique qui ne se chargerait
        // pas rendrait zéro fiche vide, donc un test vert sur une mesure qui n'a
        // pas eu lieu.
        assert!(
            lexique.entrees().len() > 50,
            "seulement {} fiches lues — le relevé est cassé, pas le lexique",
            lexique.entrees().len()
        );

        let vides: Vec<&str> = lexique
            .entrees()
            .iter()
            .filter(|e| e.definition.is_empty())
            .map(|e| e.lemme.as_str())
            .collect();

        assert!(
            vides.is_empty(),
            "{} fiche(s) du lexique n'ont aucune définition : {vides:?}\n\
             Le mot est rendu en or, il est touchable, il mène à une page — et \
             cette page ne dit rien. C'est pire qu'un lemme absent, qui rougirait \
             dans le test d'à côté.",
            vides.len()
        );
    }
}
