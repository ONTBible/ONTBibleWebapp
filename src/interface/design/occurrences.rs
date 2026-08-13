use leptos::prelude::*;

use crate::domaine::corpus::Occurrence;

/// Où un terme paraît dans le corpus.
///
/// C'est la moitié qui manque à une définition. Une fiche qui explique ce que
/// *neshamah* porte, sans montrer les endroits où le mot est employé, demande
/// au lecteur de la croire sur parole — alors que le corpus est là, à un clic,
/// et que c'est lui qui prouve.
///
/// L'extrait vient du pipeline, coupé par lui : il porte le mot dans sa phrase.
/// Il n'est **pas** composé avec les trois niveaux, et c'est délibéré — c'est
/// une chaîne plate, dont les gloses ont déjà été retirées. Y remettre de l'or
/// donnerait deux ors dans la même page, l'un menant à cette fiche-ci.
///
/// Le lien mène au verset quand il y en a un, au chapitre sinon : 319 des 2033
/// occurrences vivent dans un titre intercalaire ou une note, où aucun numéro
/// ne s'applique.
#[component]
pub fn Occurrences(occurrences: Vec<Occurrence>) -> impl IntoView {
    let total = occurrences.len();

    view! {
        <section class="mt-16">
            <h2 class="mb-2 flex items-center gap-4 text-encre-vive">
                <span class="massif w-7 shrink-0 text-accent"></span>
                "Dans le corpus"
            </h2>
            <p class="mb-8 text-sm text-encre-douce">
                {if total > 1 {
                    format!("{total} occurrences")
                } else {
                    format!("{total} occurrence")
                }}
            </p>

            <ul class="m-0 list-none p-0">
                {occurrences
                    .into_iter()
                    .map(|occurrence| {
                        let chemin = match occurrence.verset {
                            Some(numero) => {
                                format!(
                                    "/fr/lire/{}/{}?v={numero}#v{numero}",
                                    occurrence.livre,
                                    occurrence.chapitre,
                                )
                            }
                            None => format!("/fr/lire/{}/{}", occurrence.livre, occurrence.chapitre),
                        };
                        // Le renvoi lisible se compose depuis l'identifiant de
                        // chapitre, qui porte déjà le livre — « bereshit-10 »
                        // devient « Bereshit 10 ». Le pipeline n'écrit pas de
                        // renvoi tout fait pour une occurrence.
                        let renvoi = renvoi_lisible(&occurrence.chapitre, occurrence.verset);
                        view! {
                            <li class="border-b border-filet/40 last:border-0">
                                <a href=chemin class="block py-4 no-underline">
                                    <span class="mb-1.5 block text-sm uppercase tracking-capitales text-accent">
                                        {renvoi}
                                    </span>
                                    <span class="block text-[0.92em] leading-relaxed text-encre-douce">
                                        {occurrence.extrait}
                                    </span>
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}

/// « bereshit-10 » et le verset 6 donnent « Bereshit 10:6 ».
///
/// Le dernier segment de l'identifiant est le numéro d'unité quand c'en est un.
/// Une introduction — « toledot-adam-ve-chavah-0-intro » — n'en a pas : on rend
/// alors le nom du livre suivi de « intro », plutôt qu'un numéro inventé.
fn renvoi_lisible(chapitre: &str, verset: Option<u32>) -> String {
    let mut segments: Vec<&str> = chapitre.split('-').collect();
    let dernier = segments.pop().unwrap_or_default();

    let (nom, suffixe) = match dernier.parse::<u32>() {
        Ok(numero) => (segments.join(" "), numero.to_string()),
        // Pas un nombre : c'est une introduction, et le segment porte son nom.
        Err(_) => (segments.join(" "), dernier.to_string()),
    };

    let nom = capitaliser(&nom);
    match verset {
        Some(numero) => format!("{nom} {suffixe}:{numero}"),
        None => format!("{nom} {suffixe}"),
    }
}

/// « bereshit » devient « Bereshit », « toledot adam ve chavah » devient
/// « Toledot adam ve chavah ».
fn capitaliser(texte: &str) -> String {
    let mut caracteres = texte.chars();
    match caracteres.next() {
        Some(premier) => premier.to_uppercase().collect::<String>() + caracteres.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_renvoi_de_chapitre_et_de_verset() {
        assert_eq!(renvoi_lisible("bereshit-10", Some(6)), "Bereshit 10:6");
    }

    #[test]
    fn un_renvoi_sans_verset() {
        assert_eq!(renvoi_lisible("bereshit-10", None), "Bereshit 10");
    }

    /// Le cas qui casse une découpe naïve : le nom du livre porte lui-même des
    /// tirets, et l'unité n'est pas un nombre.
    #[test]
    fn une_introduction_ne_recoit_pas_de_numero_invente() {
        assert_eq!(
            renvoi_lisible("toledot-adam-ve-chavah-0-intro", None),
            "Toledot adam ve chavah 0 intro"
        );
    }

    #[test]
    fn un_livre_a_nom_compose() {
        assert_eq!(
            renvoi_lisible("sefar-gibbaraya-3", Some(2)),
            "Sefar gibbaraya 3:2"
        );
    }
}
