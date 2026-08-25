use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_query_map};

use crate::api::passage;
use crate::domaine::selection;
use crate::interface::design::{
    fournir_preferences, nom_d_unite, Blocs, MentionBrouillon, PageDeLecture, ReglagesDeLecture,
};
use crate::interface::tete::Tete;

/// `/fr/lire/{livre}/{unité}` — un passage.
///
/// ## La route la plus sensible du site
///
/// C'est celle qu'ouvrent les liens partagés depuis l'app, et celle que
/// l'association d'app réserve à iOS (voir [`crate::interface::association`]).
/// Elle est aujourd'hui servie par la Lambda de l'API ; le jour où le site
/// prend la racine du domaine, c'est cette page qui répond à sa place. Si elle
/// tombe, les liens déjà partagés tombent avec elle — et rien ne le signale.
///
/// ## Ce qu'elle montre, et ce que l'ancienne page ne montrait pas
///
/// La page de repli du backend affichait le renvoi et proposait l'app : le
/// corpus vivait dans le bundle de l'app, pas dans la Lambda, et l'y dupliquer
/// aurait créé une seconde source de vérité. Ici le corpus est embarqué à la
/// compilation depuis `dist/` — la **même** source que l'app — donc la page
/// peut enfin montrer le texte.
#[component]
pub fn Passage() -> impl IntoView {
    let parametres = use_params_map();
    let requete = use_query_map();

    let cle = move || {
        let p = parametres.read();
        (
            p.get("livre").unwrap_or_default(),
            p.get("unite").unwrap_or_default(),
        )
    };

    // Le `?v=` d'un lien partagé. Il ne participe pas au chargement — c'est le
    // même chapitre avec ou sans lui — donc il est lu à part de la ressource :
    // changer la sélection ne doit pas relancer une requête.
    let designes = move || {
        requete
            .read()
            .get("v")
            .map(|v| selection::versets(&v))
            .unwrap_or_default()
    };

    let contenu =
        Resource::new_blocking(
            cle,
            |(livre, unite)| async move { passage(livre, unite).await },
        );

    // Les réglages sont installés **ici** et non dans un composant plus bas :
    // c'est la page qui décide qu'on lit du corpus, donc c'est elle qui ouvre
    // la possibilité d'en éteindre les niveaux. Le lexique n'en a pas — une
    // fiche est un commentaire, elle n'a pas d'appareil critique à retirer.
    let preferences = fournir_preferences();

    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                let en_avant = designes();
                match contenu.await {
                    Ok(Some(p)) => {
                        let chapitre = p.chapitre;
                        let brouillon = chapitre.statut.est_provisoire();
                        let reference = chapitre
                            .sous_titre
                            .as_ref()
                            .and_then(|s| s.reference.clone());

                        // La description d'aperçu est le **texte** des versets
                        // désignés quand le lien en désigne : c'est ce que la
                        // personne a partagé, et c'est donc ce qu'une messagerie
                        // doit montrer. À défaut, le renvoi et le livre.
                        let description = apercu(&chapitre, &en_avant, &p.livre_titre);

                        let renvoi = reference.clone();
                        let livre_titre = p.livre_titre.clone();
                        let chapeau = Box::new(move || {
                            view! {
                                {renvoi
                                    .map(|r| {
                                        view! {
                                            <p class="chiffres-tableau mb-4 text-encre-douce">
                                                {livre_titre} " " {r}
                                            </p>
                                        }
                                    })}
                                {brouillon.then(|| view! { <MentionBrouillon /> })}
                            }
                                .into_any()
                        });

                        view! {
                            <Tete
                                titre=chapitre.titre.clone()
                                description=description
                                chemin=format!("/fr/lire/{}/{}", p.livre_id, chapitre.id)
                            />

                            <PageDeLecture
                                fil=vec![
                                    ("/fr/lire".to_string(), "Lire".to_string()),
                                    (
                                        format!("/fr/lire/{}", p.livre_id),
                                        p.livre_titre.clone(),
                                    ),
                                ]
                                // Le nom **dans le registre du lecteur**, et non
                                // le nom ONT brut. Sans ça, on touche
                                // « Chapitre 2 » au sommaire et l'on arrive sur
                                // une page intitulée « Bereshit 2 » : deux
                                // écrans, un seul calcul, l'autre oublié.
                                //
                                // La balise `<title>` ci-dessus garde le nom
                                // ONT, elle : rendue par le serveur, qui ne
                                // connaît pas les préférences, et employée pour
                                // le référencement et le partage — deux usages
                                // où un nom stable vaut mieux qu'un nom juste.
                                titre=nom_d_unite(chapitre.titre.clone(), chapitre.numero)
                                chapeau=chapeau
                            >
                                <ReglagesDeLecture preferences />

                                <Blocs blocs=chapitre.blocs en_avant=en_avant />

                                // Les notes de bas d'unité : les décisions de
                                // traduction que le vault consigne. Elles sont
                                // sous un filet et en retrait — ce n'est plus le
                                // texte, c'est ce qui l'explique.
                                {(!chapitre.notes.is_empty())
                                    .then(|| {
                                        view! {
                                            <aside class="mt-20 border-t border-filet pt-10 text-[0.92em] text-encre-douce">
                                                <h2 class="mb-6 text-sm uppercase tracking-capitales text-accent">
                                                    "Notes de traduction"
                                                </h2>
                                                <Blocs blocs=chapitre.notes />
                                            </aside>
                                        }
                                    })}

                                <Voisins precedent=p.precedent suivant=p.suivant />
                            </PageDeLecture>
                        }
                            .into_any()
                    }
                    _ => view! { <Absent /> }.into_any(),
                }
            })}
        </Suspense>
    }
}

