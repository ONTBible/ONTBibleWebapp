use std::collections::BTreeSet;

use leptos::prelude::*;

/// Les versets que le lecteur a désignés à l'écran.
///
/// ## Pourquoi un contexte plutôt qu'un prop
///
/// C'est le même raisonnement que pour les réglages de lecture : un verset est
/// rendu au bout d'une chaîne de composition — page, blocs, bloc, verset — et
/// faire descendre un signal de main en main à travers ces étages obligerait
/// chacun à le connaître, y compris ceux qui n'en font rien.
///
/// ## Un `BTreeSet`, et les trois propriétés comptent
///
/// **Ensemble** : toucher deux fois le même verset ne le compte pas deux fois.
/// **Trié** : `selection::parametre` trie déjà, mais l'affichage lit cet
/// ensemble directement — « 4, 5, 6 » et non l'ordre où l'on a tapé.
/// **Ordonné à la lecture** : le repliement en `1-3,7` dépend de l'ordre, et le
/// faire ici plutôt qu'à chaque appel évite deux tris.
pub type Selection = RwSignal<BTreeSet<u32>>;

/// Installe la sélection pour la page.
///
/// À appeler **une fois**, dans la page qui porte un passage. Comme pour les
/// réglages, le contexte est la seule voie : un composant de texte qui ne le
/// trouve pas doit se comporter comme s'il n'y avait pas de sélection possible,
/// et non planter — le même `Verset` sert sur l'accueil et dans une fiche du
/// lexique, où l'on ne sélectionne rien.
pub fn fournir_selection() -> Selection {
    let selection = RwSignal::new(BTreeSet::new());
    provide_context(selection);
    selection
}

/// La sélection de la page, ou un ensemble vide et **inerte**.
///
/// Le repli diffère de celui des réglages, et c'est délibéré. Là-bas, un signal
/// constant était une panne muette : les bascules ne faisaient rien alors
/// qu'elles étaient à l'écran. Ici, l'absence de contexte signifie qu'il n'y a
/// **pas d'interface de sélection** — sur l'accueil, dans une fiche — donc rien
/// ne promet quoi que ce soit au lecteur. Un signal vide est le comportement
/// juste, pas un défaut déguisé.
///
/// C'est la règle qui distingue les deux : un repli est acceptable quand il
/// rend une page **cohérente**, et pas seulement quand il l'empêche de tomber.
pub fn selection() -> Option<Selection> {
    use_context::<Selection>()
}

/// Bascule un verset dans la sélection.
///
/// Rend `true` s'il y entre, `false` s'il en sort — l'appelant s'en sert pour
/// annoncer le changement à un lecteur d'écran, qui ne voit pas la surbrillance.
pub fn basculer(selection: Selection, numero: u32) -> bool {
    selection
        .try_update(|s| {
            if s.contains(&numero) {
                s.remove(&numero);
                false
            } else {
                s.insert(numero);
                true
            }
        })
        .unwrap_or(false)
}

/// Le libellé d'un renvoi, tel qu'on l'écrit en français.
///
/// « Bereshit 1:4-6 » et non « Bereshit 1:4,5,6 ». C'est ce qui accompagne le
/// texte copié, et c'est ce qu'un lecteur reconnaîtra : la forme classique des
/// renvois bibliques, celle que le sous-titre du chapitre porte déjà.
///
/// Elle passe par `selection::libelle`, qui **cite `parametre`** : le
/// groupement est donc le même dans le renvoi et dans l'adresse, à l'espace
/// française près. Deux repliements distincts feraient dire au renvoi autre
/// chose qu'au lien qu'il accompagne.
pub fn renvoi(livre: &str, chapitre: u32, numeros: &BTreeSet<u32>) -> String {
    let versets: Vec<u32> = numeros.iter().copied().collect();
    if versets.is_empty() {
        return format!("{livre} {chapitre}");
    }
    format!(
        "{livre} {chapitre}:{}",
        crate::domaine::selection::libelle(&versets)
    )
}

