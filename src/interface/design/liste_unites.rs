use leptos::prelude::*;

use crate::api::UniteDto;
use crate::interface::design::reglages_de_lecture::preferences;
use crate::interface::design::MentionBrouillon;

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
                                <span class="text-accent">{libelle(&unite)}</span>

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
/// Une introduction — rang zéro — garde son titre : elle n'a pas de rang à
/// afficher, et « Chapitre 0 » ne voudrait rien dire.
fn libelle(unite: &UniteDto) -> impl IntoView {
    if unite.numero == 0 {
        let titre = unite.titre.clone();
        return Signal::stored(titre);
    }
    let n = unite.numero;
    let prefs = preferences();
    Signal::derive(move || {
        let mot = if prefs.get().francais {
            "Chapitre"
        } else {
            "Parashah"
        };
        format!("{mot} {n}")
    })
}
