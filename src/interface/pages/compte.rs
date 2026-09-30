use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::api::mon_compte;
use crate::domaine::compte::Fournisseur;
use crate::interface::design::{
    fournir_preferences, EnteteDeSection, Groupe, Lien, Ligne, PageDeLecture, PiedDeSection,
};
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

    let _ = &requete;

    view! {
        <Tete
            titre="Votre compte"
            description="Ouvrir un compte pour retrouver vos surlignages et vos notes \
                         d'un appareil à l'autre. La lecture, elle, n'en demande aucun."
            chemin="/fr/compte"
        />

        // ── C'est un écran de la webapp, pas une page du site ─────────────
        //
        // **Elle rendait `Entete` et `Bloc`**, c'est-à-dire la chrome de
        // l'édition : la navigation en capitales au-dessus de tout, et un
        // conteneur qui fait au moins la hauteur de l'écran.
        //
        // Conséquence, et c'était le plus gros trou du fork : toucher « Vous »
        // dans la barre d'onglets **sortait de la webapp**. Plus de barre,
        // plus de barre latérale, plus de thème — le lecteur qui lisait sur
        // parchemin recevait une nuit d'aubergine et l'en-tête d'un site
        // vitrine. Il n'y avait pas de retour visible vers là d'où il venait,
        // hors le bouton du navigateur.
        //
        // Ça ne se voyait dans aucun fichier : `COMPTE` déclare son chemin
        // dans la navigation, et la page le rend à sa façon quatre cents
        // lignes plus loin. `PageDeLecture` est ce qui porte la chrome et la
        // peau — la règle du §8 undecies, « les pages de la liseuse sont
        // exactement celles qui l'emploient » — donc s'en passer, c'est en
        // sortir.
        //
        // **`liste=true`**, comme l'app : `YouTab` est une `List` de sections,
        // pas un texte suivi. Le grand titre serré à gauche, et la liste
        // commence.
        //
        // Ce que l'ancien commentaire disait reste vrai et n'a plus d'objet :
        // deux `Bloc` faisaient deux trous d'un demi-écran. `PageDeLecture`
        // n'en pose qu'un.
        <PageDeLecture liste=true titre="Vous">
            <div>
                // L'erreur est descendue dans le pied de la carte du compte —
                // voir `Ferme`. Ici, détachée en tête, elle **remplaçait**
                // l'explication dans le regard du lecteur au lieu de s'y
                // ajouter.
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

            // **Ce que nous gardons** — trois paragraphes qui étaient une
            // section d'édition, avec son `h2` en corps 2xl. C'est un **pied
            // de carte** : une note attachée au réglage qui la motive, pas la
            // suite d'un texte. L'app range ses explications longues de la
            // même façon, en `footer:` de `Section`.
            <PiedDeSection>
                "Ce compte garde la " <b>"référence"</b> " du verset — son livre, son unité, "
                "son numéro —, la couleur choisie et la note écrite. "
                <b>"Jamais le texte du verset"</b> ", qui est déjà là. Un surlignage se "
                "rattache à un verset et non à une position dans le texte : c'est ce qui le "
                "garde juste quand une traduction est révisée. Tout s'efface depuis "
                <Lien href="/fr/confidentialite">"la page de confidentialité"</Lien>
                ", immédiatement et complètement."
            </PiedDeSection>

            <LaLecture />
            <LHabillage />
            <LeRegistre />
            <LeCorpus />
            <Credits />
        </PageDeLecture>
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
                                    "/fr/webapp/{}/{}",
                                    p.book_id,
                                    p.chapter_id,
                                )>{p.chapter_title.clone()}</Lien>
                                // Le verset n'est pas nommé : le site retient
                                // l'unité, pas la ligne — voir
                                // `api::retenir_la_position`. Annoncer un verset
                                // qu'on ne vise pas serait une précision fausse.
                                ". Reprendre là où vous en étiez ?"
                            </p>
                        }
                    })
            })}
        </Suspense>
        // `rel="external"` — et **c'est lui qui fait tout le travail**.
        //
        <MonProfil />
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

