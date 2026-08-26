use leptos::ev;
use leptos::prelude::*;

use crate::domaine::lecture::Preferences;

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
                "les réglages de lecture sont lus sans avoir été fournis : \
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

/// Les réglages de lecture — un bouton qui suit, et une feuille qui monte.
///
/// ## Pourquoi il flotte
///
/// Une première version posait le panneau **en haut du chapitre**. Ça ne tenait
/// pas : un chapitre fait jusqu'à quarante-six versets, et l'on décide
/// d'éteindre les gloses au milieu de la lecture, pas avant de l'avoir
/// commencée. Un réglage qu'il faut remonter chercher n'en est plus un.
///
/// Le bouton reste donc à portée, en bas, et la feuille monte par-dessus le
/// texte — comme la feuille « aA » de l'app, et pour la même raison.
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
                class="halo se-poser fixed end-6 z-50 flex size-14 items-center justify-center rounded-full border border-or/30 bg-surface-haute text-accent transition-[transform,border-color,box-shadow,opacity] duration-200 ease-out hover:border-or/60 active:scale-95 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent motion-reduce:transition-none"
                // Il s'efface pendant une sélection, et les deux raisons
                // comptent.
                //
                // La première est mécanique : la barre de sélection occupe
                // toute la largeur en bas, ce bouton est à `1.5rem` du bas à
                // droite — **ils se chevauchent**. Vu au simulateur, pas déduit
                // du code : les deux valeurs sont dans deux fichiers qu'on
                // n'ouvre pas ensemble.
                //
                // La seconde est de propos : on ne règle pas sa typographie
                // pendant qu'on choisit des versets à partager. Laisser les
                // deux à l'écran ferait deux actions principales, donc aucune.
                //
                // `inert` en plus de l'opacité : un bouton transparent reste
                // cliquable et tabulable — c'est la même règle que pour la
                // feuille, et l'oublier mettrait un piège invisible sous la
                // barre.
                class=("opacity-0", selection_active)
                class=("pointer-events-none", selection_active)
                inert=move || selection_active().then_some("")
                style="bottom: calc(1.5rem + env(safe-area-inset-bottom))"
            >
                <span aria-hidden="true" class="font-titre text-xl leading-none">"aA"</span>
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
                class="fixed inset-x-0 bottom-0 z-50 max-h-[85dvh] overflow-y-auto rounded-t-carte border-t border-filet bg-surface-haute px-6 pt-6 transition-[transform,opacity] duration-300 ease-out sm:inset-x-auto sm:end-6 sm:bottom-24 sm:w-96 sm:origin-bottom-right sm:rounded-carte sm:border motion-reduce:transition-none"
                class=("translate-y-full", move || !ouvert.get())
                class=("opacity-0", move || !ouvert.get())
                class=("pointer-events-none", move || !ouvert.get())
                class=("sm:translate-y-2", move || !ouvert.get())
                class=("sm:scale-95", move || !ouvert.get())
                class=("translate-y-0", move || ouvert.get())
                class=("opacity-100", move || ouvert.get())
                class=("sm:scale-100", move || ouvert.get())
                style="padding-bottom: calc(1.5rem + env(safe-area-inset-bottom))"
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

                    <Groupe titre="Disposition">
                        <Bascule
                            libelle="Versets à la suite"
                            actif=Signal::derive(move || preferences.get().continu)
                            au_changement=move |v| {
                                preferences.update(|p| p.continu = v);
                            }
                        />
                    </Groupe>
                    <p class=NOTE>
                        "À la suite, les versets coulent en prose et leurs numéros passent en "
                        "exposant — c'est la lecture suivie. En blocs, chaque verset se tient "
                        "seul : c'est le mode d'étude."
                    </p>

                    <Groupe titre="Nom des livres">
                        <Bascule
                            libelle="Le français reçu"
                            actif=Signal::derive(move || preferences.get().francais)
                            au_changement=move |v| {
                                preferences.update(|p| p.francais = v);
                            }
                        />
                    </Groupe>
                    <p class=NOTE>
                        "Allumé, les livres portent le nom qu'on leur connaît — « Apocalypse », "
                        "« la Loi », « Actes des Apôtres ». Éteint, ils portent ce que leur nom "
                        "hébreu veut dire : « le machazeh de Yohanan », « la Fondation », « les "
                        "gevurot de YHWH par ses neviim »."
                    </p>
                    <p class=NOTE>
                        "L'écart entre les deux n'est pas une nuance de traduction. La torah est "
                        "l'instruction qui vise ; le grec l'a rendue par nomos, le code qui "
                        "contraint, et le français en a hérité « la Loi »."
                    </p>
                    <p class=NOTE>
                        "Ce réglage est une béquille, et il est allumé pour qu'on puisse marcher "
                        "avant de savoir. En l'éteignant, des mots apparaissent que vous n'avez "
                        "peut-être jamais lus — parashah, par exemple, la division que le scribe "
                        "hébreu traçait en laissant un blanc, mille ans avant qu'on numérote des "
                        "chapitres. Ils sont en or : ils se touchent, et ils expliquent."
                    </p>

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
            </div>
        </Show>
    }
}

/// La note sous un groupe — la voix de l'app, reprise mot pour mot.
const NOTE: &str = "mt-3 mb-8 text-sm leading-relaxed text-encre-douce last:mb-0";

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
/// Le `debug_assert!` de [`preferences`] a trouvé la panne de `/fr/lire` — mais
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
