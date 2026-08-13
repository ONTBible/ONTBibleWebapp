use leptos::prelude::*;

use crate::domaine::lecture::Preferences;

/// Où le navigateur retient les réglages.
///
/// Un espace de noms préfixé : le site en aura d'autres, et une clé nue comme
/// `lecture` finirait par entrer en conflit avec quelque chose.
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
        None => Signal::stored(Preferences::default()),
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

/// Le panneau de lecture — éteindre les niveaux du texte.
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
/// Le panneau est rendu par le navigateur seulement. Sans JavaScript, des
/// interrupteurs qui ne commutent rien seraient un mensonge — pire qu'une
/// absence, parce qu'on les essaie. La page reste alors ce qu'elle est : le
/// texte entier, tous niveaux montrés.
#[component]
pub fn ReglagesDeLecture(preferences: RwSignal<Preferences>) -> impl IntoView {
    // Faux au rendu du serveur, vrai dès que le navigateur a repris la main.
    // Les deux côtés partent donc du même balisage, et le panneau se pose après
    // — sans désaccord d'hydratation.
    let utilisable = RwSignal::new(false);
    Effect::new(move |_| utilisable.set(true));

    view! {
        <Show when=move || utilisable.get()>
            <details class="mb-12 rounded-carte border border-filet bg-surface/40">
                <summary class="flex cursor-pointer list-none items-center gap-3 px-6 py-4 text-sm uppercase tracking-capitales text-encre-douce marker:hidden hover:text-encre">
                    <span aria-hidden="true" class="font-titre text-[1.15em] leading-none">
                        "aA"
                    </span>
                    "Lecture"
                </summary>

                <div class="border-t border-filet/60 px-6 py-6">
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
                        "seul : c'est le mode d'étude."
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
                        "explicitent l'implicite hébreu ; le niveau 3 donne le mot original."
                    </p>
                </div>
            </details>
        </Show>
    }
}

/// La note sous un groupe — la voix de l'app, reprise mot pour mot.
const NOTE: &str = "mt-3 mb-8 text-sm leading-relaxed text-encre-douce last:mb-0";

#[component]
fn Groupe(#[prop(into)] titre: String, children: Children) -> impl IntoView {
    view! {
        <fieldset class="m-0 border-0 p-0">
            <legend class="mb-3 p-0 text-sm uppercase tracking-capitales text-accent">
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
