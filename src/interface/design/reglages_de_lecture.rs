use leptos::ev;
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

use crate::domaine::lecture::{Fonte, Preferences, Theme};

/// Où le navigateur retient les réglages.
///
/// Un espace de noms préfixé : le site en aura d'autres, et une clé nue comme
/// `lecture` finirait par entrer en conflit avec quelque chose.
#[cfg(feature = "hydrate")]
const CLE: &str = "ont.lecture";

/// Installe les réglages de lecture pour la page.
///
/// À appeler **une fois**, dans la page qui porte du corpus. Elle rend le
/// signal, que la page passe au panneau ; tous les composants de texte le
/// retrouvent par le contexte, sans qu'on ait à le leur passer de main en main
/// à travers cinq niveaux de composition.
pub fn fournir_preferences() -> RwSignal<Preferences> {
    // **Idempotente**, et c'est une condition du thème.
    //
    // Les niveaux du texte n'intéressent que les pages qui en portent ; la
    // peau, elle, vaut pour tout le site — l'auteur a demandé une webapp, pas
    // une section. `App` l'installe donc pour tout le monde, et les quatre
    // pages de corpus continuent de l'appeler pour elles-mêmes.
    //
    // Sans ce court-circuit, le second appel poserait un **second signal** :
    // le panneau piloterait celui de la page, l'attribut de `<html>` suivrait
    // celui de `App`, et changer de thème ne ferait rien. Une panne qui
    // ressemble à un réglage sans effet — c'est-à-dire au défaut que le repli
    // muet de `preferences()` a déjà coûté une fois, vingt lignes plus bas.
    if let Some(deja) = use_context::<RwSignal<Preferences>>() {
        return deja;
    }

    let preferences = RwSignal::new(Preferences::default());
    provide_context(preferences);

    // Le serveur rend toujours avec les défauts — tout est montré. C'est le
    // seul rendu honnête pour qui n'a pas de JavaScript, et c'est aussi ce
    // qu'un moteur de recherche doit indexer : le texte entier, avec son
    // appareil critique.
    //
    // Le navigateur lit ensuite ce qu'il a retenu et recompose. L'ordre est ce
    // qui évite un désaccord d'hydratation : les deux côtés partent du même
    // état, et seul le second bouge.
    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            if let Some(retenu) = lire() {
                preferences.set(retenu);
            }
        });

        Effect::new(move |_| ecrire(preferences.get()));

        // La peau ne se pose **pas** ici. Elle dépend du chemin autant que du
        // signal — l'auteur l'a bornée à la liseuse le 21 septembre — et le
        // chemin n'est lisible que sous `<Router>`, où cette fonction n'est pas
        // encore. C'est `app::PeauDuLecteur` qui s'en charge.
    }

    preferences
}

/// Les réglages de la page, ou les défauts.
///
/// Les défauts et non une erreur quand le contexte manque : ce composant de
/// texte sert aussi hors de la liseuse — la comparaison de l'accueil, la carte
/// du verset du jour — où il n'y a pas de panneau et où tout doit se voir.
pub fn preferences() -> Signal<Preferences> {
    match use_context::<RwSignal<Preferences>>() {
        Some(signal) => signal.into(),
        None => {
            // **Le repli est muet, et c'est ce qui coûte.**
            //
            // Sans contexte, tout ce qui lit les réglages reçoit un signal
            // *constant* : la page s'affiche, le texte est juste, et aucun
            // réglage n'a jamais d'effet. Rien ne casse — c'est une panne qui
            // ressemble exactement à un fonctionnement.
            //
            // Elle a vécu sur deux pages sur trois : la fiche du lexique et le
            // sommaire d'un livre consommaient les réglages sans que personne
            // ne les fournisse.
            //
            // Le repli reste — une page sans réglages doit s'afficher — mais il
            // le dit maintenant. `debug_assert` en développement, où l'on peut
            // corriger ; un avertissement dans la console du navigateur, où
            // l'on peut le voir sans relire tout l'arbre.
            debug_assert!(
                false,
                "les réglages de lecture sont lus sans avoir été fournis : \
                 appeler `fournir_preferences()` dans la page qui compose ce \
                 texte, sinon aucune bascule n'aura d'effet"
            );
            #[cfg(feature = "hydrate")]
            web_sys::console::warn_1(
                &"réglages de lecture lus sans contexte — les bascules n'auront aucun effet".into(),
            );
            Signal::stored(Preferences::default())
        }
    }
}

#[cfg(feature = "hydrate")]
fn stockage() -> Option<web_sys::Storage> {
    // `local_storage()` échoue quand le stockage est refusé — navigation
    // privée stricte, cookies bloqués par le site. On s'en passe alors : les
    // réglages valent pour la page, et rien n'est retenu. C'est très
    // préférable à une page qui refuse de s'afficher.
    web_sys::window()?.local_storage().ok().flatten()
}

#[cfg(feature = "hydrate")]
fn lire() -> Option<Preferences> {
    let brut = stockage()?.get_item(CLE).ok().flatten()?;
    serde_json::from_str(&brut).ok()
}

#[cfg(feature = "hydrate")]
fn ecrire(preferences: Preferences) {
    if let (Some(stockage), Ok(json)) = (stockage(), serde_json::to_string(&preferences)) {
        let _ = stockage.set_item(CLE, &json);
    }
}