/// La barre qui monte quand des versets sont désignés.
///
/// ## Elle recopie `VerseActionBar` de l'app, elle ne la réinvente pas
///
/// Même dessin — une carte arrondie **détachée des quatre bords**, dont le
/// commentaire Swift donne la raison : « le texte continue de courir derrière
/// elle, et la carte se lit comme un objet qu'on peut écarter, pas comme un
/// morceau de l'écran ».
///
/// Mêmes actions, dans le même ordre : **Copier**, **Partager**, **Tout**.
/// L'app en a deux de plus — *Noter* et les couleurs de surlignage — qui
/// demandent un compte et une synchronisation ; elles viendront avec lui. Et
/// *Image*, qui rend un carré de 1080 px, demande un rendu que le navigateur ne
/// fait pas gratuitement.
///
/// ## Ce que le web change, et rien d'autre
///
/// **Le glissement vers le bas n'existe pas** : c'est un geste tactile, et la
/// webapp se lit aussi à la souris. Le bouton *Effacer* fait le même travail
/// pour les deux, ce que la poignée de l'app ne pourrait pas faire sur un
/// ordinateur.
///
/// **Le partage passe par `navigator.share` quand il existe**, et retombe sur
/// une copie sinon — un ordinateur de bureau n'a pas de feuille de partage
/// système. C'est le seul endroit où la webapp ne peut pas faire ce que fait
/// l'app, et elle le remplace au lieu de le taire.
///
/// ## Elle reste montée, comme la feuille de réglages
///
/// Un `<Show>` l'arracherait du document, et rien ne peut transiter sur ce qui
/// n'existe plus : la fermeture serait sèche là où l'ouverture est douce. Elle
/// est donc toujours là, et **inerte** quand la sélection est vide.
///
/// `inert` est rendu **absent** et non « faux » : c'est un attribut booléen,
/// donc `inert="false"` rendrait la barre inerte tout autant. Sans lui, ses
/// boutons resteraient dans l'ordre de tabulation et dans l'arbre
/// d'accessibilité — invisibles et pourtant atteignables.
#[component]
pub fn BarreDeSelection(
    /// La sélection courante, celle que la page a fournie.
    selection: Selection,
    /// Le nom du livre tel qu'on l'écrit dans un renvoi — « Bereshit ».
    #[prop(into)]
    livre: String,
    /// Le rang de l'unité. Zéro pour une introduction, qui n'en porte pas.
    chapitre: u32,
    /// Le chemin de la page, sans paramètre — « /fr/lire/bereshit/bereshit-1 ».
    #[prop(into)]
    chemin: String,
    /// Le texte de chaque verset, par numéro. Sert à ce qu'on copie.
    textes: std::collections::BTreeMap<u32, String>,
) -> impl IntoView {
    let livre = StoredValue::new(livre);
    let chemin = StoredValue::new(chemin);
    let tous: Vec<u32> = textes.keys().copied().collect();
    let tous = StoredValue::new(tous);
    let textes = StoredValue::new(textes);

    let vide = move || selection.with(|s| s.is_empty());

    let renvoi_courant = move || selection.with(|s| livre.with_value(|l| renvoi(l, chapitre, s)));

    // L'adresse partagée, avec le domaine en entier : un lien relatif ne se
    // colle pas dans une messagerie, et c'est le domaine qui déclenche
    // l'ouverture de l'app sur les deux plateformes.
    let adresse = move || {
        let numeros: Vec<u32> = selection.with(|s| s.iter().copied().collect());
        let parametre = crate::domaine::selection::parametre(&numeros);
        chemin.with_value(|c| {
            let base = format!("{}{c}", crate::interface::tete::ORIGINE);
            if parametre.is_empty() {
                base
            } else {
                format!("{base}?v={parametre}")
            }
        })
    };

    // Le texte partagé, **au format de l'app** — `shareText` dans
    // `ChapterView.swift` : les versets joints par une espace, sans leurs
    // numéros, puis le renvoi.
    //
    // Sans numéros parce qu'une citation n'en porte pas : « 1. Quand Elohim »
    // se colle dans un message comme une capture d'écran de logiciel. Et le
    // lien n'y est **pas** — l'app le passe comme un second objet de partage,
    // pour que la messagerie en tire un aperçu au lieu de l'afficher en clair.
    let texte_partage = move || {
        let numeros: Vec<u32> = selection.with(|s| s.iter().copied().collect());
        let corps = textes.with_value(|t| {
            numeros
                .iter()
                .filter_map(|n| t.get(n).map(String::as_str))
                .collect::<Vec<_>>()
                .join(" ")
        });
        format!("{corps}\n\n— {}, La Bible ONT", renvoi_courant())
    };

    view! {
        <div
            class="pointer-events-none fixed inset-x-0 bottom-0 z-40 flex justify-center px-3.5 pb-[calc(0.625rem+env(safe-area-inset-bottom))] transition-all duration-150 ease-out motion-reduce:transition-none"
            class=("translate-y-4", vide)
            class=("opacity-0", vide)
            class=("pointer-events-auto", move || !vide())
            inert=move || vide().then_some("")
            aria-label="Versets sélectionnés"
        >
            // 26 points de rayon, une ombre portée, un filet — les valeurs de
            // `VerseActionBar`. La carte ne touche aucun bord : c'est ce qui la
            // fait lire comme un objet posé sur la page.
            <div class="w-full max-w-mesure rounded-[1.625rem] border border-filet bg-surface-haute px-[1.125rem] pb-3.5 pt-2.5 shadow-2xl">
                // Le renvoi tient la place du titre de navigation, que l'app
                // remplit pendant une sélection — « comme dans Bible Strong ».
                // Sur un site il n'y a pas de barre de navigation à emprunter,
                // donc il vit ici, au-dessus des actions.
                <p class="chiffres-tableau mb-3 text-center text-sm text-encre-douce">
                    {renvoi_courant}
                </p>

                <div class="flex items-stretch">
                    <ActionDeSelection
                        libelle="Copier"
                        texte=Callback::new(move |()| texte_partage())
                        efface_apres=true
                        selection
                    />
                    <ActionDePartage
                        texte=Callback::new(move |()| texte_partage())
                        lien=Callback::new(move |()| adresse())
                    />
                    <ActionSimple
                        libelle="Tout"
                        au_clic=Callback::new(move |()| {
                            tous.with_value(|t| selection.set(t.iter().copied().collect()));
                        })
                    />
                    <ActionSimple
                        libelle="Effacer"
                        au_clic=Callback::new(move |()| selection.update(|s| s.clear()))
                    />
                </div>
            </div>
        </div>
    }
}