/// Ce que dit un retour de connexion qui a échoué.
///
/// **Une fonction libre et non une closure de la page** : c'est `Ferme` qui
/// l'affiche, dans le pied de sa carte, et non plus la page en tête. Le
/// message doit vivre là où il se rend — sinon il faut le faire descendre par
/// un prop à travers deux composants, et la première refonte le décroche.
///
/// Les codes viennent de la route de retour, qui les pose dans l'adresse.
fn erreur_de_connexion() -> Option<&'static str> {
    use_query_map()
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
}

/// L'état fermé : on propose les fournisseurs déclarés.
#[component]
fn Ferme() -> impl IntoView {
    let disponibles: Vec<Fournisseur> = Fournisseur::tous()
        .into_iter()
        .filter(|f| crate::interface::compte_public::disponible(*f))
        .collect();

    let partiels = disponibles.len() < Fournisseur::tous().len();

    view! {
        <EnteteDeSection sobre=true>"Compte"</EnteteDeSection>

        // **Des capsules pleines, en aplat de marque.** Elles étaient cerclées
        // et en capitales espacées — la forme que le design system réserve à
        // une *seconde* voie, et la voix de l'édition. Or c'est ici l'action
        // principale de l'écran, et l'app la peint en `brandInk` avec
        // `onBrandAccent` dessus : le même aplat que la carte de
        // prononciation, et pour la même raison — « ceci n'est pas du corpus,
        // c'est l'app qui te parle ».
        //
        // Pleine largeur et empilées, comme là-bas : trois libellés
        // « Continuer avec … » côte à côte ne tiennent sur aucun téléphone, et
        // une action principale ne se met pas en concurrence avec elle-même.
        <div class="flex flex-col gap-3">
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
                            class="presse survol survol--souleve flex items-center justify-center gap-3 rounded-full bg-marque-encre px-6 py-3.5 text-center font-titre text-sur-marque-accent no-underline"
                        >
                            // **La marque, en monochrome.** L'app portait trois
                            // provenances pour trois boutons — `apple.logo`,
                            // un `g.circle.fill` qui n'est pas le G de Google,
                            // et trois chevrons pour GitHub. Trois graisses,
                            // trois grilles.
                            //
                            // L'auteur a tranché pour Ionicons, monochrome, le
                            // 29 septembre 2026 : les trois prennent alors la
                            // teinte de leur bouton au lieu d'y poser la leur.
                            //
                            // **Google demande son mark en quatre couleurs**,
                            // et aucune variante monochrome officielle
                            // n'existe — mesuré côté app, cinq sources du kit
                            // essayées. C'est donc un écart connu, pas un
                            // oubli ; et il coûte moins ici, un site n'étant
                            // relu par personne. Ce qu'on garde est que les
                            // deux écrans se ressemblent.
                            <svg
                                aria-hidden="true"
                                viewBox="0 0 512 512"
                                fill="currentColor"
                                class="size-[1.15em] shrink-0"
                            >
                                <path d=crate::interface::design::symboles::trace(
                                    &format!("marque-{}", f.cle()),
                                    false,
                                ) />
                            </svg>
                            <span>"Continuer avec " {f.nom()}</span>
                        </a>
                    }
                })
                .collect_view()}
        </div>

        {partiels
            .then(|| {
                view! {
                    <p class="note-courte mt-3 px-4 text-[0.82em] text-encre-douce/80">
                        "D'autres façons de se connecter arrivent."
                    </p>
                }
            })}

        // **L'échec s'ajoute à l'explication, il ne la remplace pas.**
        //
        // C'est une leçon que l'app a payée cher : son message d'erreur
        // *remplaçait* le pied, donc une connexion ratée effaçait la seule
        // phrase qui dit que le compte est facultatif — et laissait croire
        // l'app cassée. Un examinateur de l'App Store l'a vue ainsi le
        // 19 août 2026.
        //
        // Le site avait la même forme sous un autre nom : l'erreur était un
        // bloc en tête de page, détaché, et la phrase qui rassure vivait
        // quatre écrans plus bas.
        <PiedDeSection>
            {move || {
                erreur_de_connexion()
                    .map(|message| {
                        view! {
                            // La braise de la gamme et non le rouge du système :
                            // un échec se lit sans crier.
                            <span
                                role="alert"
                                class="mb-2 block rounded-bloc bg-accentuation/12 px-3 py-2 text-accentuation"
                            >
                                {message}
                            </span>
                        }
                    })
            }}
            "La lecture, les surlignages et les notes fonctionnent entièrement sans compte. "
            "La connexion ne sert qu'à les retrouver sur un autre appareil."
        </PiedDeSection>
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
    let chemin = format!("/fr/webapp/{}/{}?v={}", v.livre_id, v.unite_id, v.verset);

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

