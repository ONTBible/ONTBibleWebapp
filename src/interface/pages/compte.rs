use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::api::mon_compte;
use crate::domaine::compte::Fournisseur;
use crate::interface::design::{Bloc, Lien, PageDeLecture};
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

        <PageDeLecture
            // Le rappel ne redit pas le titre : ils se suivent à l'écran, et
            // « Votre compte / Votre compte » se lit comme un défaut de rendu.
            // Il nomme ce à quoi le compte sert, ce que le titre ne dit pas.
            rappel="Vos marques"
            titre="Votre compte"
            chapeau=Box::new(|| {
                view! {
                    <p class="text-encre-douce">
                        "Il sert à une seule chose : retrouver vos surlignages et vos notes "
                        "d'un appareil à l'autre."
                    </p>
                }
                    .into_any()
            })
        >
            <Bloc>
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
            </Bloc>

            <Bloc>
                <h2>"Ce que nous gardons"</h2>
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
            </Bloc>
        </PageDeLecture>
    }
}

/// L'état ouvert : on peut partir.
#[component]
fn Ouvert() -> impl IntoView {
    view! {
        <p class="mb-6">
            "Votre compte est ouvert. Vos surlignages suivent entre ce site et l'application."
        </p>
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