/// La peau du lecteur, **montée par les pages qui la portent**.
///
/// ## Pourquoi elle ne se décide pas depuis l'URL
///
/// La première version consultait `use_location().pathname` et comparait à une
/// table de préfixes. **Elle a été prise en défaut avant d'être livrée.**
///
/// Le banc : le site chargé dans un cadre de même origine, et un lien cliqué à
/// sa place — le seul moyen d'agir sur la page sans main sur l'écran. Réglages
/// du lecteur : `theme: clair`. Relevé du 21 septembre 2026 :
///
/// ```text
///  6 s  url=/fr  peau=—      titre=Le cosmos hébreu n'est
/// 14 s  url=/fr  peau=—      titre=Le cosmos hébreu n'est   ← clic sur « Lire »
/// 16 s  url=/fr  peau=clair  titre=Le cosmos hébreu n'est
/// 20 s  url=/fr  peau=clair  titre=Le cosmos hébreu n'est
/// ```
///
/// **L'accueil porte la peau de la liseuse**, et il la garde. C'est exactement
/// le rendu que l'auteur a écarté en bornant le thème : le massif d'aubergine
/// devient une forme violette sur de la crème, et « C'est un Temple », qui est
/// en or, disparaît dans son fond.
///
/// ### Ce que le banc ne prouve pas, et qu'il ne faut pas lui faire dire
///
/// La cause exacte n'est **pas** établie. L'explication naturelle — le chemin
/// du routeur change au départ d'une navigation, la vue attend ses données —
/// est plausible et je ne peux pas la démontrer ici : sur la version corrigée,
/// **le banc n'obtient plus aucune navigation du tout**, ni vers la liseuse ni
/// vers une page d'édition, sur vingt secondes de relevé. Un clic synthétique
/// dans un cadre ne pilote pas ce routeur ; il permet seulement d'observer.
///
/// Ce qui est établi tient en deux lignes, et suffit :
///
/// - avec la peinture par l'URL, l'accueil a été **vu** portant `clair` ;
/// - avec la peinture par le montage, il ne l'est plus — et il ne peut plus
///   l'être, puisque l'attribut ne peut pas exister sans que la page de
///   lecture soit à l'écran.
///
/// La seconde propriété ne dépend d'aucune hypothèse sur le routeur, et c'est
/// pour ça qu'on la préfère : elle est vraie par construction, pas par mesure.
///
/// Reste non éprouvé ici : qu'une navigation **aboutie** vers la liseuse pose
/// bien la peau. C'est « un composant se monte, son effet s'exécute » — le même
/// mécanisme que les trois bascules de cette feuille, et il est éprouvé par le
/// chargement direct d'un chapitre.
///
/// ## Ce qui remplace la table
///
/// **Rien, côté navigateur.** Ce composant est monté par `PageDeLecture` et par
/// la recherche ; il pose l'attribut en arrivant et le retire en partant. Le
/// montage suit la vue par construction, donc la peau aussi.
///
/// La règle « quelles pages sont la liseuse » cesse d'être une liste de chemins
/// qu'il faut tenir d'accord avec les routes : c'est la structure qui la dit.
/// `LA_LISEUSE` ne sert plus qu'au script de l'en-tête, qui s'exécute avant
/// qu'aucun composant n'existe — et là, l'URL et la vue coïncident par
/// définition, puisque rien n'est encore rendu.
#[component]
pub fn PeauDeLaLiseuse() -> impl IntoView {
    // **Le compteur vit des deux côtés**, et il le faut.
    //
    // Il était dans le bloc `hydrate`, donc à zéro sur le serveur — qui
    // rendait alors le pied de page du site, que le navigateur retirait
    // aussitôt. Les deux arbres auraient divergé à l'hydratation : le piège
    // exact du banc de la carte, deux heures plus tôt.
    //
    // **La peau suit le chemin, et non un compteur de montage.**
    //
    // Il y a eu ici un compteur de pages de liseuse à l'écran. Il répondait à
    // une vraie question — *reste-t-il une page de liseuse ?* — parce qu'en
    // passant d'une page à l'autre, Leptos **monte la nouvelle avant de
    // nettoyer l'ancienne** : un simple pose/retire laissait le dernier mot à
    // la page qu'on venait de quitter, et le thème s'effaçait à chaque
    // navigation.
    //
    // Le compteur réglait ça et en apportait un autre, que le banc a fini par
    // rendre le 1er octobre 2026 :
    //
    // ```text
    // At reglages_de_lecture.rs:243, you tried to access a reactive value
    // which was defined at reglages_de_lecture.rs:268, but it has already
    // been disposed.
    // ```
    //
    // Le signal était créé par **la première page qui l'appelait**, donc dans
    // une portée qui meurt à la navigation suivante ; le nettoyage de la page
    // d'après lisait un signal mort, et le WASM s'arrêtait à la septième
    // navigation.
    //
    // ==Un compteur de montages répond à la question « qu'est-ce qui est
    // là ? ». Le chemin répond à « où sommes-nous ? » — et c'est la seconde
    // qu'on posait.== Une condition tirée du chemin n'a ni ordre ni durée de
    // vie : le serveur et le client y répondent la même chose au même instant.
    #[cfg(feature = "hydrate")]
    {
        let preferences = preferences();
        let sous_un_arbre = crate::interface::arbre::dans_un_arbre();

        // **Changer d'habillage change d'adresse.** Le réglage vit dans
        // `ont.lecture` ; le cookie et le serveur s'en chargent aux chargements
        // suivants, mais celui qui vient de cliquer doit voir l'effet tout de
        // suite — et sur **la même page**, pas à l'accueil de l'autre arbre.
        //
        // > Un réglage qui fait perdre sa place n'est pas un réglage, c'est une
        // > sortie.
        //
        // On recharge plutôt qu'on ne navigue : le chrome est rendu par le
        // serveur, donc une navigation du routeur laisserait l'ancien en place.
        Effect::new(move |_| {
            let vise = preferences.get().habillage.resoudre(grand_ecran());
            let ici = leptos_router::hooks::use_location().pathname.get();
            if crate::domaine::lecture::Arbre::du_chemin(&ici).is_some_and(|a| a != vise) {
                poser_le_cookie(vise);
                let cible = crate::domaine::chemins::dans(vise, &ici);
                if let Some(fenetre) = web_sys::window() {
                    let _ = fenetre.location().replace(&cible);
                }
            }
        });

        // Un seul effet, et il suit les deux : les réglages **et** le lieu.
        // Sortir de la liseuse retire la peau sans qu'aucun nettoyage n'ait à
        // s'exécuter dans le bon ordre.
        Effect::new(move |_| {
            if sous_un_arbre.get() {
                let reglages = preferences.get();
                poser_la_peau(Some(reglages.theme), Some(reglages.fonte));
                poser_la_taille(reglages.corps);
                poser_l_interligne(reglages.interligne);
                poser_la_coupure(reglages.coupure);
            } else {
                poser_la_peau(None, None);
            }
        });
    }
    view! { <></> }
}

/// Écrit — ou retire — l'attribut de peau, et accorde la barre du navigateur.
///
/// `None` retire l'attribut plutôt que d'y poser `mystique` : le défaut de
/// `jetons.css` est déjà mystique, donc une racine nue rend la nuit
/// d'aubergine. Un attribut posé dirait « ce lecteur a choisi cette peau ici »,
/// ce qui est faux.
///
/// ## `theme-color` se lit, il ne se transcrit pas
///
/// La barre du navigateur sur téléphone prend la couleur de cette balise. La
/// valeur juste est le fond en vigueur — qui vit dans `jetons.css`, donc chez
/// l'app. La recopier ici en ferait la seule couleur du site hors de la chaîne
/// de portage, et elle se périmerait à la première retouche.
///
/// On la **demande au navigateur** : l'attribut vient d'être posé, la feuille
/// est chargée, `getComputedStyle` rend le `--ont-background` en vigueur.
/// Pose le facteur de la seconde échelle sur la racine.
///
/// `--lecture` vaut `corps / 19` — 1 au défaut, donc `calc(x * 1)` rend `x` et
/// rien ne bouge d'un pixel tant que le lecteur n'a pas touché au réglage.
///
/// **Elle ne se retire pas au démontage**, contrairement à la peau. La
/// variable n'est lue que par `.liseuse`, qui n'existe que dans la liseuse :
/// laissée sur la racine, elle est inerte partout ailleurs. La retirer
/// coûterait une seconde condition sans rien protéger.
#[cfg(feature = "hydrate")]
/// L'interligne, en **total CSS** et non en supplément.
///
/// L'app compte un supplément — `.lineSpacing` s'ajoute à l'interligne naturel
/// de la fonte —, la CSS compte la hauteur entière. Les deux ne se
/// convertissent pas exactement, le supplément naturel de Literata n'étant pas
/// un nombre que ce dépôt connaisse.
///
/// Ce qui se transpose est **le défaut et l'amplitude** : au cran 5, la valeur
/// rendue est `1,68` — celle que le §5 a mesurée —, et chaque cran vaut un
/// dixième. De 1,38 à 2,18, ce qui couvre l'amplitude de l'app.
///
/// Le défaut rend donc **exactement** ce que le site rendait avant ce réglage,
/// et c'était la condition : un curseur dont le cran du milieu déplacerait la
/// valeur documentée changerait la composition de tout le monde pour offrir un
/// réglage à quelques-uns.
#[cfg(feature = "hydrate")]
fn poser_l_interligne(crans: u8) {
    let Some(racine) = racine() else { return };
    let total = 1.68 + (f64::from(crans) - f64::from(Theme::INTERLIGNE_PAR_DEFAUT)) / 10.0;
    let _ = racine
        .style()
        .set_property("--interligne", &format!("{total}"));
}

