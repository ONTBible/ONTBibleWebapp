use leptos::prelude::*;

use crate::domaine::texte::{Noeud, Verset as VersetDomaine};

/// Un verset de l'ONT, avec ses trois niveaux.
///
/// C'est le composant le plus important du site : il est la seule chose qui
/// montre ce que la traduction fait. Le décrire ne suffit pas — il faut le
/// voir sur un vrai verset.
///
/// Deux marquages, deux promesses, et il ne faut jamais les confondre :
/// **l'or promet une fiche** et le lien la tient ; le bordeaux clair marque un
/// terme important et ne promet rien, donc il n'est pas cliquable. Un bordeaux
/// cliquable mentirait ; un or inerte trahirait.
#[component]
pub fn Verset(verset: VersetDomaine) -> impl IntoView {
    view! {
        <p class="font-corps text-lg leading-loose text-pretty">
            <span
                aria-hidden="true"
                class="me-[0.35em] align-[0.55em] text-[0.62em] text-accent"
            >
                {verset.numero}
            </span>
            {rendre(&verset.noeuds)}
        </p>
    }
}

fn rendre(noeuds: &[Noeud]) -> Vec<AnyView> {
    noeuds.iter().map(rendre_un).collect()
}

fn rendre_un(noeud: &Noeud) -> AnyView {
    match noeud {
        Noeud::Texte(t) => t.clone().into_any(),

        Noeud::Intraduisible { mot, lemme } => view! {
            <a
                href=format!("/fr/lexique/{lemme}")
                class="font-semibold text-accent decoration-accent/40"
            >
                {mot.clone()}
            </a>
        }
        .into_any(),

        Noeud::Important(mot) => view! {
            <b class="font-semibold text-important">{mot.clone()}</b>
        }
        .into_any(),

        // 0,86 × le corps, et l'encre à 62 % : les deux valeurs de
        // `ONTTypography`. Une glose composée autrement ici ferait deux
        // niveaux 2 différents entre le site et l'app.
        Noeud::Glose(enfants) => view! {
            <span class="text-[0.86em] italic text-encre-douce">
                "["{rendre(enfants)}"]"
            </span>
        }
        .into_any(),

        // L'hébreu s'écrit de droite à gauche au milieu d'une phrase française.
        // Sans isolation, la ponctuation qui le suit part du mauvais côté — un
        // point de fin de phrase se retrouve devant le mot. `dir="rtl"` sur
        // l'élément suffit à isoler la séquence.
        Noeud::Hebreu {
            translitteration,
            hebreu,
        } => view! {
            <span class="text-[0.86em] text-encre-douce">
                "("<i>{translitteration.clone()}</i>
                " / "
                // 1,08 — `ONTFonts.hebrewScale`. L'hébreu compose plus petit
                // que le latin à taille égale : sans cette correction, les deux
                // écritures ne s'accordent pas sur une même ligne.
                <span dir="rtl" lang="he" class="font-hebreu text-[1.08em] not-italic">
                    {hebreu.clone()}
                </span>")"
            </span>
        }
        .into_any(),

        Noeud::Emphase(enfants) => view! { <em>{rendre(enfants)}</em> }.into_any(),
    }
}
