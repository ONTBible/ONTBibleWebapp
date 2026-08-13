use leptos::prelude::*;

/// L'état du corpus, en chiffres.
///
/// Ils viennent du `manifest.json` du pipeline, figés à la compilation par
/// `build.rs` — **jamais recopiés**. Un site qui annonce « trois livres » alors
/// que le vault en a cinq ment sans que personne ne s'en aperçoive, parce que
/// rien ne compare les deux.
///
/// Le premier chiffre est **3 sur 70**, et c'est volontaire qu'il soit le
/// premier : un projet en cours qui cache son avancement se fait démasquer par
/// le premier lecteur qui cherche un livre absent. L'annoncer d'emblée en fait
/// une promesse au lieu d'un manque.
#[component]
pub fn Chiffres() -> impl IntoView {
    view! {
        <dl class="grid grid-cols-2 gap-px overflow-hidden rounded-carte border border-filet bg-filet sm:grid-cols-4">
            <Chiffre
                // Une espace insécable fine autour de la barre : le nombre est
                // une seule valeur, il ne se coupe pas en deux lignes.
                valeur=format!("{} / {}", env!("CORPUS_LIVRES_ECRITS"), env!("CORPUS_LIVRES"))
                libelle="livres"
            />
            <Chiffre valeur=env!("CORPUS_UNITES") libelle="unités" />
            <Chiffre valeur=env!("CORPUS_VERSETS") libelle="versets" />
            <Chiffre valeur=env!("CORPUS_LEXIQUE") libelle="entrées de lexique" />
        </dl>
    }
}

#[component]
fn Chiffre(#[prop(into)] valeur: String, #[prop(into)] libelle: String) -> impl IntoView {
    view! {
        <div class="bg-surface px-4 py-6 text-center">
            <dd class="chiffres-tableau font-titre text-2xl whitespace-nowrap text-accent">{valeur}</dd>
            <dt class="mt-1 text-sm text-encre-douce">{libelle}</dt>
        </div>
    }
}
