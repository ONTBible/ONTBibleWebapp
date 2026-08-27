use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::api::mon_compte;
use crate::domaine::compte::Fournisseur;
use crate::interface::design::{Bloc, Entete, Lien};
use crate::interface::tete::Tete;

/// `/fr/compte` — ouvrir un compte, ou le fermer.
///
/// ## Ce que le compte apporte, et ce qu'il ne change pas
///
/// Il **ajoute** les surlignages et les notes, et les fait suivre entre le site
/// et l'app. Il ne change rien à la lecture : le corpus, le lexique et le verset
/// du jour sont là sans lui, et le resteront.
///
/// Ce n'est pas une politesse — c'est la conséquence de ce que ces données
/// sont. Le backend le dit en tête de son module de synchronisation :
/// « les surlignages et les notes d'un lecteur de Bible, rattachés à une
/// identité, révèlent des convictions religieuses : article 9 du RGPD ». Un site
/// qui exigerait un compte pour lire ferait de cette lecture une donnée.
#[component]
pub fn Compte() -> impl IntoView {
    let requete = use_query_map();
    let etat = Resource::new_blocking(|| (), |_| async { mon_compte().await });

    let erreur = move || {
        requete
            .read()
            .get("erreur")
            .map(|code| match code.as_str() {
                "refus" => "La connexion a été interrompue. Rien n'a été enregistré.",
                "expire" => "La demande a expiré. Recommencez, ça ne prend qu'un instant.",
                "indisponible" => "Le service de comptes ne répond pas. Réessayez dans un moment.",
                "fournisseur" | "reponse" | "interne" => {
                    "Quelque chose s'est mal passé de notre côté. Réessayez."
                }
                _ => "La connexion n'a pas abouti.",
            })
    };

    view! {
        <Tete
            titre="Votre compte"
            description="Ouvrir un compte pour retrouver vos surlignages et vos notes \
                         d'un appareil à l'autre. La lecture, elle, n'en demande aucun."
            chemin="/fr/compte"
        />

        // ── Un seul bloc, et c'est la correction ──────────────────────────
        //
        // La page en portait **deux**, chacun sur `PageDeLecture` : or un `Bloc`
        // fait au moins la hauteur de l'écran, contenu centré. C'est juste pour
        // une page éditoriale, où chaque section *est* un écran — l'accueil,
        // « Le pourquoi ». Ici il y avait six paragraphes répartis sur deux
        // écrans entiers, donc deux trous d'un demi-écran chacun.
        //
        // La règle qui en sort : `Bloc` sert à ce qui se lit **section par
        // section**. Une page fonctionnelle — un compte, un formulaire — suit
        // `PageLegale`, qui n'en prend qu'un seul.
        <Entete />
        <Bloc>
            <a
                href="/fr"
                class="text-sm uppercase tracking-capitales text-encre-douce no-underline hover:text-encre"
            >
                "← Retour"
            </a>

            <h1 class="mt-6">"Votre compte"</h1>
            <p class="mb-10 text-encre-douce">
                "Il sert à une seule chose : retrouver vos surlignages et vos notes "
                "d'un appareil à l'autre."
            </p>

            <div>
                {move || {
                    erreur()
                        .map(|message| {
                            view! {
                                <p
                                    role="alert"
                                    class="mb-8 rounded-carte border border-accentuation/40 bg-surface px-5 py-4 text-encre"
                                >
                                    {message}
                                </p>
                            }
                        })
                }}

                <Suspense fallback=|| {
                    view! { <p class="text-encre-douce">"…"</p> }
                }>
                    {move || Suspend::new(async move {
                        let connecte = etat.await.map(|e| e.connecte).unwrap_or(false);
                        if connecte {
                            view! { <Ouvert /> }.into_any()
                        } else {
                            view! { <Ferme /> }.into_any()
                        }
                    })}
                </Suspense>
            </div>

            <div class="mt-16 border-t border-filet pt-10">
                <h2 class="text-2xl">"Ce que nous gardons"</h2>
                <p>
                    "La " <b>"référence"</b> " du verset — son livre, son unité, son numéro — "
                    "la couleur que vous avez choisie, et la note que vous y avez écrite. "
                    <b>"Jamais le texte du verset"</b> ", qui est déjà dans le site et dans l'app."
                </p>
                <p>
                    "Un surlignage se rattache à un " <b>"verset"</b> " et non à une position "
                    "dans le texte. C'est ce qui le garde juste quand une traduction est "
                    "révisée : les caractères bougent, le numéro du verset ne bouge pas."
                </p>
                <p>
                    "Vous pouvez tout effacer, à tout moment, depuis "
                    <Lien href="/fr/confidentialite">"la page de confidentialité"</Lien>
                    " — l'effacement est immédiat et il est complet."
                </p>
            </div>
        </Bloc>
    }
}

