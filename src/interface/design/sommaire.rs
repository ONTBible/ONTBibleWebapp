use leptos::prelude::*;

use crate::domaine::corpus::{Conteneur, Ensemble, EntreeDeLivre, Section};
use crate::interface::design::reglages_de_lecture::preferences;
use crate::interface::design::{EnteteDeSection, Groupe, Ligne};

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
/// ## Le gabarit est celui de l'app depuis le 29 septembre 2026
///
/// Le sommaire composait une **édition** : un grand titre d'ensemble avec le
/// signe de la montagne, un intertitre de section, puis des lignes à plat avec
/// le nom hébreu à droite. L'app compose une **liste** : un en-tête de section
/// sur deux lignes, puis une carte arrondie où chaque ligne porte son chevron.
///
/// L'auteur a comparé les deux écrans et tranché : « je veux aussi une copie de
/// l'UI ». Ce qui suit emploie donc `liste_groupee`, et ce qui a disparu est
/// une décision, pas un oubli :
///
/// - **le paragraphe d'introduction** — une liste s'ouvre, elle ne s'introduit
///   pas. Il vivait sur la page, pas dans ce composant ;
/// - **le signe de la montagne** devant chaque ensemble — il faisait de
///   l'en-tête un titre d'affiche, là où l'app en fait un repère ;
/// - **le nom hébreu à droite de chaque ligne** — il tenait la place de la
///   valeur, qui chez l'app porte l'avancement. Il passe sous le titre, avec le
///   second nom.
/// ## Il liste des **parties**, pas des livres — depuis le 29 septembre 2026
///
/// L'app navigue en quatre temps : la Bible, une section, un livre, une unité.
/// Le site en avait trois — soixante-dix livres posés d'un coup, groupés par
/// intertitres.
///
/// La différence n'est pas de mise en page, c'est de **nature**. L'app présente
/// les sections comme des destinations, avec leur avancement en regard :
/// « Torah 1/6 », « Nevi'im 0/20 ». On choisit un rayon, puis un livre. Le site
/// demandait de balayer soixante-dix entrées pour trouver les six qui ont du
/// texte — et il le demandait sur la **première page** de la liseuse.
///
/// Cet écart ne se voyait dans aucune comparaison de jetons. Il a fallu mettre
/// les deux écrans côte à côte, ce que l'auteur avait demandé : *« passe ton
/// temps à faire des comparaisons des screens »*.
#[component]
pub fn Sommaire(ensembles: Vec<Ensemble>) -> impl IntoView {
    ensembles
        .into_iter()
        .map(|ensemble| {
            let titre = ensemble.titre.clone();
            let francais = ensemble.francais.clone();
            let glose = ensemble.glose.clone();
            view! {
                <section class="mb-8 last:mb-0">
                    <EnteteDeSection glose=Box::new({
                        let f = francais.clone();
                        let g = glose.clone();
                        move || sous_titre(f, g, "").into_any()
                    })>{titre}</EnteteDeSection>
                    <Groupe>
                        {ensemble
                            .sections
                            .into_iter()
                            .map(ligne_de_partie)
                            .collect_view()}
                    </Groupe>
                </section>
            }
        })
        .collect_view()
}

/// Une partie du corpus, en une ligne.
///
/// **L'avancement est la valeur de droite**, comme chez l'app — « 1/6 ». C'est
/// ce que le chapeau du sommaire disait en prose (« les titres se lisent, les
/// autres attendent ») et que la forme dit mieux : un lecteur voit d'un coup
/// où il y a du texte, sans lire une ligne.
fn ligne_de_partie(section: Section) -> AnyView {
    let id = section.id.clone();
    let titre = section.titre.clone();
    let ecrits = section.livres.iter().filter(|l| l.ecrit).count();
    let total = section.livres.len();
    let second = sous_titre(section.francais.clone(), section.glose.clone(), "");

    view! {
        <Ligne
            // Une partie sans aucun livre écrit reste **atteignable** : son
            // plan est le propos. C'est la règle du sommaire d'origine, et elle
            // ne change pas — « l'ampleur *est* le propos ».
            chemin=Some(crate::domaine::chemins::partie(crate::interface::arbre::arbre_maintenant(), &id))
            titre=Box::new(move || titre.into_any())
            sous_titre=Box::new(move || second.into_any())
            valeur=Box::new(move || {
                view! { <span class="chiffres-tableau">{ecrits} "/" {total}</span> }.into_any()
            })
        />
    }
    .into_any()
}

/// Les livres d'une partie — l'étage que `Partie` rend.
///
/// Il reprend les en-têtes de conteneur du sommaire d'origine : c'est ici
/// qu'ils comptent, puisque c'est ici que les livres se lisent. Le *Ḥurban* et
/// sa césure vivent dans une partie, pas sur la première page.
#[component]
pub fn SommaireDUnePartie(section: Section) -> impl IntoView {
    view! {
        <Groupe>
            {disposer(section).into_iter().map(ligne_de_sommaire).collect_view()}
        </Groupe>
    }
}

/// Une entrée du sommaire — un livre, ou l'en-tête du conteneur qui s'ouvre.
fn ligne_de_sommaire(element: Element) -> AnyView {
    let livre = match element {
        Element::Entete(c) => return entete(c).into_any(),
        Element::Livre(l) => l,
    };

    let nom = livre.titre.clone();
    let hebreu = livre.hebreu.clone();
    // Le second nom suit le registre choisi. Un livre sans glose garde son
    // pont : *Marqus* est un nom d'homme, « Marc » est tout ce qu'il y a à dire.
    let second = sous_titre(livre.francais.clone(), livre.glose.clone(), "");

    view! {
        <Ligne
            chemin=livre.ecrit.then(|| crate::domaine::chemins::livre(crate::interface::arbre::arbre_maintenant(), &livre.id))
            titre=Box::new(move || {
                view! {
                    {nom}
                    // Le nom hébreu double le titre latin : un lecteur d'écran
                    // le prononcerait deux fois.
                    <span
                        aria-hidden="true"
                        dir="rtl"
                        lang="he"
                        class="ms-2.5 font-hebreu text-[0.9em] font-normal text-encre-douce"
                    >
                        {hebreu}
                    </span>
                }
                    .into_any()
            })
            sous_titre=Box::new(move || second.into_any())
        />
    }
    .into_any()
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
