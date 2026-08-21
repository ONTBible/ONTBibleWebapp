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

        assert!(!cites.is_empty(), "aucun lemme trouvé — le relevé est cassé");

        for lemme in &cites {
            assert!(
                lexique.entree(lemme).is_some(),
                "une page pose <Terme lemme=\"{lemme}\"> — ce lemme n'a pas de fiche, \
                 donc l'or promet une explication qu'il ne tient pas"
            );
        }
    }
}
