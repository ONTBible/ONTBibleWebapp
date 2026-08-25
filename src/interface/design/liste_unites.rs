use leptos::prelude::*;

use crate::api::UniteDto;
use crate::interface::design::reglages_de_lecture::preferences;
use crate::interface::design::{MentionBrouillon, Terme};

/// Les unités d'un livre.
///
/// ## Pourquoi « unité » et pas « chapitre »
///
/// Les unités de l'ONT ne coïncident pas avec les chapitres reçus : la
/// septième de Bereshit couvre les chapitres 7 et 8, la huitième couvre
/// 9:1-17. C'est une conséquence de la traduction, pas un choix d'affichage —
/// le découpage suit le mouvement du texte hébreu, pas la numérotation
/// médiévale.
///
/// D'où le **renvoi classique** à droite de chaque ligne. Sans lui, quelqu'un
/// qui cherche « Genèse 9 » ne saurait pas dans laquelle des deux entrer, et
/// conclurait que le corpus est incomplet.
///
/// ## Le nom du livre ne se répète pas
///
/// Le titre d'une unité est « Bereshit 7 ». Le poser vingt fois de suite dans
/// la page de *Bereshit* n'apprend rien et noie le seul chiffre utile. La
/// liste compose donc son libellé à partir du rang.
///
/// Et elle le nomme selon le registre choisi — **`Chapitre 7`** en français
/// reçu, **`Parashah 7`** en glose ONT. La paire est instructive à elle
/// seule : « chapitre » est la division de Stephen Langton, XIIIᵉ siècle,
/// que le §2.3 écarte comme « administrative médiévale — souvent
/// arbitraire » ; la *parashah* est la division native de l'hébreu, attestée
/// à Qumrân, et elle marque qu'un propos se clôt, non qu'un compteur avance.
///
/// Le renvoi classique reste à côté, et c'est lui qui dit la vérité quand le
/// mot français ment : « Chapitre 7 · 7-8 ».
#[component]
pub fn ListeDUnites(livre: String, unites: Vec<UniteDto>) -> impl IntoView {
    view! {
        <ul class="m-0 list-none p-0">
            {unites
                .into_iter()
                .map(|unite| {
                    view! {
                        <li class="border-b border-filet/40 last:border-0">
                            <a
                                href=format!("/fr/lire/{livre}/{}", unite.id)
                                class="flex flex-wrap items-baseline gap-x-4 gap-y-1.5 py-4 no-underline"
                            >
                                {libelle(&unite)}

                                {unite
                                    .reference
                                    .map(|reference| {
                                        view! {
                                            <span class="chiffres-tableau text-[0.86em] text-encre-douce">
                                                {reference}
                                            </span>
                                        }
                                    })}

                                // Le compte de versets pousse le reste à
                                // gauche et se pose au bout de la ligne.
                                <span class="chiffres-tableau ms-auto text-[0.8em] text-encre-douce">
                                    {unite.versets} " v."
                                </span>

                                {unite
                                    .brouillon
                                    .then(|| view! { <MentionBrouillon breve=true /> })}
                            </a>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}

/// Le libellé d'une unité, dans le registre choisi.
///
/// En français reçu, c'est du texte : « Chapitre 7 » ne promet rien et n'a
/// rien à expliquer.
///
/// En glose ONT, **`Parashah` est un intraduisible** — en or, et il ouvre sa
/// fiche. C'est peut-être le premier que le lecteur rencontre : il apparaît
/// au moment précis où l'on retire la béquille du français, et il vaut qu'on
/// puisse le toucher pour savoir ce qu'on vient de gagner.
///
/// Une introduction — rang zéro — garde son titre : elle n'a pas de rang à
/// afficher, et « Chapitre 0 » ne voudrait rien dire.
fn libelle(unite: &UniteDto) -> impl IntoView {
    let titre = unite.titre.clone();
    let n = unite.numero;
    let prefs = preferences();
    move || {
        if n == 0 {
            return view! { <span class="text-accent">{titre.clone()}</span> }.into_any();
        }
        if prefs.get().francais {
            return view! { <span class="text-accent">{MOT_RECU}" "{n}</span> }.into_any();
        }
        view! {
            <span>
                <Terme lemme="parashah">{MOT_ONT}</Terme>
                <span class="text-accent">" " {n}</span>
            </span>
        }
        .into_any()
    }
}

/// Le mot qui nomme une unité, dans chacun des deux registres.
///
/// Ils sont ici et nulle part ailleurs. Deux vues les emploient — le sommaire,
/// qui rend `Parashah` en or et cliquable, et [`nom_d_unite`], qui n'en donne
/// que le texte — et chacune les écrivait pour son compte. Deux copies d'un
/// même mot finissent par diverger : c'est précisément ce qui venait de se
/// produire d'un écran à l'autre, en plus grand.
const MOT_RECU: &str = "Chapitre";
const MOT_ONT: &str = "Parashah";

/// Le nom d'une unité **en texte**, dans le registre choisi.
///
/// Le pendant de [`libelle`] pour les endroits qui ne peuvent pas porter de
/// balise : un `<h1>`, une balise de navigation, un texte de lien.
///
/// ## Pourquoi il existe
///
/// Le calcul vivait dans le seul sommaire. La page de lecture, elle, affichait
/// `chapitre.titre` — le nom ONT brut. Un lecteur touchait donc « Chapitre 2 »
/// et arrivait sur une page intitulée « Bereshit 2 » : deux écrans, un seul
/// calcul, l'autre oublié. C'est le même défaut que la session de l'app a
/// trouvé chez elle entre son sommaire et son sélecteur de renvoi.
///
/// ## Il ne rend pas « Parashah » en or, et c'est délibéré
///
/// Dans le sommaire, `Parashah` est un lien vers sa fiche — le premier
/// intraduisible que beaucoup rencontreront. Dans un titre de page, un lien
/// n'a pas sa place : on ne quitte pas la page qu'on vient d'ouvrir par son
/// propre titre. Le lecteur qui veut la fiche la trouve dans le sommaire d'où
/// il vient, ou dans le lexique.
///
/// ## Il ne convient pas à la balise `<title>`
///
/// Celle-là doit rester le nom ONT : elle est rendue par le serveur, qui ne
/// connaît pas les préférences, et elle sert le référencement et le partage —
/// deux usages où un nom stable vaut mieux qu'un nom juste.
pub fn nom_d_unite(titre: String, numero: u32) -> Signal<String> {
    let prefs = preferences();
    Signal::derive(move || {
        // Rang zéro : une introduction. Elle garde son titre — elle n'a pas de
        // rang, et « Chapitre 0 » ne voudrait rien dire.
        if numero == 0 {
            return titre.clone();
        }
        if prefs.get().francais {
            format!("{MOT_RECU} {numero}")
        } else {
            format!("{MOT_ONT} {numero}")
        }
    })
}