/// La césure, allumée ou éteinte par le lecteur.
///
/// **Elle était globale**, posée sur `p` dans la feuille — donc allumée pour
/// tout le monde, sans moyen de l'éteindre. C'est un réglage chez l'app, et sa
/// raison vaut doublement ici : *qui grossit le texte pour le voir se retrouve
/// avec plus de coupures, pas moins.*
#[cfg(feature = "hydrate")]
fn poser_la_coupure(coupe: bool) {
    let Some(racine) = racine() else { return };
    let _ = racine
        .style()
        .set_property("--coupure", if coupe { "auto" } else { "manual" });
}

/// L'élément racine, quand le navigateur est là.
#[cfg(feature = "hydrate")]
fn racine() -> Option<web_sys::HtmlElement> {
    web_sys::window()
        .and_then(|f| f.document())
        .and_then(|d| d.document_element())
        .and_then(|r| r.dyn_ref::<web_sys::HtmlElement>().cloned())
}

#[cfg(feature = "hydrate")]
fn poser_la_taille(corps: u8) {
    // **L'attribut `cfg` avait disparu**, et il y était : mon insertion s'est
    // glissée entre lui et sa fonction, si bien qu'il gardait `racine()` et
    // laissait celle-ci compiler côté serveur — où `web_sys` n'existe pas.
    //
    // Un attribut n'appartient pas au fichier, il appartient à l'élément qui
    // le suit immédiatement. Insérer « avant une fonction » veut donc dire
    // « avant ses attributs », et rien ne le rappelle à la lecture : les deux
    // lignes se suivent sans qu'on voie laquelle porte l'autre.
    let Some(racine) = racine() else { return };
    let facteur = f64::from(corps) / f64::from(Theme::CORPS_PAR_DEFAUT);
    let _ = racine
        .style()
        .set_property("--lecture", &format!("{facteur}"));
}

/// Sommes-nous sur un grand écran ? La même borne que le script de l'en-tête.
///
/// **64 rem et non une largeur en pixels** : la borne suit la taille de police
/// du lecteur, comme tout le reste de la feuille. Quelqu'un qui grossit son
/// texte a un écran proportionnellement plus petit, et c'est exactement le
/// lecteur pour qui la barre latérale devient un couloir.
#[cfg(feature = "hydrate")]
fn grand_ecran() -> bool {
    web_sys::window()
        .and_then(|f| f.match_media("(min-width: 64rem)").ok().flatten())
        .is_some_and(|m| m.matches())
}

/// Pose le cookie que le serveur lira au prochain chargement.
///
/// Un an, `samesite=lax` : il ne voyage pas sur une requête d'un autre site, et
/// il n'a rien de sensible — c'est un choix d'affichage, pas une identité.
#[cfg(feature = "hydrate")]
fn poser_le_cookie(arbre: crate::domaine::lecture::Arbre) {
    if let Some(document) = web_sys::window()
        .and_then(|f| f.document())
        .and_then(|d| d.dyn_into::<web_sys::HtmlDocument>().ok())
    {
        let _ = document.set_cookie(&format!(
            "ont.habillage={};path=/;max-age=31536000;samesite=lax",
            arbre.segment()
        ));
    }
}

#[cfg(feature = "hydrate")]
fn poser_la_peau(theme: Option<Theme>, fonte: Option<Fonte>) {
    let Some(document) = web_sys::window().and_then(|f| f.document()) else {
        return;
    };
    let Some(racine) = document.document_element() else {
        return;
    };

    match theme {
        Some(theme) => {
            let _ = racine.set_attribute("data-theme", theme.attribut());
        }
        None => {
            let _ = racine.remove_attribute("data-theme");
        }
    }
    // La fonte suit la même règle que la peau — posée dans la liseuse, retirée
    // en sortant. Son sélecteur est déjà borné par `.liseuse`, mais laisser
    // l'attribut traîner dirait « ce lecteur a choisi ici », ce qui est faux.
    match fonte {
        Some(fonte) => {
            let _ = racine.set_attribute("data-fonte", fonte.attribut());
        }
        None => {
            let _ = racine.remove_attribute("data-fonte");
        }
    }

    let fond = web_sys::window()
        .and_then(|f| f.get_computed_style(&racine).ok().flatten())
        .and_then(|style| style.get_property_value("--ont-background").ok())
        .unwrap_or_default();
    let fond = fond.trim();

    // Vide quand la feuille n'est pas encore là. On ne pose rien plutôt que de
    // poser une chaîne vide, qui ferait retomber la barre sur le blanc du
    // navigateur — plus visible que la couleur périmée qu'on remplaçait.
    if fond.is_empty() {
        return;
    }
    if let Ok(Some(balise)) = document.query_selector("meta[name='theme-color']") {
        let _ = balise.set_attribute("content", fond);
    }
}

