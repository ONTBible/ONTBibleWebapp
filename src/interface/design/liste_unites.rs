use leptos::prelude::*;

use crate::api::UniteDto;
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
                                <span class="text-accent">{unite.titre}</span>

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
