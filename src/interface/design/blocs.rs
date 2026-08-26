use leptos::prelude::*;

use crate::domaine::corpus::Bloc as BlocDeTexte;
use crate::domaine::lecture::{preparer, Preferences};
use crate::domaine::texte::Noeud;
use crate::interface::design::verset::rendre_noeuds;
use crate::interface::design::{basculer, preferences, selection, Selection, Verset};

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
/// ## Il se recompose quand les réglages changent
///
/// Le chapitre entier est reconstruit à chaque bascule, et c'est **voulu** :
/// éteindre un niveau ne masque pas des nœuds, il les **retire** de l'arbre,
/// puis referme les blancs qu'ils laissaient (voir
/// [`crate::domaine::lecture`]). Un simple `display: none` aurait donné
/// « habitant , et la face » et des mots collés — exactement le défaut qu'on
/// vient de corriger ailleurs.
///
/// Le coût est celui d'un chapitre : quarante-six versets au plus, dans le
/// navigateur, sur un geste du lecteur.
#[component]
pub fn Blocs(
    blocs: Vec<BlocDeTexte>,
    /// Les numéros de versets que l'adresse a désignés.
    #[prop(optional)]
    en_avant: Vec<u32>,
) -> impl IntoView {
    // `StoredValue` et non une capture : la vue est recalculée à chaque
    // changement de réglage, donc elle a besoin de retrouver le texte d'origine
    // — celui d'avant tout retrait. Recomposer depuis un arbre déjà élagué
    // rendrait l'extinction irréversible.
    let blocs = StoredValue::new(blocs);
    let en_avant = StoredValue::new(en_avant);
    let preferences = preferences();
    // Absente hors de la liseuse — sur l'accueil, dans une fiche. Le texte s'y
    // rend alors exactement comme avant : pas de curseur, pas de rôle, pas de
    // clic. Un verset qui aurait l'air cliquable sans l'être serait pire que
    // pas de sélection du tout.
    let choix = selection();

    move || {
        let p = preferences.get();
        blocs.with_value(|blocs| {
            en_avant.with_value(|en_avant| {
                blocs
                    .iter()
                    .map(|bloc| rendre_bloc(bloc.clone(), en_avant, p, choix))
                    .collect_view()
            })
        })
    }
}

/// Les attributs qui rendent un verset sélectionnable.
///
/// ## Pourquoi un `<span>` ou un `<div>` et non un `<button>`
///
/// Un bouton ne peut pas contenir de contenu de flux — or un verset porte des
/// liens vers les fiches du lexique, et un lien **dans** un bouton est du HTML
/// invalide que les lecteurs d'écran annoncent de travers.
///
/// D'où `role="button"` et `tabindex="0"`, qui donnent le comportement sans la
/// balise. Le prix est qu'il faut alors gérer **Entrée et Espace** à la main :
/// un vrai bouton les traite, un `role` ne les traite pas. Les oublier ferait
/// une sélection accessible à la souris et à rien d'autre.
///
/// ## Le clic ne doit pas voler celui d'un lien
///
/// Un verset contient des mots d'or qui mènent au lexique. Sans précaution, le
/// clic sur *bara* sélectionnerait le verset **et** suivrait le lien. On ignore
/// donc les clics dont la cible est un `<a>` — c'est le lien qui gagne, parce
/// que c'est le geste le plus précis des deux.
#[cfg(feature = "hydrate")]
fn cible_est_un_lien(evenement: &leptos::ev::MouseEvent) -> bool {
    use wasm_bindgen::JsCast;
    evenement
        .target()
        .and_then(|c| c.dyn_into::<web_sys::Element>().ok())
        .is_some_and(|e| e.closest("a").ok().flatten().is_some())
}