/// La navigation d'une unité à la suivante.
///
/// Elle est en bas et non en haut : quelqu'un qui arrive ici vient lire, et lui
/// proposer de partir avant qu'il n'ait commencé serait absurde. En bas, il a
/// fini — et c'est le seul moment où « la suite » veut dire quelque chose.
#[component]
fn Voisins(
    precedent: Option<crate::api::VoisinDto>,
    suivant: Option<crate::api::VoisinDto>,
) -> impl IntoView {
    if precedent.is_none() && suivant.is_none() {
        return ().into_any();
    }

    view! {
        <nav class="mt-20 flex flex-wrap justify-between gap-6 border-t border-filet pt-10 text-sm uppercase tracking-capitales">
            {precedent
                .map(|v| {
                    let nom = nom_d_unite(v.titre, v.numero);
                    view! {
                        <a href=v.chemin class="text-accent no-underline">
                            <span aria-hidden="true">"← "</span>
                            {move || nom.get()}
                        </a>
                    }
                })}
            // La marge automatique pousse le suivant à droite même quand le
            // précédent manque — sur la première unité d'un livre.
            {suivant
                .map(|v| {
                    let nom = nom_d_unite(v.titre, v.numero);
                    view! {
                        <a href=v.chemin class="ms-auto text-accent no-underline">
                            {move || nom.get()}
                            <span aria-hidden="true">" →"</span>
                        </a>
                    }
                })}
        </nav>
    }
    .into_any()
}

/// Ce que montre une messagerie quand le lien y est collé.
///
/// Les versets désignés d'abord : un lien vers `?v=1-3` a été partagé **pour**
/// ces trois versets, et l'aperçu qui montrerait autre chose trahirait
/// l'intention de qui l'a envoyé.
///
/// Le corps seul, sans gloses ni translittérations — c'est déjà la règle du
/// verset du jour, et pour la même raison : sortie de son appareil critique, où
/// elle est consultable et attribuée, une glose devient une affirmation sans
/// recours.
fn apercu(chapitre: &crate::domaine::corpus::Chapitre, en_avant: &[u32], livre: &str) -> String {
    let choisis: Vec<String> = en_avant
        .iter()
        .filter_map(|numero| chapitre.verset(*numero))
        .map(|v| v.corps())
        .collect();

    if !choisis.is_empty() {
        return tronquer(&choisis.join(" "), 200);
    }

    // Sans sélection, le début du chapitre — c'est ce qu'on verrait en
    // l'ouvrant.
    match chapitre.versets().next() {
        Some(premier) => tronquer(&premier.corps(), 200),
        None => format!("{livre} — un passage de La Bible ONT."),
    }
}

/// Coupe à la limite d'un mot, et pose une ellipse.
///
/// Une coupe brutale au caractère près tomberait au milieu d'un mot, et sur du
/// texte multi-octets elle tomberait au milieu d'un caractère — ce qui n'est
/// même pas une chaîne valide.
fn tronquer(texte: &str, limite: usize) -> String {
    if texte.chars().count() <= limite {
        return texte.to_string();
    }
    let coupe: String = texte.chars().take(limite).collect();
    match coupe.rfind(' ') {
        Some(espace) => format!("{}…", &coupe[..espace]),
        None => format!("{coupe}…"),
    }
}

/// Le passage qui n'existe pas.
#[component]
fn Absent() -> impl IntoView {
    view! {
        <Tete
            titre="Passage introuvable"
            description="Ce passage n'a pas encore été restitué."
            chemin="/fr/lire"
        />
        <leptos_meta::Meta name="robots" content="noindex, follow" />

        <PageDeLecture
            fil=vec![("/fr/lire".to_string(), "Lire".to_string())]
            rappel="Le corpus"
            titre="Ce passage n'est pas encore là"
        >
            <p class="text-encre-douce text-pretty">
                "Le lien est peut-être ancien, ou l'unité n'a pas encore été traduite. \
                 Le sommaire dit ce qui se lit aujourd'hui."
            </p>
        </PageDeLecture>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn une_coupe_tombe_sur_un_blanc() {
        assert_eq!(tronquer("un deux trois quatre", 12), "un deux…");
    }

    #[test]
    fn un_texte_court_n_est_pas_coupe() {
        assert_eq!(tronquer("court", 200), "court");
    }

    /// La coupe compte des **caractères**, pas des octets : une limite en
    /// octets tomberait au milieu d'un « é » et produirait une chaîne invalide.
    #[test]
    fn une_coupe_ne_casse_pas_un_caractere_multioctet() {
        let texte = "éééééééééé ééééé";
        let coupe = tronquer(texte, 12);
        assert!(texte.starts_with(coupe.trim_end_matches('…')));
    }
}