/// Une tuile d'action — `ActionTile` de l'app.
///
/// Elles se partagent la largeur à parts égales (`flex-1`), comme le
/// `.frame(maxWidth: .infinity)` de chacune côté Swift. C'est ce qui fait que la
/// barre ne se réorganise pas quand une action apparaît ou disparaît.
#[component]
fn ActionSimple(libelle: &'static str, au_clic: Callback<(), ()>) -> impl IntoView {
    view! {
        <button
            type="button"
            class="flex-1 rounded-xl px-2 py-2 text-sm uppercase tracking-capitales text-encre-douce transition-colors hover:bg-aubergine/40 hover:text-encre focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
            on:click=move |_| au_clic.run(())
        >
            {libelle}
        </button>
    }
}

/// Copier, avec l'accusé de réception que l'app n'a pas besoin de donner.
///
/// Sur iOS, `UIPasteboard` est instantané et le geste est tactile : on sent
/// qu'il a eu lieu. Dans un navigateur, rien ne bouge — et un geste sans retour
/// se répète. Le libellé bascule donc en « Copié » deux secondes.
///
/// L'app **vide la sélection** après avoir copié (`selection.removeAll()`), et
/// on fait pareil : c'est ce qui dit que le geste est consommé.
// `selection` et `efface_apres` ne servent qu'au navigateur : côté serveur, le
// clic n'existe pas. Le `cfg_attr` vaut mieux qu'un préfixe `_`, qui les
// rendrait muettes des deux côtés et ferait perdre le nom au lecteur.
#[cfg_attr(not(feature = "hydrate"), allow(unused_variables))]
#[component]
fn ActionDeSelection(
    libelle: &'static str,
    texte: Callback<(), String>,
    efface_apres: bool,
    selection: Selection,
) -> impl IntoView {
    let copie = RwSignal::new(false);

    let au_clic = move |_| {
        let _contenu = texte.run(());
        #[cfg(feature = "hydrate")]
        if let Some(fenetre) = web_sys::window() {
            let _ = fenetre.navigator().clipboard().write_text(&_contenu);
        }
        copie.set(true);
        #[cfg(feature = "hydrate")]
        {
            let selection = selection;
            set_timeout(
                move || {
                    copie.set(false);
                    if efface_apres {
                        selection.update(|s| s.clear());
                    }
                },
                std::time::Duration::from_millis(900),
            );
        }
    };

    view! {
        <button
            type="button"
            class="flex-1 rounded-xl px-2 py-2 text-sm uppercase tracking-capitales text-accent transition-colors hover:bg-aubergine/40 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
            aria-live="polite"
            on:click=au_clic
        >
            {move || if copie.get() { "Copié" } else { libelle }}
        </button>
    }
}