/// Les trois choses qu'un verset sélectionnable doit savoir faire.
///
/// Rendues ensemble parce qu'elles partagent le même état et qu'on les pose au
/// même endroit — les séparer ferait trois fermetures capturant chacune le
/// signal, et trois occasions d'en oublier une.
///
/// Sans sélection dans le contexte, les trois sont inertes : `choisi` rend
/// toujours `false`, et les gestes ne font rien. Le texte se rend alors comme
/// avant, ce qui est le comportement juste sur l'accueil et dans une fiche.
#[allow(clippy::type_complexity)]
fn gestes(
    choix: Option<Selection>,
    numero: u32,
) -> (
    impl Fn() -> bool + Copy + Send + Sync + 'static,
    impl Fn(leptos::ev::MouseEvent) + Clone + 'static,
    impl Fn(leptos::ev::KeyboardEvent) + Clone + 'static,
) {
    let choisi = move || {
        choix
            .map(|s| s.with(|s| s.contains(&numero)))
            .unwrap_or(false)
    };

    let au_clic = move |_evenement: leptos::ev::MouseEvent| {
        let Some(s) = choix else { return };
        // Un verset porte des mots d'or qui mènent au lexique. Sans ce garde,
        // cliquer sur *bara* sélectionnerait le verset **et** suivrait le
        // lien : le lecteur partirait sur la fiche en laissant derrière lui une
        // sélection qu'il n'a pas voulue.
        #[cfg(feature = "hydrate")]
        if cible_est_un_lien(&_evenement) {
            return;
        }
        basculer(s, numero);
    };

    let au_clavier = move |evenement: leptos::ev::KeyboardEvent| {
        let Some(s) = choix else { return };
        // `role="button"` donne l'apparence d'un bouton à un lecteur d'écran,
        // pas son comportement : un vrai `<button>` traite Entrée et Espace, un
        // rôle ne traite rien. Les omettre ferait une sélection accessible à la
        // souris et à rien d'autre — exactement le genre de manque qu'aucun
        // test de rendu ne voit.
        let touche = evenement.key();
        if touche == "Enter" || touche == " " {
            // Espace sur un élément focalisé fait défiler la page. Ici, le
            // geste est la sélection, donc le défilement serait un effet de
            // bord — et il déplacerait le verset qu'on vient de désigner.
            evenement.prevent_default();
            basculer(s, numero);
        }
    };

    (choisi, au_clic, au_clavier)
}