/// Le profil du lecteur — ce qu'il choisit de dire de lui.
///
/// ## Il se lit d'abord, il s'écrit ensuite
///
/// Le premier état est un **affichage**, pas un formulaire. Un compte neuf n'a
/// rien à montrer et le dit en une phrase ; un compte qui a un profil le montre
/// tel qu'il est. On n'ouvre les champs que sur un geste — sinon la page de
/// compte s'ouvre sur un travail à faire, alors qu'elle sert d'abord à
/// retrouver ce qu'on a.
///
/// ## Le portrait vient de l'app, et le site n'y touche pas
///
/// Le téléverser demanderait un stockage que le projet n'a pas. On affiche
/// celui qui existe, on relit sa valeur avant d'écrire le reste pour ne pas
/// l'effacer, et à défaut on montre les initiales — ce que fait l'app quand un
/// lecteur n'a pas posé d'image.
#[component]
fn MonProfil() -> impl IntoView {
    let profil = Resource::new_blocking(|| (), |_| async { crate::api::mon_profil().await });
    let (edite, poser_edite) = signal(false);
    let enregistrer = ServerAction::<crate::api::EnregistrerMonProfil>::new();

    // Après un enregistrement, on relit : le backend garde le plus récent des
    // deux profils, donc ce qu'on vient d'envoyer n'est pas forcément ce qui
    // fait foi. Afficher notre propre envoi mentirait dans ce cas.
    Effect::new(move |_| {
        if enregistrer.version().get() > 0 {
            profil.refetch();
            poser_edite.set(false);
        }
    });

    view! {
        <div class="mt-14 border-t border-filet pt-10">
            <h2 class="text-2xl">"Vous"</h2>

            <Suspense fallback=|| {
                view! { <p class="text-encre-douce">"…"</p> }
            }>
                {move || Suspend::new(async move {
                    let p = profil.await.ok().flatten().unwrap_or_default();
                    let vide = p.est_vide();
                    let initiales = p.initiales();
                    let nom = p.nom_de_barre();
                    let arobase = p.arobase();
                    let bio = p.bio.clone();
                    let portrait = p.portrait.clone();
                    let (u, pr, n, b) = (
                        p.nom_dusage.clone(),
                        p.prenom.clone(),
                        p.nom.clone(),
                        p.bio.clone(),
                    );

                    view! {
                        <Show
                            when=move || !edite.get()
                            fallback=move || {
                                let (u, pr, n, b) = (
                                    u.clone(),
                                    pr.clone(),
                                    n.clone(),
                                    b.clone(),
                                );
                                view! {
                                    <ActionForm action=enregistrer>
                                        <Champ nom="nom_dusage" libelle="Nom d'usage" valeur=u />
                                        <Champ nom="prenom" libelle="Prénom" valeur=pr />
                                        <Champ nom="nom" libelle="Nom" valeur=n />
                                        <label class="mt-4 block">
                                            <span class="mb-2 block text-sm uppercase tracking-capitales text-encre-douce">
                                                "Bio"
                                            </span>
                                            <textarea
                                                name="bio"
                                                rows="3"
                                                class="w-full rounded-sm border border-filet bg-surface/40 px-4 py-3 text-base text-encre focus:border-accent focus:outline-none"
                                            >
                                                {b}
                                            </textarea>
                                        </label>
                                        <div class="mt-4 flex gap-3">
                                            <button
                                                type="submit"
                                                class="rounded-full border border-accent px-5 py-1 text-sm uppercase tracking-capitales text-accent"
                                            >
                                                "Enregistrer"
                                            </button>
                                            <button
                                                type="button"
                                                class="rounded-full border border-filet px-5 py-1 text-sm uppercase tracking-capitales text-encre-douce"
                                                on:click=move |_| poser_edite.set(false)
                                            >
                                                "Annuler"
                                            </button>
                                        </div>
                                    </ActionForm>
                                }
                            }
                        >
                            {
                                let (initiales, nom, arobase, bio, portrait) = (
                                    initiales.clone(),
                                    nom.clone(),
                                    arobase.clone(),
                                    bio.clone(),
                                    portrait.clone(),
                                );
                                view! {
                                    <div class="flex items-start gap-4">
                                        // Le portrait, ou les initiales. Jamais un
                                        // trou : un profil sans image est un cas
                                        // normal, pas une donnée manquante.
                                        {match portrait {
                                            Some(src) => {
                                                view! {
                                                    <img
                                                        src=src
                                                        alt=""
                                                        class="size-14 shrink-0 rounded-full object-cover"
                                                    />
                                                }
                                                    .into_any()
                                            }
                                            None => {
                                                view! {
                                                    <span
                                                        aria-hidden="true"
                                                        class="flex size-14 shrink-0 items-center justify-center rounded-full border border-filet text-lg text-encre-douce"
                                                    >
                                                        {initiales}
                                                    </span>
                                                }
                                                    .into_any()
                                            }
                                        }}
                                        <div class="min-w-0">
                                            <p class="mb-0 text-lg text-encre-vive">{nom}</p>
                                            {arobase
                                                .map(|a| {
                                                    view! {
                                                        <p class="mb-0 text-sm text-encre-douce">{a}</p>
                                                    }
                                                })}
                                            {(!bio.is_empty())
                                                .then(|| view! { <p class="mt-3 mb-0">{bio}</p> })}
                                            {vide
                                                .then(|| {
                                                    view! {
                                                        <p class="mt-1 mb-0 text-encre-douce">
                                                            "Vous n'avez rien écrit de vous. Ce que vous mettrez ici \
                                                             vous suivra dans l'application."
                                                        </p>
                                                    }
                                                })}
                                            <button
                                                type="button"
                                                class="mt-4 rounded-full border border-filet px-4 py-1 text-sm uppercase tracking-capitales text-encre-douce"
                                                on:click=move |_| poser_edite.set(true)
                                            >
                                                {if vide { "Se présenter" } else { "Modifier" }}
                                            </button>
                                        </div>
                                    </div>
                                }
                            }
                        </Show>
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Un champ de texte du formulaire de profil.
#[component]
fn Champ(nom: &'static str, libelle: &'static str, valeur: String) -> impl IntoView {
    view! {
        <label class="mt-4 block">
            <span class="mb-2 block text-sm uppercase tracking-capitales text-encre-douce">
                {libelle}
            </span>
            <input
                type="text"
                name=nom
                value=valeur
                autocomplete="off"
                class="w-full rounded-sm border border-filet bg-surface/40 px-4 py-3 text-base text-encre focus:border-accent focus:outline-none"
            />
        </label>
    }
}

/// La section « Lecture » de l'app.
///
/// Elle porte trois entrées là-bas — réglages, options de partage, surlignages
/// avec leur compteur. Le site en a deux, et l'absence est nommée plus bas.
#[component]
fn LaLecture() -> impl IntoView {
    view! {
        <EnteteDeSection sobre=true>"Lecture"</EnteteDeSection>
        <Groupe>
            // **Les réglages sont atteignables d'ici**, et pas seulement
            // depuis le bouton « aA » d'un chapitre. C'est ce que l'app fait,
            // et la raison tient : on veut parfois régler sa typographie
            // **avant** d'ouvrir un texte, et le bouton « aA » n'existe que
            // dans un texte ouvert.
            //
            // Le lien mène à la Bible, où la feuille s'ouvre : le site n'a pas
            // d'écran de réglages à lui, et lui en fabriquer un ferait une
            // seconde copie des mêmes bascules.
            <Ligne
                chemin=Some("/fr/compte/lecture".to_string())
                titre=Box::new(|| view! { "Réglages de lecture" }.into_any())
                sous_titre=Box::new(|| {
                    view! { "Thème, fonte, taille, niveaux du texte" }.into_any()
                })
            />
            <Ligne
                chemin=Some("/fr/compte#versets".to_string())
                titre=Box::new(|| view! { "Surlignages" }.into_any())
                sous_titre=Box::new(|| view! { "Ce que vous avez marqué" }.into_any())
            />
        </Groupe>
        // **« Options de partage » n'y est pas**, et ce n'est pas un oubli :
        // l'écran qu'elle règle chez l'app est l'action *Image*, qui rend un
        // carré de 1080 px. Le site ne la porte pas encore — une entrée qui
        // règlerait une action inexistante serait « allumée vers rien ».
        <PiedDeSection>
            "Les réglages s'ouvrent aussi par le bouton « aA » d'un chapitre, "
            "au moment où l'on décide d'éteindre une glose."
        </PiedDeSection>
    }
}

/// Le registre — **le réglage qui décide de ce qu'on lit**, pas de son état.
///
/// ## Pourquoi il est ici et non dans les réglages de lecture
///
/// L'app l'en a sorti délibérément, et son commentaire dit pourquoi : il était
/// rangé « entre la disposition des versets et la taille du texte », c'est-à-
/// dire **avec la typographie**. Or il ne change pas la façon dont le texte se
/// présente : il change **ce que les livres sont appelés**, donc le corpus tel
/// que le lecteur le rencontre.
///
/// Le site l'avait au même mauvais endroit — dans la feuille « aA », entre les
/// gloses et le corps. Il y reste atteignable en lecture, où l'on bascule d'un
/// geste ; il a désormais **sa carte** ici, où l'on décide.
///
/// ## Et le texte qui l'accompagne n'est pas une notice
///
/// C'est l'explication la plus longue de l'app, et elle a sa raison : ce
/// réglage est **une béquille allumée par défaut**, et l'éteindre fait
/// apparaître des mots que le lecteur n'a peut-être jamais lus. Sans le pied,
/// il le remet sans comprendre ce qu'il vient de voir.
#[component]
fn LeRegistre() -> impl IntoView {
    // **`fournir_preferences` et non `preferences`** : il faut pouvoir
    // **écrire**, et `preferences()` ne rend qu'un signal de lecture — son
    // repli muet rendrait d'ailleurs un signal constant, donc un interrupteur
    // qui ne commute rien. Elle est idempotente : appelée ici, elle retrouve
    // celui qu'`App` a posé pour tout le site, et la bascule est donc la même
    // que celle de la feuille « aA ».
    let prefs = fournir_preferences();
    let recu = Signal::derive(move || prefs.get().francais);

    view! {
        <EnteteDeSection sobre=true>"Le corpus"</EnteteDeSection>
        <Groupe>
            <li>
                <label class="group flex cursor-pointer items-center justify-between gap-6 px-4 py-3.5">
                    <span class="text-encre">"Le français reçu"</span>
                    <input
                        type="checkbox"
                        class="peer sr-only"
                        prop:checked=move || recu.get()
                        on:change=move |_| prefs.update(|p| p.francais = !p.francais)
                    />
                    <span class="relative h-6 w-11 shrink-0 rounded-full bg-encre-douce/25 transition-colors peer-checked:bg-accent/40 peer-focus-visible:outline peer-focus-visible:outline-2 peer-focus-visible:outline-accent motion-reduce:transition-none">
                        <span class="absolute top-1/2 start-0.5 size-5 -translate-y-1/2 rounded-full bg-encre-douce transition-transform group-has-[:checked]:translate-x-5 group-has-[:checked]:bg-accent motion-reduce:transition-none"></span>
                    </span>
                </label>
            </li>
        </Groupe>
        <PiedDeSection>
            "Allumé, les livres portent le nom qu'on leur connaît — « Apocalypse », "
            "« la Loi », « Chapitre 7 ». Éteint, ils portent ce que leur nom hébreu veut "
            "dire : « le machazeh de Yohanan », « la Fondation », « Parashah 7 »."
            <br /><br />
            "L'écart n'est pas une nuance de traduction. La "
            <i>"torah"</i> " est l'instruction qui vise ; le grec l'a rendue par "
            <i>"nomos"</i> ", le code qui contraint, et le français en a hérité « la Loi »."
            <br /><br />
            "Ce réglage est une béquille, et il est allumé pour qu'on puisse marcher avant "
            "de savoir. En l'éteignant, des mots apparaissent que vous n'avez peut-être "
            "jamais lus — " <i>"parashah"</i> ", la division que le scribe hébreu traçait en "
            "laissant un blanc, mille ans avant qu'on numérote des chapitres."
        </PiedDeSection>
    }
}

/// Le choix de l'habillage — **l'application, ou l'édition**.
///
/// ## Pourquoi il est ici et pas dans la feuille « aA »
///
/// La feuille porte ce qui change **le texte** : ses niveaux, son corps, sa
/// fonte, sa peau. L'habillage ne touche pas au texte — les deux rendent la
/// même mesure, la même fonte, la même composition, et c'est ce fait mesuré qui
/// a permis de garder les deux sans les faire diverger.
///
/// Il change ce qu'il y a **autour**, c'est-à-dire la façon dont on circule. Et
/// une façon de circuler se décide une fois, là où l'on décide — pas au milieu
/// d'un chapitre. C'est la même raison qui a sorti le registre des réglages de
/// lecture : il y était rangé avec la typographie alors qu'il change ce que les
/// livres sont appelés.
///
/// ## Deux lignes nommées, et non un interrupteur
///
/// Un interrupteur dirait « habillage de l'application » avec un rond à
/// basculer, et le lecteur ne saurait pas ce que l'autre état lui donne. Deux
/// lignes disent chacune **ce qu'on y voit** — les onglets en bas, ou l'en-tête
/// du site — et le choix se fait sur la conséquence, pas sur le nom.
#[component]
fn LHabillage() -> impl IntoView {
    use crate::domaine::lecture::Habillage;

    // Comme le registre : il faut pouvoir **écrire**, donc `fournir_preferences`
    // et non `preferences`, dont le repli muet rendrait un signal constant.
    let prefs = fournir_preferences();

    view! {
        <EnteteDeSection sobre=true>"L'habillage"</EnteteDeSection>
        <Groupe>
            {Habillage::TOUS
                .into_iter()
                .map(|habillage| {
                    let choisi = Signal::derive(move || prefs.get().habillage == habillage);
                    view! {
                        <li>
                            <label class="presse--ligne survol flex cursor-pointer items-start justify-between gap-5 px-4 py-3.5">
                                <span class="min-w-0">
                                    <span class="block text-encre">{habillage.libelle()}</span>
                                    <span class="mt-1 block text-sm text-encre-douce text-pretty">
                                        {habillage.note()}
                                    </span>
                                </span>
                                <input
                                    type="radio"
                                    name="habillage"
                                    class="peer sr-only"
                                    prop:checked=move || choisi.get()
                                    on:change=move |_| prefs.update(|p| p.habillage = habillage)
                                />
                                // Le témoin est **plein quand il est choisi**, et
                                // c'est la seule marque : deux ronds cerclés dont
                                // l'un porte un point se distinguent mal quand on
                                // voit mal.
                                <span
                                    aria-hidden="true"
                                    class="mt-1 flex size-5 shrink-0 items-center justify-center rounded-full border border-encre-douce/50 peer-checked:border-accent peer-focus-visible:outline peer-focus-visible:outline-2 peer-focus-visible:outline-accent"
                                >
                                    <span
                                        class="size-2.5 rounded-full bg-accent transition-opacity motion-reduce:transition-none"
                                        class=("opacity-0", move || !choisi.get())
                                    ></span>
                                </span>
                            </label>
                        </li>
                    }
                })
                .collect_view()}
        </Groupe>
        <PiedDeSection>
            "Le texte ne bouge pas d'un habillage à l'autre\u{a0}: même largeur de "
            "colonne, même fonte, même composition. Ce qui change est ce qu'il y a "
            "autour — la façon d'aller d'un livre à l'autre."
            <br /><br />
            "L'application est ce que vous avez sur le téléphone, et c'est le défaut. "
            "L'édition est la liseuse telle qu'elle était\u{a0}: l'en-tête du site, son "
            "pied de page, et rien autour du texte."
            <br /><br />
            "Votre choix est retenu dans ce navigateur, avec les autres réglages de "
            "lecture. Il ne demande aucun compte."
        </PiedDeSection>
    }
}

/// L'état du chantier, comme `YouTab` le donne.
///
/// ## Pourquoi il est ici et pas seulement sur l'accueil
///
/// L'accueil l'annonce à qui **découvre** le projet — c'est un argument, et il
/// y est le cinquième bloc. Ici c'est autre chose : le lecteur qui revient veut
/// savoir *ce qui a bougé depuis la dernière fois*. L'app le range de même,
/// sous « Le Corpus », dans l'onglet où l'on va voir ses affaires.
///
/// Les nombres viennent du `manifest.json` du pipeline, figés par `build.rs` —
/// **jamais recopiés**. Un site qui annonce trois livres quand le vault en a
/// cinq ment sans que personne ne le remarque.
///
/// ## En lignes, pas en grille
///
/// Le site a déjà `Chiffres`, une grille de quatre cellules, et elle est juste
/// là où elle est : sur l'accueil, elle **frappe**. Dans une liste de réglages,
/// une grille rompt la colonne — l'app emploie des `LabeledContent`, c'est-à-
/// dire un intitulé à gauche et sa valeur à droite. C'est ce que `Ligne` fait
/// déjà, et c'est pour ça qu'elle a un prop `valeur`.
#[component]
fn LeCorpus() -> impl IntoView {
    view! {
        <Groupe>
            <Ligne
                titre=Box::new(|| view! { "Livres rédigés" }.into_any())
                valeur=Box::new(|| {
                    view! {
                        <span class="chiffres-tableau whitespace-nowrap">
                            {env!("CORPUS_LIVRES_ECRITS")} " / " {env!("CORPUS_LIVRES")}
                        </span>
                    }
                        .into_any()
                })
            />
            <Ligne
                titre=Box::new(|| view! { "Unités" }.into_any())
                valeur=Box::new(|| {
                    view! { <span class="chiffres-tableau">{env!("CORPUS_UNITES")}</span> }
                        .into_any()
                })
            />
            <Ligne
                titre=Box::new(|| view! { "Versets" }.into_any())
                valeur=Box::new(|| {
                    view! { <span class="chiffres-tableau">{env!("CORPUS_VERSETS")}</span> }
                        .into_any()
                })
            />
            <Ligne
                titre=Box::new(|| view! { "Entrées de lexique" }.into_any())
                valeur=Box::new(|| {
                    view! { <span class="chiffres-tableau">{env!("CORPUS_LEXIQUE")}</span> }
                        .into_any()
                })
            />
        </Groupe>
        <PiedDeSection>
            "La Bible ONT est une restitution en cours. Le corpus s'étend à mesure que "
            "les unités sont verrouillées."
        </PiedDeSection>
    }
}

/// Les crédits — et ce n'est pas de la politesse.
///
/// ## L'OFL l'exige
///
/// Les six familles de lecture et les deux fontes hébraïques sont sous SIL
/// Open Font License. Elle autorise la redistribution **à condition que la
/// licence parte avec** — `scripts/fontes.sh` copie donc les fichiers de
/// licence dans `public/fontes/`, et son en-tête dit pourquoi SBL Hebrew et
/// Taamey Frank CLM n'y entrent jamais.
///
/// Un fichier posé à côté d'une fonte satisfait la lettre. Le nommer dans
/// l'interface satisfait ce que la lettre protège : **on sait qui a dessiné ce
/// qu'on lit.** L'app le fait, sous « Crédits », et le site ne le faisait nulle
/// part.
///
/// ## Et la traduction porte un nom
///
/// « Gloire Bikouta », jamais « Sha'eliel » : c'est le nom interne au vault, et
/// il n'en sort pas.
#[component]
fn Credits() -> impl IntoView {
    view! {
        <EnteteDeSection sobre=true>"Crédits"</EnteteDeSection>
        <Groupe>
            <Ligne
                titre=Box::new(|| view! { "Traduction" }.into_any())
                valeur=Box::new(|| view! { "Gloire Bikouta" }.into_any())
            />
            <Ligne
                titre=Box::new(|| view! { "Corps du texte" }.into_any())
                valeur=Box::new(|| view! { "Literata — OFL" }.into_any())
            />
            <Ligne
                titre=Box::new(|| view! { "Titres" }.into_any())
                valeur=Box::new(|| view! { "Jost — OFL" }.into_any())
            />
            <Ligne
                titre=Box::new(|| view! { "Hébreu" }.into_any())
                valeur=Box::new(|| view! { "Ezra SIL — OFL" }.into_any())
            />
        </Groupe>
    }
}