/// Partager — la feuille du système quand elle existe, une copie sinon.
///
/// `navigator.share` n'existe que sur mobile et dans quelques navigateurs de
/// bureau. **Le repli n'est pas une dégradation cachée** : le libellé dit
/// « Copié » quand c'est ce qui s'est passé, pour qu'on n'aille pas chercher une
/// feuille de partage qui ne viendra pas.
///
/// Le texte et le lien voyagent **séparés**, comme dans l'app : la messagerie
/// prend le texte, et ce qui sait lire une URL en tire un aperçu. Les coller
/// ensemble donnerait une adresse en clair au milieu d'une citation.
#[component]
fn ActionDePartage(texte: Callback<(), String>, lien: Callback<(), String>) -> impl IntoView {
    let etat = RwSignal::new("Partager");

    let au_clic = move |_| {
        let _t = texte.run(());
        let _l = lien.run(());
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::{JsCast, JsValue};
            let Some(fenetre) = web_sys::window() else {
                return;
            };
            let navigateur = fenetre.navigator();
            let donnees = js_sys::Object::new();
            let _ = js_sys::Reflect::set(&donnees, &"text".into(), &JsValue::from_str(&_t));
            let _ = js_sys::Reflect::set(&donnees, &"url".into(), &JsValue::from_str(&_l));

            // `share` n'est pas déclaré par web-sys sur toutes les cibles : on
            // le cherche sur l'objet plutôt que de le supposer. Absent, on
            // copie — et on le dit.
            let partage = js_sys::Reflect::get(&navigateur, &"share".into()).ok();
            let disponible = partage.as_ref().is_some_and(|p| p.is_function());

            if disponible {
                if let Some(f) = partage.and_then(|p| p.dyn_into::<js_sys::Function>().ok()) {
                    let _ = f.call1(&navigateur, &donnees);
                }
            } else {
                let _ = navigateur.clipboard().write_text(&format!("{_t}\n{_l}"));
                etat.set("Copié");
                set_timeout(
                    move || etat.set("Partager"),
                    std::time::Duration::from_millis(1200),
                );
            }
        }
    };

    view! {
        <button
            type="button"
            class="flex-1 rounded-xl px-2 py-2 text-sm uppercase tracking-capitales text-accent transition-colors hover:bg-aubergine/40 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
            aria-live="polite"
            on:click=au_clic
        >
            {move || etat.get()}
        </button>
    }
}