/// Les réglages de lecture — un bouton qui suit, et une feuille qui monte.
///
/// ## Le bouton est dans la barre du haut, et il y a été **déplacé**
///
/// Ce paragraphe disait « il flotte en bas », et l'argument tenait : un
/// chapitre fait jusqu'à quarante-six versets, on éteint les gloses au milieu
/// de la lecture, et un réglage qu'il faut remonter chercher n'en est plus un.
///
/// Il y a été repris pour deux raisons, et la seconde annule la première.
///
/// D'abord l'app : `ChapterView` pose « aA » en `ONTPlacement.principale`,
/// c'est-à-dire **en haut à droite**. Le site le mettait en bas en croyant
/// copier l'app ; il copiait un geste, pas une place.
///
/// Ensuite la place elle-même : le bas n'est plus libre. La barre d'onglets
/// l'occupe depuis que la liseuse porte la chrome de l'app — il a fallu
/// dégager le bouton par-dessus elle, puis l'effacer pendant une sélection
/// parce qu'il chevauchait la barre de partage. Deux rustines pour tenir une
/// place que trois objets se disputaient.
///
/// Ce que l'argument d'origine demandait vraiment n'est pas « en bas », c'est
/// **toujours atteignable**. La barre du haut est `sticky` : elle suit le
/// lecteur au quarante-sixième verset comme au premier. La feuille, elle,
/// monte toujours par-dessus le texte.
///
/// ## Il porte les réglages de l'app, et rien d'autre
///
/// Deux bascules de niveaux et une de disposition, avec les libellés de
/// `ReadingSettingsSheet`. Un lecteur qui passe du téléphone au site doit
/// retrouver les mêmes mots pour les mêmes choses.
///
/// Ce que le site n'emprunte **pas** : la taille du corps, l'interligne, la
/// fonte et le thème. L'app a raison de les offrir — elle est un lecteur, et un
/// lecteur s'adapte à qui le tient. Le site est une **édition** : sa nuit
/// d'aubergine, son corps à 21 px et sa Literata sont des décisions, pas des
/// défauts qu'on propose de corriger.
///
/// ## Il n'apparaît qu'une fois qu'il peut servir
///
/// Rendu par le navigateur seulement. Sans JavaScript, des interrupteurs qui ne
/// commutent rien seraient un mensonge — pire qu'une absence, parce qu'on les
/// essaie. La page reste alors ce qu'elle est : le texte entier, tous niveaux
/// montrés.
#[component]
pub fn ReglagesDeLecture(preferences: RwSignal<Preferences>) -> impl IntoView {
    // Vraie dès qu'un verset est désigné à l'écran. Absente hors de la
    // liseuse — le bouton reste alors visible en toutes circonstances, ce qui
    // est le comportement juste là où l'on ne sélectionne rien.
    let choix = crate::interface::design::selection();
    let selection_active = move || choix.is_some_and(|s| s.with(|s| !s.is_empty()));

    // Faux au rendu du serveur, vrai dès que le navigateur a repris la main.
    // Les deux côtés partent donc du même balisage, et le bouton se pose après
    // — sans désaccord d'hydratation.
    let utilisable = RwSignal::new(false);
    Effect::new(move |_| utilisable.set(true));

    let ouvert = RwSignal::new(false);

    // **Lu une fois, et sans s'abonner.** L'habillage d'une page ne change pas
    // sans qu'elle soit remontée : le sélecteur de « Vous » recharge, et le
    // routeur remonte à chaque navigation.
    let sous_l_edition = crate::interface::arbre::sous_l_edition();

    // Échap referme. C'est le geste attendu de tout ce qui se pose par-dessus
    // une page, et l'omettre enferme qui navigue au clavier.
    let _ = window_event_listener(ev::keydown, move |evenement| {
        if evenement.key() == "Escape" {
            ouvert.set(false);
        }
    });

    view! {
        <Show when=move || utilisable.get()>
            // Le voile. Il ferme au clic, et il est `aria-hidden` : ce n'est pas
            // un objet, c'est la page qui recule.
            //
            // Il reste **monté** en permanence, et c'est ce qui permet
            // d'animer la fermeture autant que l'ouverture : un `<Show>`
            // arrache l'élément du document, et rien ne peut plus transiter
            // sur ce qui n'existe plus. Une feuille qui monte doucement et
            // disparaît d'un coup se remarque davantage qu'une feuille qui
            // n'était pas animée du tout.
            <div
                aria-hidden="true"
                on:click=move |_| ouvert.set(false)
                class="fixed inset-0 z-40 bg-nuit/70 backdrop-blur-sm transition-opacity duration-300 ease-out motion-reduce:transition-none"
                class=("opacity-0", move || !ouvert.get())
                class=("pointer-events-none", move || !ouvert.get())
                class=("opacity-100", move || ouvert.get())
            ></div>

            // `end-6` et non `right-6` : la propriété logique suivra le jour
            // d'une édition en écriture droite-à-gauche.
            //
            // Le retrait du bas ajoute la zone sûre de l'appareil — sans elle,
            // le bouton se pose sur la barre d'accueil d'un iPhone, où le geste
            // de retour à l'écran d'accueil le prend en premier.
            <button
                type="button"
                on:click=move |_| ouvert.update(|o| *o = !*o)
                aria-expanded=move || ouvert.get().to_string()
                aria-label="Réglages de lecture"
                // `active:scale-95` : le bouton s'enfonce sous le doigt. C'est
                // le seul retour tactile qu'un navigateur laisse donner, et son
                // absence fait douter que le clic ait été pris.
                // **Deux places, parce que deux chromes.**
                //
                // Sous l'app, c'est une capsule de la barre du haut, à droite
                // d'une pastille de renvoi : le dessin d'iOS 26, et
                // `ONTPlacement.principale` le met là.
                //
                // Sous l'édition, il **flotte en bas à droite**, comme `main` —
                // et l'argument d'origine du §8 bis tient toujours : *un
                // chapitre fait jusqu'à quarante-six versets, et l'on décide
                // d'éteindre les gloses au milieu de la lecture ; un réglage
                // qu'il faut remonter chercher n'en est plus un.* Ce qui avait
                // fait remonter le bouton le 29 septembre est la barre
                // d'onglets, qui occupait le bas — et l'édition n'en a pas.
                //
                // La zone sûre s'ajoute au retrait : sans elle, le bouton se
                // pose sur la barre d'accueil d'un iPhone, où le geste de
                // retour prend le clic en premier.
                class=move || {
                    if sous_l_edition {
                        "halo se-poser fixed end-6 z-50 flex size-14 items-center \
                         justify-center rounded-full border border-or/30 bg-surface-haute \
                         text-accent transition-[transform,border-color,box-shadow,opacity] \
                         duration-200 ease-out hover:border-or/60 active:scale-95 \
                         focus-visible:outline focus-visible:outline-2 \
                         focus-visible:outline-offset-2 focus-visible:outline-accent \
                         motion-reduce:transition-none"
                    } else {
                        "presse verre se-poser flex size-9 items-center justify-center \
                         rounded-full text-accent focus-visible:outline focus-visible:outline-2 \
                         focus-visible:outline-offset-2 focus-visible:outline-accent"
                    }
                }
                style=move || {
                    sous_l_edition.then_some("bottom: calc(1.5rem + env(safe-area-inset-bottom))")
                }
                // Il s'efface pendant une sélection, et il n'en reste
                // qu'une raison sur deux.
                //
                // La mécanique est tombée avec le déplacement : le bouton
                // n'est plus dans le coin qu'occupe la barre de sélection, ils
                // ne peuvent plus se chevaucher.
                //
                // Celle de propos tient seule, et elle suffit : on ne règle pas
                // sa typographie pendant qu'on choisit des versets à partager.
                // Laisser les deux à l'écran ferait deux actions principales,
                // donc aucune. L'app dit la même chose autrement — toute sa
                // barre d'outils est sous `if actif`.
                //
                // `inert` en plus de l'opacité : un bouton transparent reste
                // cliquable et tabulable — c'est la même règle que pour la
                // feuille, et l'oublier mettrait un piège invisible sous la
                // barre.
                class=("opacity-0", selection_active)
                class=("pointer-events-none", selection_active)
                inert=move || selection_active().then_some("")
            >
                // **Le libellé suit le bouton, pas l'inverse.** `main` écrit
                // `text-xl` dans un cercle de `size-14` ; la capsule de l'app
                // fait `size-9` et prend `text-base`. Reporter l'un dans
                // l'autre donne un « aA » perdu au milieu d'un grand disque —
                // ce que l'auteur a vu tout de suite : *« le texte à
                // l'intérieur n'a pas la bonne taille »*.
                //
                // ==Une taille de contenu reprise sans sa boîte n'est pas une
                // valeur portée, c'est une valeur déplacée.==
                <span
                    aria-hidden="true"
                    class="font-titre leading-none"
                    class=("text-xl", sous_l_edition)
                    class=("text-base", !sous_l_edition)
                >
                    "aA"
                </span>
            </button>

            <div
                role="dialog"
                aria-modal="true"
                aria-label="Réglages de lecture"
                // `inert` quand elle est fermée — et rendu **absent** plutôt que
                // « faux » : c'est un attribut booléen, donc `inert="false"`
                // rendrait la feuille inerte tout autant. Sans lui, une feuille
                // restée montée garderait ses interrupteurs dans l'ordre de
                // tabulation et dans l'arbre d'accessibilité, invisibles mais
                // atteignables.
                inert=move || (!ouvert.get()).then_some("")
                // Collée en bas sur un téléphone — c'est là qu'arrive le
                // pouce, et c'est de là qu'elle monte. Sur un grand écran elle
                // se pose au-dessus du bouton, à sa largeur, et croît depuis
                // son coin : le mouvement dit d'où elle sort.
                // **Elle sort du bouton, donc elle le suit.** Sur un grand
                // écran, la feuille croît depuis son coin d'origine : ancrée en
                // haut quand le bouton est en haut, au-dessus de lui quand il
                // flotte en bas. Une feuille qui pousse du coin opposé à celui
                // qu'on vient de toucher ne se lit plus comme venant de là.
                class=move || {
                    let commun = "fixed inset-x-0 bottom-0 z-50 max-h-[85dvh] overflow-y-auto \
                                  rounded-t-feuille border-t border-filet bg-surface-haute px-6 \
                                  pt-6 transition-[transform,opacity] duration-300 ease-out \
                                  sm:inset-x-auto sm:end-6 sm:w-96 sm:rounded-feuille sm:border \
                                  motion-reduce:transition-none";
                    if sous_l_edition {
                        format!("{commun} sm:bottom-24 sm:origin-bottom-right")
                    } else {
                        format!("{commun} sm:bottom-auto sm:top-16 sm:origin-top-right")
                    }
                }
                class=("translate-y-full", move || !ouvert.get())
                class=("opacity-0", move || !ouvert.get())
                class=("pointer-events-none", move || !ouvert.get())
                class=("sm:-translate-y-2", move || !ouvert.get())
                class=("sm:scale-95", move || !ouvert.get())
                class=("translate-y-0", move || ouvert.get())
                class=("opacity-100", move || ouvert.get())
                class=("sm:scale-100", move || ouvert.get())
                style="padding-bottom: calc(1.5rem + var(--barre-d-onglets, 0px) + env(safe-area-inset-bottom))"
            >
                    // La poignée : c'est elle qui fait lire l'objet comme une
                    // feuille qu'on tire, et non comme une boîte qui a surgi.
                    // Décorative, donc masquée à l'oreille — et seulement sur
                    // téléphone, où la feuille vient du bas.
                    <span
                        aria-hidden="true"
                        class="mx-auto mb-5 block h-1 w-10 rounded-full bg-filet sm:hidden"
                    ></span>

                    <div class="mb-6 flex items-center justify-between gap-4">
                        <p class="m-0 text-sm uppercase tracking-capitales text-accent">"Lecture"</p>
                        <button
                            type="button"
                            on:click=move |_| ouvert.set(false)
                            class="-me-2 px-2 py-1 text-sm uppercase tracking-capitales text-encre-douce hover:text-encre"
                        >
                            "OK"
                        </button>
                    </div>

                    <LesReglages preferences />
            </div>
        </Show>
    }
}

