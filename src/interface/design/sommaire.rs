use leptos::prelude::*;

use crate::domaine::corpus::{Conteneur, Ensemble, EntreeDeLivre, Section};
use crate::interface::design::reglages_de_lecture::preferences;

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
                    {sous_titre(ensemble.francais.clone(), ensemble.glose.clone(), "mb-8 -mt-6")}

                    {ensemble
                        .sections
                        .into_iter()
                        .map(|section| {
                            view! {
                                <div class="mb-12 last:mb-0">
                                    <h3 class="mb-1 text-sm uppercase tracking-capitales text-accent">
                                        {section.titre.clone()}
                                    </h3>
                                    {sous_titre(section.francais.clone(), section.glose.clone(), "mb-5")}
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
                                                // Le second nom suit le registre choisi. Un livre
                                                // sans glose garde son pont : *Marqus* est un nom
                                                // d'homme, « Marc » est tout ce qu'il y a à dire.
                                                let second = {
                                                    let fr = livre.francais.clone();
                                                    let gl = livre.glose.clone();
                                                    let prefs = preferences();
                                                    Signal::derive(move || {
                                                        if prefs.get().francais {
                                                            fr.clone()
                                                        } else {
                                                            gl.clone().unwrap_or_else(|| fr.clone())
                                                        }
                                                    })
                                                };
                                                let francais = second;
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
            // **Le filet est en or, et c'est un revirement.**
            //
            // Il a d'abord été posé en accentuation, au motif que l'or dit
            // l'intraduisible partout ailleurs et qu'une règle horizontale n'en
            // est pas un. L'argument était juste sur le mot, faux sur la page :
            // l'accentuation est une couleur *de texte*, et un filet bordeaux
            // au milieu d'un sommaire se lit comme une alerte — quelque chose
            // ne va pas —, alors qu'il annonce une charnière.
            //
            // L'or est la couleur de direction artistique du projet, celle des
            // filets et des cadres. C'est ce que le lecteur y attend.
            <div class="mt-10 mb-8 flex flex-col gap-3 border-t-2 border-or/70 pt-6">
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
            // Le registre vaut ici comme ailleurs.
            //
            // Cette ligne écrivait `{c.francais}` en dur, donc un lecteur qui
            // avait choisi la glose la voyait partout — sections, livres — sauf
            // sur *Trei Asar* et le *Ḥurban*. Un réglage qui s'applique presque
            // partout est pire qu'un réglage absent : on le croit cassé, et l'on
            // ne sait pas où.
            //
            // Relevé par la session Android, qui comparait les deux écrans pour
            // un défaut voisin de son côté. Elle ne s'est pas prononcée — « c'est
            // ton dépôt, je signale seulement » — et elle a bien fait : la glose
            // d'un conteneur est facultative, donc l'écart ne se voyait que sur
            // ceux qui en ont une.
            {sous_titre(c.francais, c.glose, "mb-2")}
        </li>
    }
}

/// Le second nom, dans le registre que le lecteur a choisi.
///
/// **Le français par défaut**, parce qu'un lecteur qui arrive doit pouvoir se
/// repérer avec les mots qu'il connaît. En glose, il lit ce que le nom ONT
/// veut dire — et l'écart entre les deux est ce que le projet montre : *torah*,
/// l'instruction qui vise, est devenue « la Loi », le code qui contraint.
///
/// Rien ne s'affiche quand il n'y a rien à dire : une section dont la glose
/// redirait le pont — *Ketouvim* est « Écrits » des deux côtés — n'en porte
/// pas, et la ligne disparaît plutôt que de se répéter.
fn sous_titre(francais: String, glose: Option<String>, marge: &'static str) -> impl IntoView {
    let prefs = preferences();
    move || {
        let texte = if prefs.get().francais {
            francais.clone()
        } else {
            glose.clone().unwrap_or_else(|| francais.clone())
        };
        (!texte.is_empty()).then(|| {
            view! {
                <p class=format!("m-0 text-[0.82em] text-encre-douce/70 {marge}")>
                    {texte.clone()}
                </p>
            }
        })
    }
}

/// Tout second nom passe par `sous_titre`, et donc par le registre.
///
/// ## Pourquoi un test, et pas une relecture
///
/// Le défaut qu'il garde était invisible : l'en-tête d'un conteneur écrivait
/// `{c.francais}` en dur, si bien que le registre s'appliquait aux sections, aux
/// livres, aux unités — **et pas là**. Un réglage qui vaut presque partout est
/// pire qu'un réglage absent : on le croit cassé, et l'on ne sait pas où.
///
/// Il a fallu qu'une session voisine compare deux écrans pour un défaut voisin
/// de son côté. Personne ne l'aurait vu en relisant ce fichier — la ligne est
/// juste, elle affiche bien un nom français, et rien n'y manque *en apparence*.
///
/// Le test cherche donc la **forme** du défaut plutôt que le cas : un champ
/// `francais` interpolé dans une vue sans passer par la fonction qui décide.
#[cfg(all(test, feature = "ssr"))]
mod tests {
    /// Aucun `francais` n'est peint directement dans une vue de ce module.
    #[test]
    fn le_registre_ne_se_contourne_pas() {
        // Le module de tests est écarté : il **cite** le défaut pour
        // l'expliquer, et un test qui se détecte lui-même n'échoue que sur sa
        // propre prose. C'est le piège de tout relevé qui lit sa propre source.
        let entier = include_str!("sommaire.rs");
        let source = entier
            .split_once("#[cfg(all(test")
            .map(|(avant, _)| avant)
            .unwrap_or(entier);

        // On relève les interpolations `{…francais}` d'une vue, en écartant la
        // seule légitime : l'argument passé à `sous_titre`, qui *est* le point
        // de décision.
        let fautes: Vec<&str> = source
            .lines()
            .map(str::trim)
            .filter(|ligne| !ligne.starts_with("//"))
            .filter(|ligne| ligne.contains(".francais}"))
            .filter(|ligne| !ligne.contains("sous_titre("))
            .collect();

        assert!(
            fautes.is_empty(),
            "un second nom est peint sans passer par `sous_titre`, donc sans le \
             registre — le lecteur qui a choisi la glose verra le français à cet \
             endroit et nulle part ailleurs :\n  {}",
            fautes.join("\n  ")
        );
    }
}