/// L'état ouvert : on peut reprendre sa lecture, ou partir.
#[component]
fn Ouvert() -> impl IntoView {
    // Où l'on en était. C'est le seul endroit du site qui montre la position :
    // elle n'a de sens qu'ici, où l'on ne lit pas encore.
    let position = Resource::new_blocking(|| (), |_| async { crate::api::ma_position().await });

    view! {
        <p class="mb-6">
            "Votre compte est ouvert. Vos surlignages suivent entre ce site et l'application."
        </p>

        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                position
                    .await
                    .ok()
                    .flatten()
                    .map(|p| {
                        view! {
                            <p class="mb-6">
                                "Vous lisiez "
                                <Lien href=format!(
                                    "/fr/lire/{}/{}",
                                    p.book_id,
                                    p.chapter_id,
                                )>{p.chapter_title.clone()}</Lien>
                                // Le verset n'est pas nommé : le site retient
                                // l'unité, pas la ligne — voir
                                // `api::retenir_la_position`. Annoncer un verset
                                // qu'on ne vise pas serait une précision fausse.
                                ". Reprendre là où vous en étiez ?"
                            </p>
                        }
                    })
            })}
        </Suspense>
        // Une ancre ordinaire et non un bouton du routeur : la déconnexion est
        // une route du serveur, qui écrit un en-tête. Le routeur la traiterait
        // comme une page à charger, et le cookie ne serait jamais effacé.
        <a
            href="/fr/compte/partir"
            class="inline-block rounded-full border border-or/50 px-6 py-3 text-sm uppercase tracking-capitales text-accent no-underline transition-colors hover:border-or hover:bg-aubergine/40"
        >
            "Se déconnecter"
        </a>
    }
}

/// L'état fermé : on propose les fournisseurs déclarés.
#[component]
fn Ferme() -> impl IntoView {
    let disponibles: Vec<Fournisseur> = Fournisseur::tous()
        .into_iter()
        .filter(|f| crate::interface::compte_public::disponible(*f))
        .collect();

    view! {
        <p class="mb-6">
            "Choisissez par où vous connecter. Nous ne recevons que de quoi vous reconnaître — "
            "ni votre nom, ni vos contacts."
        </p>

        <div class="flex flex-col gap-3 sm:flex-row sm:flex-wrap">
            {disponibles
                .iter()
                .map(|f| {
                    let f = *f;
                    view! {
                        <a
                            href=format!("/fr/compte/aller/{}", f.cle())
                            class="inline-block rounded-full border border-or/50 px-6 py-3 text-center text-sm uppercase tracking-capitales text-accent no-underline transition-colors hover:border-or hover:bg-aubergine/40"
                        >
                            "Continuer avec " {f.nom()}
                        </a>
                    }
                })
                .collect_view()}
        </div>

        {(disponibles.len() < Fournisseur::tous().len())
            .then(|| {
                view! {
                    <p class="mt-6 text-sm text-encre-douce">
                        "D'autres façons de se connecter arrivent."
                    </p>
                }
            })}
    }
}