/// La note sous un groupe — la voix de l'app, reprise mot pour mot.
const NOTE: &str = "mt-3 mb-8 text-sm leading-relaxed text-encre-douce last:mb-0";

/// Les quatre thèmes, et chacun se montre tel qu'il est.
///
/// ## Une pastille ne décrit pas sa peau, elle la porte
///
/// Le réflexe serait de mettre une couleur en dur dans chaque pastille. Ce
/// serait la quatre-vingt-unième valeur transcrite, celle que tout le portage
/// existe pour éviter — et elle mentirait au premier thème retouché dans l'app.
///
/// Chaque pastille porte donc **son propre `data-theme`**, et ses couleurs
/// viennent de `var(--ont-*)` : les mêmes variables que la page, résolues dans
/// son sous-arbre à elle. Une pastille est un échantillon vivant du thème
/// qu'elle propose.
///
/// Le détour par `--color-*` ne marcherait pas pour ça, et c'est mécanique :
/// une propriété personnalisée se résout **une fois**, sur l'élément qui la
/// déclare, puis s'hérite déjà substituée. `--color-nuit` étant déclarée sur
/// `:root`, elle vaut le fond de la page partout dans le document — y compris
/// sous un `data-theme` différent. On vise donc `--ont-*` directement.
///
/// ## Des boutons, et un `radiogroup`
///
/// Quatre choix mutuellement exclusifs : c'est un groupe de boutons radio pour
/// qui écoute la page, même si rien n'y ressemble à un rond à cocher. Sans les
/// rôles, un lecteur d'écran annoncerait quatre boutons sans dire lequel est
/// actif ni qu'ils s'excluent.
#[component]
fn ChoixDeTheme(preferences: RwSignal<Preferences>) -> impl IntoView {
    view! {
        <div role="radiogroup" aria-label="Thème" class="mt-1 grid grid-cols-4 gap-2">
            {Theme::TOUS
                .into_iter()
                .map(|theme| {
                    let actif = Signal::derive(move || preferences.get().theme == theme);
                    view! {
                        <button
                            type="button"
                            role="radio"
                            aria-checked=move || actif.get().to_string()
                            aria-label=theme.libelle()
                            on:click=move |_| preferences.update(|p| p.theme = theme)
                            data-theme=theme.attribut()
                            // Le fond, l'encre et le filet sont ceux du thème
                            // proposé, lus dans le sous-arbre de la pastille.
                            // L'anneau, lui, est l'or du thème **courant** :
                            // c'est la page qui désigne, pas l'échantillon.
                            class="flex flex-col items-center gap-1.5 rounded-xl border bg-[var(--ont-background)] px-2 py-3 text-[var(--ont-ink)] transition-[box-shadow,border-color] border-[var(--ont-separator)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent motion-reduce:transition-none"
                            class=("ring-2", move || actif.get())
                            class=("ring-accent", move || actif.get())
                            class=("ring-offset-2", move || actif.get())
                            class=("ring-offset-surface-haute", move || actif.get())
                        >
                            // Un « Aa » dans la fonte du corps, un point d'or :
                            // le texte et l'accentuation, c'est-à-dire les deux
                            // choses que le thème change et qu'on lit.
                            <span aria-hidden="true" class="font-corps text-lg leading-none">
                                "Aa"
                            </span>
                            <span
                                aria-hidden="true"
                                class="block size-1.5 rounded-full bg-[var(--ont-accent)]"
                            ></span>
                        </button>
                    }
                })
                .collect_view()}
        </div>
        // Les noms sous les pastilles, hors du groupe : une pastille se
        // reconnaît à sa couleur, mais « Sombre » et « Mystique » sont deux
        // nuits, et rien ne les distingue à deux centimètres.
        <div aria-hidden="true" class="mt-1.5 grid grid-cols-4 gap-2">
            {Theme::TOUS
                .into_iter()
                .map(|theme| {
                    view! {
                        <span
                            class="block text-center text-sm leading-tight"
                            class=("text-accent", move || preferences.get().theme == theme)
                            class=("text-encre-douce", move || preferences.get().theme != theme)
                        >
                            {theme.libelle()}
                        </span>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Le curseur de taille — **les bornes de celui de l'app**, 11 à 28.
///
/// ## Pourquoi un curseur et pas deux boutons
///
/// L'app emploie `Slider(in: 11...28, step: 1)`, et le geste compte : on
/// cherche une taille en regardant le texte bouger, pas en comptant des
/// appuis. Dix-huit crans au bouton, ce sont dix-sept allers-retours entre le
/// doigt et l'œil.
///
/// Les deux « A » qui l'encadrent sont ceux d'iOS. Ils ne sont pas décoratifs :
/// ils disent le **sens** du curseur sans un mot, et ils le disent à qui ne
/// lit pas encore confortablement la page — ce qui est précisément le lecteur
/// qui cherche ce réglage.
///
/// ## Ce qu'il touche
///
/// `--lecture`, et rien d'autre. La variable n'est lue que par `.liseuse` : ni
/// la navigation, ni le fil d'Ariane, ni ce panneau ne bougent. C'est la règle
/// de l'app, et sa raison est écrite là-bas — *un lecteur atteint de
/// kératocône monte le corps du texte très haut pour lire, et n'a aucune raison
/// de faire enfler du même geste une barre latérale.*
/// Le menu des fontes — **les sept de l'app**, et chacune s'écrit dans la sienne.
///
/// ## Une ligne qui se compose dans ce qu'elle propose
///
/// C'est le même principe que les pastilles de thème, appliqué à la lettre :
/// une ligne qui dit « Spectral » en Literata ne dit rien. Chaque ligne porte
/// donc son propre `data-fonte` **et** la classe `liseuse` — parce que c'est là
/// que la règle vit —, et se lit dans la fonte qu'elle offre.
///
/// Le nom seul ne suffirait pas à choisir : « Newsreader » ne dit rien à qui
/// n'est pas typographe. La note de l'app est donc reprise **mot pour mot** —
/// elle a été écrite pour ça, et un lecteur qui passe du téléphone au site doit
/// retrouver les mêmes phrases.
#[component]
fn ChoixDeFonte(preferences: RwSignal<Preferences>) -> impl IntoView {
    view! {
        <div role="radiogroup" aria-label="Fonte" class="mt-1 flex flex-col">
            {Fonte::TOUTES
                .into_iter()
                .map(|fonte| {
                    let actif = Signal::derive(move || preferences.get().fonte == fonte);
                    view! {
                        <button
                            type="button"
                            role="radio"
                            aria-checked=move || actif.get().to_string()
                            on:click=move |_| preferences.update(|p| p.fonte = fonte)
                            data-fonte=fonte.attribut()
                            class="liseuse -mx-2 flex items-baseline gap-3 rounded-xl px-2 py-2.5 text-start transition-colors hover:bg-aubergine/40 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
                            class=("text-accent", move || actif.get())
                            class=("text-encre", move || !actif.get())
                        >
                            // Le nom, dans sa propre lettre. `font-corps` lit
                            // `--font-corps`, que le `data-fonte` de ce bouton
                            // vient de redéclarer pour son sous-arbre.
                            <span class="font-corps text-base leading-none">{fonte.libelle()}</span>
                            <span class="flex-1 text-sm leading-tight text-encre-douce">
                                {fonte.note()}
                            </span>
                            // La coche, à la fin : elle confirme, elle n'annonce
                            // pas. Masquée à l'oreille — `aria-checked` le dit
                            // déjà, et le redire ferait « coché, coché ».
                            <span
                                aria-hidden="true"
                                class="w-3 text-accent"
                                class=("opacity-0", move || !actif.get())
                            >"·"</span>
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn TailleDuTexte(preferences: RwSignal<Preferences>) -> impl IntoView {
    let corps = Signal::derive(move || preferences.get().corps);

    view! {
        <div class="mt-1 flex items-center gap-3">
            // Les deux repères. `aria-hidden` : « A » et « A » à l'oreille ne
            // disent rien, et le curseur porte déjà son nom.
            <span aria-hidden="true" class="font-corps text-sm leading-none text-encre-douce">
                "A"
            </span>
            <input
                type="range"
                min=Theme::CORPS_MINIMUM.to_string()
                max=Theme::CORPS_MAXIMUM.to_string()
                step="1"
                aria-label="Taille du texte"
                // La valeur **et** le texte de la valeur : un lecteur d'écran
                // annoncerait « 19 » sans dire de quoi, là où « 19 points »
                // situe. C'est l'unité de l'app, et elle est la même ici.
                aria-valuetext=move || format!("{} points", corps.get())
                prop:value=move || corps.get().to_string()
                on:input=move |evenement| {
                    let brut = event_target_value(&evenement);
                    if let Ok(valeur) = brut.parse::<u8>() {
                        // Le serrage est ici **et** dans le script de l'en-tête :
                        // un `<input>` se pilote au clavier, et rien n'empêche
                        // une valeur hors bornes d'arriver par un autre chemin.
                        let valeur = valeur.clamp(Theme::CORPS_MINIMUM, Theme::CORPS_MAXIMUM);
                        preferences.update(|p| p.corps = valeur);
                    }
                }
                class="curseur h-6 flex-1 cursor-pointer appearance-none bg-transparent focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-accent"
            />
            <span aria-hidden="true" class="font-corps text-2xl leading-none text-encre-douce">
                "A"
            </span>
        </div>
    }
}

/// Le curseur d'interligne — le second de la section « Corps ».
///
/// ## Ses repères ne sont pas deux « A »
///
/// Celui de la taille en porte deux, petit et grand, et c'est lisible : la
/// chose réglée *est* la taille d'une lettre. L'interligne ne se montre pas
/// dans une lettre — il se montre dans **l'écart entre deux**. Les repères sont
/// donc deux jeux de traits, serrés puis espacés, ce que l'app dessine de la
/// même façon.
///
/// ## Et il compte des crans, pas des dixièmes
///
/// La valeur voyage en entier — neuf crans, comme le `step: 0.1` de l'app. Le
/// texte annoncé au lecteur d'écran, lui, dit le résultat : « interligne 1,68 »
/// situe, « cran 5 » ne dit rien.
#[component]
fn InterligneDuTexte(preferences: RwSignal<Preferences>) -> impl IntoView {
    let crans = Signal::derive(move || preferences.get().interligne);
    let total =
        move || 1.68 + (f64::from(crans.get()) - f64::from(Theme::INTERLIGNE_PAR_DEFAUT)) / 10.0;

    view! {
        <div class="mt-3 flex items-center gap-3">
            <span aria-hidden="true" class="flex w-4 shrink-0 flex-col gap-[2px]">
                <span class="block h-px bg-encre-douce"></span>
                <span class="block h-px bg-encre-douce"></span>
                <span class="block h-px bg-encre-douce"></span>
            </span>
            <input
                type="range"
                min=Theme::INTERLIGNE_MINIMUM.to_string()
                max=Theme::INTERLIGNE_MAXIMUM.to_string()
                step="1"
                aria-label="Interligne"
                aria-valuetext=move || format!("interligne {:.2}", total())
                prop:value=move || crans.get().to_string()
                on:input=move |evenement| {
                    let brut = event_target_value(&evenement);
                    if let Ok(valeur) = brut.parse::<u8>() {
                        let valeur = valeur
                            .clamp(Theme::INTERLIGNE_MINIMUM, Theme::INTERLIGNE_MAXIMUM);
                        preferences.update(|p| p.interligne = valeur);
                    }
                }
                class="curseur h-6 flex-1 cursor-pointer appearance-none bg-transparent focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-accent"
            />
            <span aria-hidden="true" class="flex w-4 shrink-0 flex-col gap-[5px]">
                <span class="block h-px bg-encre-douce"></span>
                <span class="block h-px bg-encre-douce"></span>
                <span class="block h-px bg-encre-douce"></span>
            </span>
        </div>
    }
}

#[component]
fn Groupe(#[prop(into)] titre: String, children: Children) -> impl IntoView {
    view! {
        <fieldset class="m-0 border-0 p-0">
            <legend class="mb-2 p-0 text-sm uppercase tracking-capitales text-encre-douce">
                {titre}
            </legend>
            <div class="flex flex-col">{children()}</div>
        </fieldset>
    }
}

/// Un interrupteur.
///
/// Une vraie case à cocher, cachée sous un dessin. C'est elle qui porte l'état
/// pour le clavier et pour un lecteur d'écran ; un `div` avec `role="switch"`
/// aurait demandé de réimplémenter à la main l'espace, le tabulateur et
/// l'annonce — et l'une des trois aurait fini par manquer.
///
/// Le libellé enveloppe la case : toute la ligne devient donc la cible, ce qui
/// compte sur un téléphone où un interrupteur de 40 px se rate.
#[component]
fn Bascule(
    #[prop(into)] libelle: String,
    actif: Signal<bool>,
    au_changement: impl Fn(bool) + 'static + Send + Sync,
) -> impl IntoView {
    view! {
        <label class="group flex cursor-pointer items-center justify-between gap-6 py-3">
            <span class="text-[0.95em] text-encre">{libelle}</span>

            <input
                type="checkbox"
                class="peer sr-only"
                prop:checked=move || actif.get()
                on:change=move |evenement| {
                    au_changement(event_target_checked(&evenement));
                }
            />

            // La glissière. `peer-checked` la suit sans qu'on ait à recalculer
            // une classe en Rust — l'état visuel est celui de la case, donc il
            // ne peut pas en diverger.
            <span
                aria-hidden="true"
                class="relative h-6 w-11 shrink-0 rounded-full border border-filet bg-nuit transition-colors peer-checked:border-or/50 peer-checked:bg-aubergine peer-focus-visible:outline peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-accent"
            >
                // `peer-*` exige une **sœur**, or la pastille est fille de la
                // glissière. C'est donc `group-has-[:checked]` qui la commande,
                // depuis le label qui porte `group`.
                <span class="absolute top-1/2 start-0.5 size-4 -translate-y-1/2 rounded-full bg-encre-douce transition-transform group-has-[:checked]:translate-x-5 group-has-[:checked]:bg-accent"></span>
            </span>
        </label>
    }
}

/// Toute page qui compose un lecteur de préférences en fournit.
///
/// ## Pourquoi une garde de plus, alors qu'il y a déjà un `debug_assert!`
///
/// Le `debug_assert!` de [`preferences`] a trouvé la panne de `/fr/webapp` — mais
/// seulement parce qu'un humain a ouvert la page en développement. Il ne s'arme
/// **pas en `--release`**, et la CI construit en release : elle appelait cette
/// route, recevait `200`, et l'annonçait saine. La page l'était ; le réglage,
/// non.
///
/// C'est le motif du journal, une fois de plus — *le format de sortie survit à
/// l'absence de mesure*. Un `200` bien formé ne dit rien de ce qui, dans la
/// page, ne répond plus.
///
/// ## Pourquoi le relevé est transitif, et pourquoi c'est le fond du problème
///
/// La correction du 25 août avait relevé qui appelait `preferences()`
/// **directement** : `livre.rs`, `fiche.rs`, `passage.rs`. Elle a manqué
/// `lire.rs`, qui ne cite ni `preferences` ni `nom_d_unite` — c'est `Sommaire`
/// qu'il compose, et c'est `Sommaire` qui lit.
///
/// Un relevé par appel direct ne peut pas voir ça, et aucune relecture non
/// plus : le lien tient sur deux fichiers qu'on n'ouvre pas en même temps.
/// Le test ferme donc la **fermeture transitive** — un composant qui compose un
/// lecteur est un lecteur — puis exige le fournisseur des pages concernées.
///
/// La borne du calcul est le nombre de composants : chaque tour en ajoute au
/// moins un, sinon il s'arrête.
#[cfg(all(test, feature = "ssr"))]
mod contrat {
    use std::collections::{HashMap, HashSet};
    use std::path::Path;

    /// Les composants d'un dossier, avec la source de chacun.
    ///
    /// Le découpage se fait sur `#[component]` : c'est la marque que Leptos
    /// exige, donc elle ne peut pas manquer sur un composant réel — un relevé
    /// fondé sur une convention de nommage, lui, raterait le jour où quelqu'un
    /// écrit une fonction auxiliaire en majuscule.
    fn composants(dossier: &Path) -> HashMap<String, String> {
        let mut trouves = HashMap::new();
        for entree in std::fs::read_dir(dossier).expect("un dossier de composants") {
            let chemin = entree.expect("une entrée").path();
            if chemin.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let source = std::fs::read_to_string(&chemin).expect("un fichier");
            for morceau in source.split("#[component]").skip(1) {
                let Some(apres) = morceau.split("fn ").nth(1) else {
                    continue;
                };
                let nom: String = apres
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                if !nom.is_empty() {
                    trouves.insert(nom, morceau.to_string());
                }
            }
        }
        trouves
    }

    /// Les composants que `source` compose — `<Sommaire`, `<ListeDUnites`…
    fn composes(source: &str, connus: &HashMap<String, String>) -> HashSet<String> {
        connus
            .keys()
            .filter(|nom| source.contains(&format!("<{nom}")))
            .cloned()
            .collect()
    }

    #[test]
    fn chaque_page_qui_lit_les_preferences_les_fournit() {
        let racine = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/interface");
        let design = composants(&racine.join("design"));
        let pages = composants(&racine.join("pages"));

        // Les lecteurs directs, puis tous ceux qui en composent un.
        let mut lecteurs: HashSet<String> = design
            .iter()
            .filter(|(_, source)| source.contains("preferences()"))
            .map(|(nom, _)| nom.clone())
            .collect();

        assert!(
            !lecteurs.is_empty(),
            "aucun lecteur de préférences relevé — le relevé est cassé"
        );

        for _ in 0..design.len() {
            let avant = lecteurs.len();
            for (nom, source) in &design {
                if !lecteurs.contains(nom) && !composes(source, &design).is_disjoint(&lecteurs) {
                    lecteurs.insert(nom.clone());
                }
            }
            if lecteurs.len() == avant {
                break;
            }
        }

        for (page, source) in &pages {
            let lus: Vec<&String> = composes(source, &design)
                .iter()
                .filter(|nom| lecteurs.contains(*nom))
                .map(|nom| design.get_key_value(nom).expect("un composant connu").0)
                .collect();

            if lus.is_empty() {
                continue;
            }
            assert!(
                source.contains("fournir_preferences()"),
                "la page `{page}` compose {lus:?}, qui lit les réglages de lecture, \
                 et n'appelle pas `fournir_preferences()`. Le réglage n'aura donc aucun \
                 effet sur cette page — et en `--release` rien ne le dira, puisque le \
                 `debug_assert!` ne s'arme pas et que la page rend `200`."
            );
        }
    }
}
/// **Les réglages eux-mêmes**, sans la feuille qui les porte.
///
/// ## Pourquoi ils sont séparés
///
/// Ils ne vivaient que dans la feuille « aA », qui n'est montée que sur un
/// **passage**. Conséquence, et c'est un cul-de-sac que l'auteur a trouvé :
/// depuis la Bible, le Lexique, Qahal ou Chuqqot, **aucun moyen d'y arriver**.
/// La ligne « Réglages de lecture » de l'onglet « Vous » menait d'ailleurs à
/// la Bible — c'est-à-dire à un écran qui n'a pas ce bouton.
///
/// L'app n'a pas ce trou : `YouTab` pousse `ReadingSettingsSheet` comme une
/// **destination**, en plus de la barre d'outils du chapitre. Deux chemins,
/// un seul écran.
///
/// Le site a maintenant les deux aussi — la feuille dans un chapitre, la page
/// `/fr/webapp/reglages` ailleurs — et **un seul jeu de bascules**. Les
/// dupliquer aurait fait deux vérités à tenir d'accord, ce que ce dépôt paie
/// chaque fois qu'il l'a laissé arriver.
///
/// ## Et ils ne demandent aucun compte
///
/// C'était la question de l'auteur : *« est-ce que c'est parce que je ne suis
/// pas connecté ? »* Non. Tout vit dans `localStorage`, sous `ont.lecture`,
/// depuis toujours — le compte ne sert qu'à retrouver ses surlignages d'un
/// appareil à l'autre. Ce qui manquait était un **chemin**, pas un droit.
#[component]
pub fn LesReglages(preferences: RwSignal<Preferences>) -> impl IntoView {
    view! {
    <Groupe titre="Thème">
        <ChoixDeTheme preferences />
    </Groupe>
    <p class=NOTE>
        "Les quatre peaux de l'application, à l'identique. Mystique est née "
        "ici — c'est la nuit d'aubergine du site — et elle a été portée sur le "
        "téléphone ; les trois autres font le chemin inverse."
    </p>

    <Groupe titre="Fonte">
        <ChoixDeFonte preferences />
    </Groupe>
    <p class=NOTE>
        "Les six familles sont embarquées avec le site, en trois coupes "
        "chacune — l'italique de la translittération est dessinée, jamais "
        "penchée à la main. Georgia vient de votre appareil."
    </p>

    // **« Corps » chez l'app**, et les deux curseurs y vont
    // ensemble : la taille et l'interligne se règlent l'un contre
    // l'autre, et les séparer ferait remonter chercher le second
    // après avoir touché le premier.
    <Groupe titre="Corps">
        <TailleDuTexte preferences />
        <InterligneDuTexte preferences />
    </Groupe>
    <p class=NOTE>
        "Elle ne touche que le texte, jamais la navigation ni ce panneau — "
        "une chrome qui grandit avec le corps mange la place où ce texte "
        "s'affiche. Le zoom du navigateur, lui, agrandit tout, et les deux "
        "se multiplient."
    </p>

    <Groupe titre="Disposition">
        <Bascule
            libelle="Versets à la suite"
            actif=Signal::derive(move || preferences.get().continu)
            au_changement=move |v| {
                preferences.update(|p| p.continu = v);
            }
        />
        <Bascule
            libelle="Couper les mots"
            actif=Signal::derive(move || preferences.get().coupure)
            au_changement=move |v| {
                preferences.update(|p| p.coupure = v);
            }
        />
    </Groupe>
    <p class=NOTE>
        "À la suite, les versets coulent en prose et leurs numéros passent en "
        "exposant — c'est la lecture suivie. En blocs, chaque verset se tient "
        "seul : c'est le mode d'étude."
    </p>
    <p class=NOTE>
        "Couper les mots resserre la justification et supprime les lézardes "
        "blanches d'une colonne étroite. Éteint par défaut : la césure hache "
        "les mots, et qui grossit le texte pour le voir se retrouve avec plus "
        "de coupures, pas moins."
    </p>

    // **Le registre est parti**, et ce n'est pas un retrait :
    // il a sa carte dans « Vous ».
    //
    // L'app l'a sorti d'ici délibérément, et son commentaire
    // dit pourquoi : il était rangé « entre la disposition des
    // versets et la taille du texte », c'est-à-dire **avec la
    // typographie**. Or il ne change pas la façon dont le texte
    // se présente — il change **ce que les livres sont
    // appelés**, donc le corpus tel que le lecteur le
    // rencontre.
    //
    // Ses trois paragraphes d'explication pesaient ici plus que
    // tous les autres réglages réunis, sur une feuille qu'on
    // ouvre au milieu d'un chapitre pour éteindre une glose.


    <Groupe titre="Niveaux du texte">
        <Bascule
            libelle="Gloses"
            actif=Signal::derive(move || preferences.get().gloses)
            au_changement=move |v| {
                preferences.update(|p| p.gloses = v);
            }
        />
        <Bascule
            libelle="Translittération et hébreu"
            actif=Signal::derive(move || preferences.get().niveau_3)
            au_changement=move |v| {
                preferences.update(|p| p.niveau_3 = v);
            }
        />
    </Groupe>
    <p class=NOTE>
        "Le corps de la traduction reste toujours visible. Les gloses "
        "explicitent l'implicite hébreu ; le niveau 3 donne le mot original."
    </p>
    }
}