fn rendre_bloc(
    bloc: BlocDeTexte,
    en_avant: &[u32],
    p: Preferences,
    choix: Option<Selection>,
) -> AnyView {
    match bloc {
        // ── Les versets à la suite ────────────────────────────────────────
        //
        // Un seul paragraphe, les numéros en exposant : c'est la lecture
        // suivie. Les ancres restent posées sur chaque verset — un lien
        // partagé vers `#v6` doit tomber juste dans les deux dispositions.
        BlocDeTexte::Versets(versets) if p.continu => view! {
            // **Le corps du site, sans le grossir ni l'aérer.**
            //
            // Ce paragraphe portait `text-lg leading-loose` : une taille au
            // dessus du corps et un interligne de 2, là où le site tient 1,68
            // — mesuré au §5 pour que l'œil retrouve la ligne suivante à 21 px.
            //
            // La lecture suivie s'en trouvait **plus aérée que le mode
            // d'étude**, c'est-à-dire l'inverse de ce qu'elle promet : le texte
            // flottait au lieu de couler, et chaque verset semblait détaché de
            // celui d'avant alors qu'ils sont dans le même paragraphe.
            //
            // Rien de spécifique ici, donc : le paragraphe hérite du corps, et
            // c'est la disposition — un seul bloc, les numéros en exposant —
            // qui fait la différence entre les deux modes.
            <p class="mb-8 text-pretty">
                {versets
                    .into_iter()
                    .map(|verset| {
                        let designe = en_avant.contains(&verset.numero);
                        let ancre = format!("v{}", verset.numero);
                        let numero = verset.numero;
                        let noeuds = preparer(&verset.noeuds, p);
                        view! {
                            <span
                                id=ancre
                                class="scroll-mt-24"
                                class=("rounded-sm", designe)
                                class=("bg-surface", designe)
                                class=("box-decoration-clone", designe)
                                class=("px-1.5", designe)
                            >
                                <span
                                    aria-hidden="true"
                                    class="me-[0.3em] align-[0.55em] text-[0.62em] text-accent"
                                >
                                    {numero}
                                </span>
                                {rendre_noeuds(&noeuds)}
                                " "
                            </span>
                        }
                    })
                    .collect_view()}
            </p>
        }
        .into_any(),

        BlocDeTexte::Versets(versets) => versets
            .into_iter()
            .map(|verset| {
                let designe = en_avant.contains(&verset.numero);
                let ancre = format!("v{}", verset.numero);
                let numero = verset.numero;
                let (choisi, au_clic, au_clavier) = gestes(choix, numero);
                let verset = crate::domaine::texte::Verset {
                    numero: verset.numero,
                    noeuds: preparer(&verset.noeuds, p),
                };
                view! {
                    // Le retrait de tête est **exclusif**, pas cumulé :
                    // `px-4` et `ps-5` sur le même élément se départageraient
                    // par l'ordre de la feuille de style, pas par celui de
                    // l'attribut. Ça marche aujourd'hui et ça marchera jusqu'au
                    // jour où Tailwind rangera ses utilitaires autrement. C'est
                    // ce piège qui a fait rendre toute la comparaison de
                    // l'accueil à la mauvaise largeur — voir `bloc.rs`.
                    <div
                        id=ancre
                        class="-mx-4 rounded-sm pe-4 scroll-mt-24 transition-colors"
                        class=("ps-4", move || !designe && !choisi())
                        class=("ps-5", move || designe || choisi())
                        // Un seul `border-s-2` : deux fois le même utilitaire
                        // sur un élément est refusé par Leptos, et il a raison —
                        // à spécificité égale c'est l'ordre de la feuille qui
                        // trancherait, pas l'intention. C'est le piège de `Bloc`
                        // rejoué, attrapé cette fois par le compilateur.
                        class=("border-s-2", move || designe || choisi())
                        class=("border-accent", move || designe && !choisi())
                        class=("bg-surface/60", move || designe && !choisi())
                        // La sélection se distingue de la désignation par
                        // l'adresse : celle-ci est un fond discret, celle-là un
                        // fond d'aubergine et un filet d'or. Les deux peuvent
                        // coexister — on ouvre un lien partagé, puis on
                        // sélectionne autre chose —, donc elles ne peuvent pas
                        // partager le même signe.
                        class=("bg-aubergine/45", choisi)
                        class=("border-or/60", choisi)
                        class=("cursor-pointer", move || choix.is_some())
                        role=move || choix.is_some().then_some("button")
                        tabindex=move || choix.is_some().then_some("0")
                        aria-pressed=move || choix.is_some().then(|| choisi().to_string())
                        on:click=au_clic
                        on:keydown=au_clavier
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
            let contenu = rendre(&preparer(&noeuds, p));
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
                .map(|noeuds| view! { <li class="mb-2">{rendre(&preparer(&noeuds, p))}</li> })
                .collect_view();
            if ordonnee {
                view! { <ol class="mb-8 list-decimal ps-6 text-encre">{entrees}</ol> }.into_any()
            } else {
                view! { <ul class="mb-8 list-disc ps-6 text-encre">{entrees}</ul> }.into_any()
            }
        }

        BlocDeTexte::Paragraphe(noeuds) => {
            view! { <p class="mb-6 text-pretty">{rendre(&preparer(&noeuds, p))}</p> }.into_any()
        }

        // Une citation détachée : un filet d'or dans la marge, du retrait, et
        // rien d'autre. Pas de guillemets dessinés — la ponctuation suspendue
        // du site les jetterait dans la marge par-dessus le filet.
        BlocDeTexte::Citation(noeuds) => view! {
            <blockquote class="my-10 border-s-2 border-or/30 ps-6 italic text-encre-douce">
                {rendre(&preparer(&noeuds, p))}
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
                                            {rendre(&preparer(&cellule, p))}
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
                                                        {rendre(&preparer(&cellule, p))}
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

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use crate::domaine::texte::Verset as VersetDomaine;

    fn chapitre() -> Vec<BlocDeTexte> {
        vec![BlocDeTexte::Versets(vec![VersetDomaine {
            numero: 1,
            noeuds: vec![
                Noeud::Texte("Quand ".into()),
                Noeud::Intraduisible {
                    mot: "Elohim".into(),
                    lemme: "elohim".into(),
                },
                Noeud::Texte(" ".into()),
                Noeud::Hebreu {
                    translitteration: "elohim".into(),
                    hebreu: "אֱלֹהִים".into(),
                },
                Noeud::Texte(" ".into()),
                Noeud::Glose(vec![Noeud::Texte("nom divin laissé intact".into())]),
                Noeud::Texte(" commença.".into()),
            ],
        }])]
    }

    /// Rend le composant en HTML sous des réglages donnés.
    ///
    /// Le test passe par le **composant**, et non par `preparer` seul : c'est
    /// précisément le câblage — contexte, signal, recomposition — qui n'est
    /// éprouvé nulle part ailleurs, et c'est lui qu'un clic exerce.
    fn rendre_sous(preferences: Preferences) -> String {
        let owner = Owner::new();
        owner.with(|| {
            provide_context(RwSignal::new(preferences));
            view! { <Blocs blocs=chapitre() /> }.to_html()
        })
    }

    #[test]
    fn par_defaut_les_trois_niveaux_sont_la() {
        let html = rendre_sous(Preferences::default());
        assert!(html.contains("Elohim"), "le corps");
        assert!(html.contains("font-hebreu"), "le niveau 3");
        assert!(html.contains("nom divin"), "la glose");
    }

    #[test]
    fn eteindre_les_gloses_retire_la_glose_et_elle_seule() {
        let html = rendre_sous(Preferences {
            gloses: false,
            ..Default::default()
        });
        assert!(!html.contains("nom divin"), "la glose doit disparaître");
        assert!(html.contains("font-hebreu"), "le niveau 3 doit rester");
        assert!(html.contains("Elohim"), "le corps doit rester");
    }

    #[test]
    fn eteindre_le_niveau_3_retire_l_hebreu_et_lui_seul() {
        let html = rendre_sous(Preferences {
            niveau_3: false,
            ..Default::default()
        });
        assert!(
            !html.contains("font-hebreu"),
            "le niveau 3 doit disparaître"
        );
        assert!(html.contains("nom divin"), "la glose doit rester");
    }

    /// Le point qui justifie de retirer les nœuds au lieu de les masquer.
    #[test]
    fn tout_eteindre_ne_laisse_aucun_blanc_orphelin() {
        let html = rendre_sous(Preferences::nu());
        assert!(
            html.contains("Quand ") && html.contains(" commença."),
            "le corps reste une phrase : {html}"
        );
        // Un `display: none` aurait laissé « Elohim   commença. » — deux
        // fragments d'espace de part et d'autre du niveau 3 retiré.
        assert!(!html.contains("  "), "blancs doublés : {html}");
    }

    /// L'intraduisible ne s'éteint jamais, et son lien tient toujours.
    #[test]
    fn l_or_promet_sa_fiche_quels_que_soient_les_reglages() {
        for p in [Preferences::default(), Preferences::nu()] {
            let html = rendre_sous(p);
            assert!(
                html.contains("/fr/lexique/elohim"),
                "le lien de la fiche doit survivre à {p:?}"
            );
        }
    }

    /// À la suite, les versets tiennent dans un seul paragraphe — et gardent
    /// leur ancre, pour qu'un lien partagé vers `#v1` tombe juste.
    #[test]
    fn a_la_suite_les_versets_coulent_mais_gardent_leur_ancre() {
        let html = rendre_sous(Preferences {
            continu: true,
            ..Default::default()
        });
        assert_eq!(
            html.matches("<p ").count(),
            1,
            "un seul paragraphe : {html}"
        );
        assert!(html.contains(r#"id="v1""#), "l'ancre doit rester");
    }
}
