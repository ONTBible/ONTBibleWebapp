use leptos::prelude::*;

use crate::api::prononciation;
use crate::interface::design::{fournir_preferences, Blocs, PageDeLecture};
use crate::interface::tete::Tete;

/// `/fr/lexique/prononciation` — comment se prononce ce qui est écrit.
///
/// ## Ce qu'elle répare
///
/// Le corpus écrit `chokhmah`, `malʾakh`, `Chanokh`, et la graphie ne prévient
/// personne. L'auteur lui-même prononçait *Chanokh* « cha-no-q » : les deux
/// consonnes fausses, et rien pour le lui dire.
///
/// ## Une page, là où l'app a une feuille
///
/// C'est la seule chose qui change, et elle change pour une raison. Une
/// `.sheet` est modale : elle couvre le lexique, on la referme, elle n'a pas
/// d'adresse. Sur le web, une page a une adresse — donc elle se partage, se
/// met en signet, s'indexe, et le retour du navigateur la referme.
///
/// La question que la session iOS avait posée était la bonne : *ce qui doit
/// rester modal*, et non comment le faire. Celle-ci n'a aucune raison de
/// l'être — elle ne demande rien, elle explique.
///
/// ## Elle ne compose rien
///
/// Le titre et les blocs viennent de `lexique/prononciation.md`, par le
/// pipeline. Écrire ici une explication de la prononciation en ferait une
/// seconde source, qui divergerait du vault à la première correction — c'est
/// mot pour mot la règle que l'app s'est donnée.
#[component]
pub fn Prononciation() -> impl IntoView {
    // **`Blocs` lit les réglages de lecture**, et cette feuille en porte : ses
    // exemples citent des intraduisibles et de l'hébreu, donc les niveaux 2 et
    // 3 y valent quelque chose. Sans cet appel, le réglage n'aurait aucun
    // effet ici — et en `--release` rien ne le dirait, la page rendant `200`.
    // La garde de `reglages_de_lecture` l'a refusée avant qu'elle ne parte.
    let _preferences = fournir_preferences();
    let feuille = Resource::new_blocking(|| (), |_| async { prononciation().await });

    view! {
        <Tete
            titre="Prononcer l'hébreu translittéré"
            description="Les cinq sons que le français n'a pas, et comment lire \
                         chokhmah, malʾakh ou Chanokh sans se tromper."
            chemin=crate::domaine::chemins::prononciation(crate::interface::arbre::arbre_maintenant())
        />

        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                match feuille.await {
                    Ok(Some(feuille)) => {
                        view! {
                            <PageDeLecture
                                fil=vec![(crate::domaine::chemins::lexique(crate::interface::arbre::arbre_maintenant()), "Lexique".to_string())]
                                titre=feuille.titre
                            >
                                <Blocs blocs=feuille.blocs />
                            </PageDeLecture>
                        }
                            .into_any()
                    }
                    // **Le pipeline ne l'émet pas encore.** Ce n'est pas une
                    // erreur du lecteur, et ce n'est pas une panne : c'est un
                    // `dist/` antérieur à cette feuille. La carte du lexique ne
                    // se pose pas dans ce cas, donc on n'arrive ici qu'en
                    // tapant l'adresse.
                    _ => view! { <Absente /> }.into_any(),
                }
            })}
        </Suspense>
    }
}

/// Ce que voit quelqu'un dont le `dist/` est antérieur à la feuille.
#[component]
fn Absente() -> impl IntoView {
    view! {
        <PageDeLecture
            fil=vec![(crate::domaine::chemins::lexique(crate::interface::arbre::arbre_maintenant()), "Lexique".to_string())]
            titre="Comment se prononce ce qui est écrit"
        >
            <p>
                "Cette feuille expliquera comment se prononce ce qui est écrit. "
                "Elle n'est pas encore publiée."
            </p>
        </PageDeLecture>
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use crate::application::ports::Lexique;
    use crate::infrastructure::corpus::LexiqueEmbarque;

    /// ## La route statique ne doit masquer aucune fiche
    ///
    /// `/fr/lexique/prononciation` passe **avant** `/fr/lexique/{lemme}` : sans
    /// cet ordre, le routeur prendrait la seconde et la page ne répondrait
    /// jamais. Le prix de cet ordre est qu'un lemme portant ce nom deviendrait
    /// injoignable — et il le deviendrait **en silence**, sa fiche existant
    /// toujours et chaque mot d'or du corpus continuant d'y pointer.
    ///
    /// Aucun lemme ne s'appelle ainsi aujourd'hui : ce sont des
    /// translittérations de l'hébreu. Mais c'est une propriété du vault, pas
    /// du site, et le vault n'a aucune raison de la connaître.
    #[test]
    fn aucune_fiche_ne_porte_le_nom_de_la_feuille() {
        let lexique = LexiqueEmbarque::charger().expect("le lexique s'ouvre");
        assert!(
            lexique.entree("prononciation").is_none(),
            "une fiche s'appelle « prononciation » : elle est masquée par la \
             route de la feuille, qui passe avant celle des fiches dans \
             `app.rs`. Renommer l'une des deux — la feuille est la moins \
             coûteuse à déplacer."
        );
    }

    /// La feuille est bien là, et elle porte quelque chose.
    ///
    /// Elle **se tait** si le `dist/` est antérieur à son émission : la page
    /// rend alors `Absente`, ce qui est le comportement voulu. Ce qu'on refuse
    /// est une feuille présente et vide, qui mènerait à une page blanche
    /// derrière une carte qui promet.
    #[test]
    fn la_feuille_publiee_n_est_pas_vide() {
        let lexique = LexiqueEmbarque::charger().expect("le lexique s'ouvre");
        let Some(feuille) = lexique.prononciation() else {
            return;
        };
        assert!(!feuille.titre.trim().is_empty(), "un titre");
        assert!(!feuille.blocs.is_empty(), "des blocs");
    }
}
