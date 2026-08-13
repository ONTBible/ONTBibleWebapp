use leptos::prelude::*;

use crate::domaine::corpus::Bloc as BlocDeTexte;
use crate::domaine::texte::Noeud;
use crate::interface::design::verset::rendre_noeuds;
use crate::interface::design::Verset;

/// Le corps d'un chapitre ou d'une fiche — tous les blocs que le pipeline sait
/// produire.
///
/// C'est le pendant de [`Verset`] un cran au-dessus : celui-ci rend un fragment
/// de texte, celui-là rend la structure qui le porte. Les deux ensemble
/// couvrent tout le corpus, et ils sont les seuls à le faire — une page qui
/// composerait des versets elle-même finirait par les composer autrement.
///
/// ## Les versets désignés par un lien
///
/// `en_avant` porte les numéros que l'adresse a désignés — le `?v=1-3` d'un
/// lien partagé. Ils reçoivent un filet d'or dans la marge et une surface à
/// peine relevée. Rien de plus : c'est un **repère**, pas une mise en
/// évidence. Quelqu'un qui reçoit le lien doit trouver le passage sans que le
/// reste du chapitre cesse d'être lisible — sinon on lui envoie un surligneur
/// au lieu d'un texte.
///
/// Le filet est dans la marge et non autour du verset : un cadre ferait une
/// boîte, et une boîte se voit plus que ce qu'elle contient. C'est la leçon
/// déjà tirée sur le portrait de la page de l'auteur.
#[component]
pub fn Blocs(
    blocs: Vec<BlocDeTexte>,
    /// Les numéros de versets que l'adresse a désignés.
    #[prop(optional)]
    en_avant: Vec<u32>,
) -> impl IntoView {
    blocs
        .into_iter()
        .map(|bloc| rendre_bloc(bloc, &en_avant))
        .collect_view()
}

fn rendre_bloc(bloc: BlocDeTexte, en_avant: &[u32]) -> AnyView {
    match bloc {
        BlocDeTexte::Versets(versets) => versets
            .into_iter()
            .map(|verset| {
                let designe = en_avant.contains(&verset.numero);
                let ancre = format!("v{}", verset.numero);
                view! {
                    <div
                        id=ancre
                        class="-mx-4 rounded-sm px-4 scroll-mt-24 transition-colors"
                        class=("border-s-2", designe)
                        class=("border-accent", designe)
                        class=("bg-surface/60", designe)
                        class=("ps-5", designe)
                    >
                        <Verset verset />
                    </div>
                }
                .into_any()
            })
            .collect_view()
            .into_any(),

        // Le niveau vient du nombre de « # » du Markdown. On le décale d'un
        // cran : le `h1` de la page est le titre du chapitre, donc un `##` du
        // vault est un `h2` ici et la hiérarchie du document reste juste —
        // c'est elle que suit un lecteur d'écran pour parcourir la page.
        BlocDeTexte::Titre { niveau, noeuds } => {
            let contenu = rendre(&noeuds);
            let classe = "mt-16 mb-6 text-encre-vive first:mt-0";
            match niveau {
                0..=2 => view! { <h2 class=classe>{contenu}</h2> }.into_any(),
                3 => view! { <h3 class=classe>{contenu}</h3> }.into_any(),
                _ => view! { <h4 class=classe>{contenu}</h4> }.into_any(),
            }
        }

        BlocDeTexte::Liste { ordonnee, items } => {
            let entrees = items
                .into_iter()
                .map(|noeuds| view! { <li class="mb-2">{rendre(&noeuds)}</li> })
                .collect_view();
            if ordonnee {
                view! { <ol class="mb-8 list-decimal ps-6 text-encre">{entrees}</ol> }.into_any()
            } else {
                view! { <ul class="mb-8 list-disc ps-6 text-encre">{entrees}</ul> }.into_any()
            }
        }

        BlocDeTexte::Paragraphe(noeuds) => {
            view! { <p class="mb-6 text-pretty">{rendre(&noeuds)}</p> }.into_any()
        }

        // Une citation détachée : un filet d'or dans la marge, du retrait, et
        // rien d'autre. Pas de guillemets dessinés — la ponctuation suspendue
        // du site les jetterait dans la marge par-dessus le filet.
        BlocDeTexte::Citation(noeuds) => view! {
            <blockquote class="my-10 border-s-2 border-or/30 ps-6 italic text-encre-douce">
                {rendre(&noeuds)}
            </blockquote>
        }
        .into_any(),

        // Le tableau déborde par le bas plutôt que d'élargir la page : sur un
        // téléphone, trois colonnes de prose ne tiennent pas dans la mesure, et
        // c'est le corps de la page qui partirait en défilement horizontal.
        BlocDeTexte::Tableau { entetes, lignes } => view! {
            <div class="my-10 -mx-6 overflow-x-auto px-6">
                <table class="chiffres-tableau w-full min-w-lg border-collapse text-[0.94em]">
                    <thead>
                        <tr>
                            {entetes
                                .into_iter()
                                .map(|cellule| {
                                    view! {
                                        <th class="border-b border-filet pb-3 pe-6 text-start text-sm font-normal uppercase tracking-capitales text-accent last:pe-0">
                                            {rendre(&cellule)}
                                        </th>
                                    }
                                })
                                .collect_view()}
                        </tr>
                    </thead>
                    <tbody>
                        {lignes
                            .into_iter()
                            .map(|ligne| {
                                view! {
                                    <tr class="align-baseline">
                                        {ligne
                                            .into_iter()
                                            .map(|cellule| {
                                                view! {
                                                    <td class="border-b border-filet/40 py-4 pe-6 last:pe-0">
                                                        {rendre(&cellule)}
                                                    </td>
                                                }
                                            })
                                            .collect_view()}
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        }
        .into_any(),

        // Un filet centré et court, pas une règle d'un bord à l'autre : c'est
        // le signe d'une pause dans un livre, pas la fin d'une section.
        BlocDeTexte::Filet => view! {
            <hr
                aria-hidden="true"
                class="mx-auto my-14 h-px w-16 border-0 bg-accent opacity-30"
            />
        }
        .into_any(),
    }
}

/// Une suite de nœuds hors d'un verset — dans un titre, une liste, un tableau.
///
/// Elle passe par la fonction de [`crate::interface::design::verset`], et non
/// par un rendu propre à ce fichier : c'est ce qui garantit qu'un intraduisible
/// se compose **partout** de la même façon, et que l'or d'une puce de liste
/// mène à la même fiche que celui d'un verset.
fn rendre(noeuds: &[Noeud]) -> Vec<AnyView> {
    rendre_noeuds(noeuds)
}
