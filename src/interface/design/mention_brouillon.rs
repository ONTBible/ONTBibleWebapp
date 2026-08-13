use leptos::prelude::*;

/// La mention qu'un texte n'est pas arrêté.
///
/// Le vault compte aujourd'hui six unités en brouillon contre trente-trois
/// verrouillées, et le site montre **les deux**. Cacher les brouillons
/// donnerait un corpus plus petit qu'il n'est, et un lien partagé vers un
/// chapitre en cours tomberait sur un 404 — ce qui se lit comme « ce texte
/// n'existe pas » alors qu'il existe et qu'il se lit.
///
/// Elle est donc **annoncée**, et c'est tout ce qu'elle fait. Pas d'avertissement
/// solennel, pas de bandeau rouge : une ligne qui dit ce qui est. Le lecteur
/// d'une traduction en cours a le droit de savoir qu'elle est en cours ; il n'a
/// pas besoin qu'on lui fasse peur.
///
/// Le bordeaux clair, et non l'or : l'or promet une fiche partout ailleurs sur
/// le site, et le laisser marquer autre chose brouillerait la promesse. Le
/// `#D87994` est déjà la couleur du terme important — celle qui **marque sans
/// rien promettre**, ce qui est exactement le propos ici.
#[component]
pub fn MentionBrouillon(
    /// Une version discrète, pour une liste d'unités où la mention se répète.
    ///
    /// Dans un sommaire de dix-neuf lignes dont six portent la mention, la
    /// forme pleine devient un motif et cesse d'informer.
    #[prop(optional)]
    breve: bool,
) -> impl IntoView {
    view! {
        <p
            class="m-0 inline-flex items-center gap-2.5 rounded-full border border-important/30 text-important"
            class=("px-4", !breve)
            class=("py-1.5", !breve)
            class=("text-sm", !breve)
            class=("px-2.5", breve)
            class=("py-0.5", breve)
            class=("text-[0.72em]", breve)
        >
            // Le point tient lieu de pictogramme. Un vrai symbole d'alerte
            // dirait « attention » ; ce texte ne demande aucune prudence, il
            // demande d'être lu pour ce qu'il est.
            <span aria-hidden="true" class="size-1.5 shrink-0 rounded-full bg-current"></span>
            <span class="uppercase tracking-capitales">
                {if breve { "Brouillon" } else { "Traduction en cours" }}
            </span>
        </p>
    }
}
