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
        // `rel="external"` — et **c'est lui qui fait tout le travail**.
        //
        <MesVersets />

        // Une ancre ordinaire ne suffisait pas : le routeur de Leptos intercepte
        // *tous* les clics sur les liens internes, cherche le chemin dans ses
        // pages, ne le trouve pas — ces routes-ci sont posées avant lui, dans
        // `main.rs` — et rend sa page d'erreur.
        //
        // Le symptôme était exactement celui que Gloire a décrit : « ça ne
        // s'affiche que quand je recharge ». Au rechargement, le navigateur fait
        // une vraie requête, le serveur répond, tout marche. Au clic, jamais.
        //
        // `location/mod.rs:346` chez `leptos_router` : le routeur rend la main
        // si l'ancre porte `download` ou un `rel` contenant `external`.
        <a
            rel="external"
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
                            // Même raison qu'à la déconnexion : ces routes sont
                            // servies avant le routeur, qui rendrait son 404.
                            rel="external"
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

/// Tous les versets que le lecteur a marqués, rangés par livre.
///
/// ## Elle n'existe que pour un compte ouvert
///
/// Sans lui, il n'y a rien à montrer — et une liste vide accompagnée d'un
/// « connectez-vous pour voir » serait un cadre vide qui promet quelque chose.
/// Le composant n'est donc rendu que par `Ouvert`.
///
/// ## L'ordre du corpus, et le texte recomposé
///
/// La liste suit l'ordre des livres, pas celui des marques : on cherche « ce
/// que j'ai marqué dans *Bereshit* », pas « ce que j'ai marqué mardi ». Et le
/// texte de chaque verset est **recomposé depuis le corpus** — il n'est stocké
/// nulle part, ce qui le garde juste quand une traduction est révisée.
#[component]
fn MesVersets() -> impl IntoView {
    let versets = Resource::new_blocking(|| (), |_| async { crate::api::mes_versets().await });
    // `None` = toutes. Le filtre vit ici et non dans l'adresse : c'est un
    // regard qu'on porte sur sa propre liste, pas un endroit qu'on partage.
    let (filtre, poser_filtre) = signal(None::<String>);

    view! {
        <div class="mt-14 border-t border-filet pt-10">
            <h2 class="text-2xl">"Vos versets"</h2>

            <Suspense fallback=|| {
                view! { <p class="text-encre-douce">"…"</p> }
            }>
                {move || Suspend::new(async move {
                    let toute = versets.await.unwrap_or_default();

                    // Les couleurs employées, avec leur compte, dans l'ordre de
                    // la palette et non d'apparition : un filtre dont les
                    // pastilles changent de place d'une visite à l'autre se
                    // relit à chaque fois.
                    let couleurs: Vec<(&'static str, &'static str, &'static str, usize)> =
                        crate::domaine::surlignage::Couleur::toutes()
                            .into_iter()
                            .filter_map(|c| {
                                let n = toute.iter().filter(|v| v.couleur == c.cle()).count();
                                (n > 0).then_some((c.cle(), c.nom(), c.teinte(), n))
                            })
                            .collect();

                    let total_tous: usize = couleurs.iter().map(|(_, _, _, n)| n).sum();
                    let choisie = filtre.get();
                    let liste: Vec<_> = match &choisie {
                        Some(cle) => toute.into_iter().filter(|v| v.couleur == *cle).collect(),
                        None => toute,
                    };
                    if liste.is_empty() && choisie.is_none() {
                        return view! {
                            <p class="text-encre-douce">
                                "Vous n'avez encore rien surligné. Ouvrez un chapitre, "
                                "touchez un verset, et choisissez une couleur."
                            </p>
                        }
                            .into_any();
                    }

                    // Groupé par livre, dans l'ordre où la fonction serveur les
                    // rend — elle a déjà trié selon le sommaire du corpus, donc
                    // il n'y a qu'à couper aux changements de livre.
                    let mut groupes: Vec<(
                        String,
                        String,
                        String,
                        Vec<crate::api::VersetSurligne>,
                    )> = Vec::new();
                    for v in liste {
                        match groupes.last_mut() {
                            Some((id, _, _, versets)) if *id == v.livre_id => versets.push(v),
                            _ => groupes.push((
                                v.livre_id.clone(),
                                v.livre_titre.clone(),
                                v.livre_francais.clone(),
                                vec![v],
                            )),
                        }
                    }

                    let combien: usize = groupes.iter().map(|(_, _, _, v)| v.len()).sum();

                    view! {
                        // Le filtre ne paraît qu'à partir de deux couleurs : à
                        // une seule il ne trierait rien, et proposerait un geste
                        // sans effet. C'est la condition de l'app, à l'identique.
                        {(couleurs.len() > 1)
                            .then(|| {
                                view! {
                                    <div class="mb-8 flex flex-wrap gap-2" role="group" aria-label="Filtrer par couleur">
                                        <FiltreCouleur
                                            cle=None
                                            nom="Toutes".to_string()
                                            teinte=None
                                            combien=total_tous
                                            choisie=choisie.clone()
                                            poser=poser_filtre
                                        />
                                        {couleurs
                                            .iter()
                                            .map(|(cle, nom, teinte, n)| {
                                                view! {
                                                    <FiltreCouleur
                                                        cle=Some(cle.to_string())
                                                        nom=nom.to_string()
                                                        teinte=Some(teinte.to_string())
                                                        combien=*n
                                                        choisie=choisie.clone()
                                                        poser=poser_filtre
                                                    />
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                }
                            })}

                        <p class="chiffres-tableau mb-8 text-sm text-encre-douce">
                            {combien} " verset" {(combien > 1).then_some("s")} " marqué"
                            {(combien > 1).then_some("s")}
                        </p>

                        {groupes
                            .into_iter()
                            .map(|(_, titre, francais, versets)| {
                                // Le nom français **sous** le titre du corpus, et
                                // jamais à sa place : c'est la règle du §8 octies —
                                // « on ne remplace pas le nom, on le traduit à
                                // côté » —, et c'est ce que fait l'en-tête de
                                // l'app. Omis quand il redirait le titre.
                                let second = (!francais.is_empty() && francais != titre)
                                    .then_some(francais);
                                view! {
                                    <section class="mb-10">
                                        <h3 class="mb-4">
                                            <span class="block text-sm uppercase tracking-capitales text-accent">
                                                {titre}
                                            </span>
                                            {second
                                                .map(|f| {
                                                    view! {
                                                        <span class="block text-sm text-encre-douce">
                                                            {f}
                                                        </span>
                                                    }
                                                })}
                                        </h3>
                                        <ul class="m-0 list-none p-0">
                                            {versets
                                                .into_iter()
                                                .map(|v| view! { <UnVerset v /> })
                                                .collect_view()}
                                        </ul>
                                    </section>
                                }
                            })
                            .collect_view()}
                    }
                        .into_any()
                })}
            </Suspense>
        </div>
    }
}

/// Un verset marqué, avec sa teinte et sa note.
///
/// La couleur se pose sur un **filet de gauche** et non sur le fond, comme dans
/// la liseuse : ici les entrées se suivent en liste, et cinq fonds colorés à la
/// file feraient une bande dessinée. Le filet dit la même chose en pesant moins.
#[component]
fn UnVerset(v: crate::api::VersetSurligne) -> impl IntoView {
    let teinte = crate::domaine::surlignage::Couleur::depuis_cle(&v.couleur)
        .map(|c| c.teinte())
        .unwrap_or("#E8C973");
    let chemin = format!("/fr/lire/{}/{}?v={}", v.livre_id, v.unite_id, v.verset);

    view! {
        <li class="mb-6 border-s-2 ps-4" style=format!("border-color: {teinte}")>
            // Le renvoi à gauche, la date à droite — la disposition de
            // `LigneDeSurlignage` dans l'app. La date dit quand on a marqué, ce
            // qui est la seule chose que la référence ne dit pas.
            <div class="flex items-baseline justify-between gap-4">
                <Lien href=chemin>
                    <span class="chiffres-tableau text-sm text-encre-douce">
                        {v.unite_titre} ":" {v.verset}
                    </span>
                </Lien>
                <span class="chiffres-tableau shrink-0 text-sm text-encre-douce/70">
                    {v.quand_affiche}
                </span>
            </div>
            // Le texte passe par `composer` : il vient du corpus, donc il porte
            // les espaces ordinaires devant les ponctuations doubles que le
            // français veut insécables. C'est la règle du §8 bis, et elle vaut
            // pour toute chaîne du corpus posée dans une page.
            <p class="mt-1 mb-0">
                {crate::interface::design::verset::composer(&v.texte)}
            </p>
            {v
                .note
                .map(|note| {
                    view! {
                        <p class="mt-2 mb-0 flex gap-2 text-sm text-encre-douce">
                            <span aria-hidden="true" class="text-accent">"❞"</span>
                            <span>{note}</span>
                        </p>
                    }
                })}
        </li>
    }
}

/// Une pastille du filtre de couleur.
///
/// Elle porte son **compte**, comme dans l'app : sans lui, on choisit une
/// couleur pour découvrir qu'elle ne garde rien, et l'on recommence.
#[component]
fn FiltreCouleur(
    cle: Option<String>,
    nom: String,
    teinte: Option<String>,
    combien: usize,
    choisie: Option<String>,
    poser: WriteSignal<Option<String>>,
) -> impl IntoView {
    let active = choisie == cle;
    let a_poser = cle.clone();
    view! {
        <button
            type="button"
            aria-pressed=active.to_string()
            class="flex items-center gap-2 rounded-full border px-3 py-1 text-sm transition-colors motion-reduce:transition-none"
            class=("border-accent", active)
            class=("text-encre-vive", active)
            class=("border-filet", !active)
            class=("text-encre-douce", !active)
            on:click=move |_| poser.set(a_poser.clone())
        >
            {teinte
                .map(|t| {
                    view! {
                        <span
                            aria-hidden="true"
                            class="size-2.5 shrink-0 rounded-full"
                            style=format!("background-color: {t}")
                        />
                    }
                })}
            <span>{nom}</span>
            <span class="chiffres-tableau text-encre-douce/70">{combien}</span>
        </button>
    }
}
