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
/// ## Elle reste montée, comme la feuille de réglages
///
/// Un `<Show>` l'arracherait du document, et rien ne peut transiter sur ce qui
/// n'existe plus : la fermeture serait sèche là où l'ouverture est douce, ce
/// qui se remarque davantage qu'une barre jamais animée. Elle est donc toujours
/// là, et **inerte** quand la sélection est vide.
///
/// `inert` est rendu **absent** et non « faux » : c'est un attribut booléen,
/// donc `inert="false"` rendrait la barre inerte tout autant. Sans lui, ses
/// boutons resteraient dans l'ordre de tabulation et dans l'arbre
/// d'accessibilité — invisibles et pourtant atteignables.
///
/// ## Ce qu'elle ne fait pas
///
/// Elle ne se rend **que côté navigateur**. Sans JavaScript, des boutons qui ne
/// copient rien seraient un mensonge — pire qu'une absence, parce qu'on les
/// essaie. Le serveur rend le texte entier, ce qui est le rendu honnête pour qui
/// n'a pas de JavaScript et ce qu'un moteur doit indexer.
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
    let textes = StoredValue::new(textes);

    let vide = move || selection.with(|s| s.is_empty());
    let combien = move || selection.with(|s| s.len());

    let renvoi_courant = move || selection.with(|s| livre.with_value(|l| renvoi(l, chapitre, s)));

    // L'adresse partagée. Elle porte le domaine en entier — un lien relatif ne
    // se colle pas dans une messagerie, et c'est le domaine qui déclenche
    // l'ouverture de l'app sur les deux plateformes.
    let adresse = move || {
        let numeros: Vec<u32> = selection.with(|s| s.iter().copied().collect());
        let parametre = crate::domaine::selection::parametre(&numeros);
        chemin.with_value(|c| {
            if parametre.is_empty() {
                format!("{}{c}", crate::interface::tete::ORIGINE)
            } else {
                format!("{}{c}?v={parametre}", crate::interface::tete::ORIGINE)
            }
        })
    };

    // Ce qu'on copie : le texte, puis le renvoi, puis le lien.
    //
    // Dans cet ordre parce que c'est celui d'une citation — on lit le verset
    // avant de savoir d'où il vient. Le lien en dernier, parce qu'une messagerie
    // qui en fait un aperçu le prend là où il est.
    let a_copier = move || {
        let numeros: Vec<u32> = selection.with(|s| s.iter().copied().collect());
        let corps = textes.with_value(|t| {
            numeros
                .iter()
                .filter_map(|n| t.get(n).map(|texte| format!("{n}. {texte}")))
                .collect::<Vec<_>>()
                .join("\n")
        });
        format!("{corps}\n\n— {}\n{}", renvoi_courant(), adresse())
    };

    view! {
        <div
            class="pointer-events-none fixed inset-x-0 bottom-0 z-40 flex justify-center px-4 pb-[calc(1rem+env(safe-area-inset-bottom))] transition-all duration-300 motion-reduce:transition-none"
            class=("translate-y-6", vide)
            class=("opacity-0", vide)
            class=("pointer-events-auto", move || !vide())
            inert=move || vide().then_some("")
            aria-label="Versets sélectionnés"
        >
            <div class="flex w-full max-w-mesure flex-col gap-3 rounded-2xl border border-filet bg-surface-haute/95 p-4 shadow-2xl backdrop-blur-sm sm:flex-row sm:items-center sm:justify-between">
                <p class="chiffres-tableau text-sm text-encre-douce">
                    {renvoi_courant}
                    <span class="ms-2 text-encre-douce/70">
                        "(" {combien} ")"
                    </span>
                </p>

                <div class="flex flex-wrap items-center gap-2">
                    <ActionDeSelection libelle="Copier" texte=Callback::new(move |()| a_copier()) />
                    <ActionDeSelection libelle="Lien" texte=Callback::new(move |()| adresse()) />
                    <button
                        type="button"
                        class="rounded-full px-3 py-2 text-sm uppercase tracking-capitales text-encre-douce transition-colors hover:text-encre"
                        on:click=move |_| selection.update(|s| s.clear())
                    >
                        "Effacer"
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Un bouton qui met du texte dans le presse-papier, et le dit.
///
/// ## Pourquoi il annonce son succès
///
/// Copier est un geste **sans retour visible** : rien ne bouge à l'écran, et
/// l'on ne sait pas si ça a marché avant de coller ailleurs. Le libellé bascule
/// donc en « Copié » pendant deux secondes — c'est le seul accusé de réception
/// possible, et son absence est ce qui fait cliquer trois fois.
///
/// `aria-live="polite"` le dit aussi à un lecteur d'écran, qui ne voit pas le
/// changement de mot.
#[component]
fn ActionDeSelection(
    /// Ce que le bouton dit au repos.
    libelle: &'static str,
    /// Ce qu'il met dans le presse-papier, calculé au moment du clic.
    texte: Callback<(), String>,
) -> impl IntoView {
    let copie = RwSignal::new(false);

    let au_clic = move |_| {
        let _contenu = texte.run(());
        #[cfg(feature = "hydrate")]
        {
            // `writeText` rend une promesse qu'on ne suit pas : son échec ne
            // peut venir que d'un refus de permission, et il n'y a rien à
            // proposer au lecteur dans ce cas — le texte est déjà à l'écran,
            // il peut le sélectionner à la main.
            if let Some(fenetre) = web_sys::window() {
                let _ = fenetre.navigator().clipboard().write_text(&_contenu);
            }
        }
        copie.set(true);
        #[cfg(feature = "hydrate")]
        set_timeout(move || copie.set(false), std::time::Duration::from_secs(2));
    };

    view! {
        <button
            type="button"
            class="rounded-full border border-or/50 px-4 py-2 text-sm uppercase tracking-capitales text-accent transition-colors hover:border-or hover:bg-aubergine/40"
            aria-live="polite"
            on:click=au_clic
        >
            {move || if copie.get() { "Copié" } else { libelle }}
        </button>
    }
}
