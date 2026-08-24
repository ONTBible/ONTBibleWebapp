use leptos::prelude::*;

use crate::domaine::corpus::{Conteneur, EntreeDeLivre, Ensemble, Section};

/// Le sommaire du corpus — les 70 livres, écrits ou non.
///
/// ## Pourquoi les livres non écrits restent
///
/// Trois livres sur soixante-dix sont traduits. Un sommaire qui ne montrerait
/// que ces trois-là serait plus flatteur et moins vrai : il laisserait croire
/// que le corpus tient en trois livres, et il effacerait ce que le projet est
/// réellement — un chantier dont on voit le plan entier dès la première visite.
///
/// L'ampleur **est** le propos. Elle se lit d'un coup d'œil : quelques titres
/// en or au milieu de soixante-sept en encre atténuée.
///
/// Ils ne sont donc ni cachés, ni grisés jusqu'à l'illisible — l'encre douce
/// tient 6,5:1, au-delà du seuil AA. Ce qui les distingue est qu'ils ne sont
/// pas cliquables : un lien qui ne mène nulle part est la seule chose qu'un
/// sommaire ne doit pas faire.
/// ## Pourquoi les conteneurs apparaissent
///
/// Chaque livre portait déjà son conteneur — `eduyot`, `trei-asar`, les deux
/// `igerot` — et **rien ne l'affichait**. Les vingt-et-une *Igerot* se lisaient
/// comme une liste plate, alors que leur ordre porte un argument.
///
/// L'une de ces coupures n'est pas un rangement. `corpus-order.md` la nomme
/// *pivot herméneutique* : le **Ḥurban**, la destruction du Second Temple en
/// 70. Les lettres d'avant parlent du Temple au présent — *Igeret HaIvrim* est
/// « le dernier mot du *Bayit* vivant » ; trois numéros plus loin, il n'existe
/// plus.
///
/// D'où deux traitements distincts, et c'est délibéré : un simple intertitre
/// pour ce qui **regroupe**, une césure marquée pour ce qui **fracture**. Une
/// césure partout ne marquerait plus rien.
#[component]
pub fn Sommaire(ensembles: Vec<Ensemble>) -> impl IntoView {
    ensembles
        .into_iter()
        .map(|ensemble| {
            view! {
                <section class="mb-20 last:mb-0">
                    <h2 class="mb-10 flex items-center gap-4 text-encre-vive">
                        <span class="massif w-8 shrink-0 text-accent"></span>
                        {ensemble.titre}
                    </h2>

                    {ensemble
                        .sections
                        .into_iter()
                        .map(|section| {
                            view! {
                                <div class="mb-12 last:mb-0">
                                    <h3 class="mb-5 text-sm uppercase tracking-capitales text-accent">
                                        {section.titre.clone()}
                                    </h3>
                                    <ul class="m-0 list-none p-0">
                                        {disposer(section)
                                            .into_iter()
                                            .map(|element| {
                                                let livre = match element {
                                                    Element::Entete(c) => {
                                                        return entete(c).into_any();
                                                    }
                                                    Element::Livre(l) => l,
                                                };
                                                let nom = livre.titre.clone();
                                                let francais = livre.francais.clone();
                                                let hebreu = livre.hebreu.clone();
                                                // Le nom hébreu n'est lu par
                                                // personne à voix haute ici : il
                                                // double le titre latin, et un
                                                // lecteur d'écran le prononcerait
                                                // deux fois.
                                                let cote = view! {
                                                    <span
                                                        aria-hidden="true"
                                                        dir="rtl"
                                                        lang="he"
                                                        class="font-hebreu text-[0.95em] text-encre-douce"
                                                    >
                                                        {hebreu}
                                                    </span>
                                                };
                                                view! {
                                                    <li class="border-b border-filet/40 last:border-0">
                                                        {if livre.ecrit {
                                                            view! {
                                                                <a
                                                                    href=format!("/fr/lire/{}", livre.id)
                                                                    class="flex items-baseline justify-between gap-4 py-3.5 no-underline"
                                                                >
                                                                    <span class="text-accent">
                                                                        {nom}
                                                                        <span class="ms-2.5 text-[0.86em] text-encre-douce">
                                                                            {francais}
                                                                        </span>
                                                                    </span>
                                                                    {cote}
                                                                </a>
                                                            }
                                                                .into_any()
                                                        } else {
                                                            view! {
                                                                <div class="flex items-baseline justify-between gap-4 py-3.5 text-encre-douce">
                                                                    <span>
                                                                        {nom}
                                                                        <span class="ms-2.5 text-[0.86em] opacity-70">
                                                                            {francais}
                                                                        </span>
                                                                    </span>
                                                                    {cote}
                                                                </div>
                                                            }
                                                                .into_any()
                                                        }}
                                                    </li>
                                                }
                                                    .into_any()
                                            })
                                            .collect_view()}
                                    </ul>
                                </div>
                            }
                        })
                        .collect_view()}
                </section>
            }
        })
        .collect_view()
}

/// Ce que le sommaire pose l'un après l'autre : un livre, ou l'en-tête du
/// conteneur qui s'ouvre.
enum Element {
    Entete(Conteneur),
    Livre(EntreeDeLivre),
}

/// Intercale les en-têtes de conteneur dans la suite des livres.
///
/// L'ordre vient des **livres**, jamais de la liste des conteneurs : c'est le
/// corpus qui décide où tombe une coupure. Un conteneur déclaré mais dont
/// aucun livre ne se réclame n'apparaît donc pas, et un identifiant porté par
/// un livre sans déclaration ne fait qu'être ignoré — un sommaire qui refuse
/// de se rendre pour un ornement coûterait plus au lecteur qu'il ne lui donne.
fn disposer(section: Section) -> Vec<Element> {
    let conteneurs = section.conteneurs;
    let mut elements = Vec::new();
    let mut courant: Option<String> = None;
    for livre in section.livres {
        if livre.conteneur != courant {
            courant = livre.conteneur.clone();
            if let Some(id) = &courant {
                if let Some(c) = conteneurs.iter().find(|c| &c.id == id) {
                    elements.push(Element::Entete(c.clone()));
                }
            }
        }
        elements.push(Element::Livre(livre));
    }
    elements
}

/// L'en-tête d'un conteneur, et sa césure quand il en a une.
///
/// **Deux poids, deux traitements.** Un conteneur qui regroupe reçoit un
/// intertitre discret ; celui qui fracture reçoit d'abord un filet appuyé et
/// la ligne qui dit ce que la fracture change pour lire. C'est la seule chose
/// qui distingue *Trei Asar* — douze livres rangés ensemble — du *Ḥurban*, où
/// le monde du texte a cessé d'exister entre deux lignes.
fn entete(c: Conteneur) -> impl IntoView {
    let rupture = c.rupture.map(|texte| {
        view! {
            // Le filet est en accentuation et non en or : l'or dit
            // l'intraduisible partout ailleurs sur le site, et une règle
            // horizontale n'en est pas un.
            <div class="mt-10 mb-8 flex flex-col gap-3 border-t-2 border-accentuation/50 pt-6">
                <p class="m-0 max-w-prose text-[0.92em] italic text-encre-douce">{texte}</p>
            </div>
        }
    });

    view! {
        <li class="list-none border-0">
            {rupture}
            <p class="m-0 mt-6 mb-1 text-xs uppercase tracking-capitales text-encre-douce first:mt-0">
                {c.titre}
            </p>
            <p class="m-0 mb-2 text-[0.82em] text-encre-douce/70">{c.francais}</p>
        </li>
    }
}
