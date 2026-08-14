use leptos::prelude::*;

use crate::domaine::texte::{Noeud, Verset as VersetDomaine};

/// Un verset de l'ONT, avec ses trois niveaux.
///
/// C'est le composant le plus important du site : il est la seule chose qui
/// montre ce que la traduction fait. Le décrire ne suffit pas — il faut le
/// voir sur un vrai verset.
///
/// Deux marquages, deux promesses, et il ne faut jamais les confondre :
/// **l'or promet une fiche** et le lien la tient ; le bordeaux clair marque un
/// terme important et ne promet rien, donc il n'est pas cliquable. Un bordeaux
/// cliquable mentirait ; un or inerte trahirait.
#[component]
pub fn Verset(verset: VersetDomaine) -> impl IntoView {
    view! {
        <p class="font-corps text-lg leading-loose text-pretty">
            <span
                aria-hidden="true"
                class="me-[0.35em] align-[0.55em] text-[0.62em] text-accent"
            >
                {verset.numero}
            </span>
            {rendre(&verset.noeuds)}
        </p>
    }
}

/// Une suite de nœuds, composée.
///
/// Publique parce que le corpus en porte **hors** des versets : dans un titre
/// intercalaire, une puce de liste, une cellule de tableau, une définition de
/// lexique. Les composer ailleurs avec un autre code ferait deux ors — celui
/// d'un verset et celui d'une liste — qui finiraient par diverger, et l'un des
/// deux cesserait de mener à sa fiche.
pub fn rendre_noeuds(noeuds: &[Noeud]) -> Vec<AnyView> {
    noeuds.iter().map(rendre_un).collect()
}

fn rendre(noeuds: &[Noeud]) -> Vec<AnyView> {
    rendre_noeuds(noeuds)
}

/// Les espaces que le français exige autour de la ponctuation double.
///
/// ## Le défaut qu'elle répare
///
/// Le corpus porte **2124** espaces ordinaires devant `;` `:` `!` `?` `»`, et
/// pas une seule insécable — le vault écrit des mots, pas de la composition.
/// Or une espace ordinaire est un point de coupure : le navigateur y renvoie
/// volontiers la ponctuation à la ligne suivante, et l'on obtient une ligne qui
/// commence par « ; ». C'est arrivé dès la première capture d'écran de la
/// liseuse, sur une phrase de quinze mots.
///
/// ## Deux espaces, et ce n'est pas la même
///
/// L'usage de l'Imprimerie nationale distingue :
///
/// * une **fine insécable** (U+202F) devant `;` `!` `?` `»` et après `«` ;
/// * une **insécable pleine** (U+00A0) devant `:`, plus large.
///
/// Les deux sont insécables, ce qui est le point : la ponctuation ne peut plus
/// se détacher de son mot. La différence de chasse est ce qui distingue une
/// page composée d'une page tapée.
///
/// ## Pourquoi ici et pas dans le pipeline
///
/// Le vault est la source du **texte**. La composition française est une
/// affaire de rendu : la corriger ici la corrige pour tout le corpus, y compris
/// les livres qui n'existent pas encore, et sans toucher à une ligne de la
/// traduction. L'app, elle, applique sa propre composition — c'est le même
/// partage.
///
/// Une limite connue : la correction ne voit que l'intérieur d'un fragment. Si
/// le pipeline coupait entre l'espace et sa ponctuation — un intraduisible
/// suivi d'un deux-points — la paire lui échapperait. Les 2124 relevées sont
/// toutes internes.
///
/// ## Publique, parce que le corpus ne voyage pas qu'en nœuds
///
/// Elle ne s'appliquait qu'à `Noeud::Texte`, donc à ce qui traverse l'arbre.
/// Or le pipeline livre aussi des **chaînes nues** : le rendu d'un
/// intraduisible, l'extrait d'une occurrence. Elles sortaient droit du JSON,
/// non composées — vingt-trois coupures possibles sur la seule fiche d'`adam`,
/// et personne pour les voir puisque la page ne casse pas.
///
/// Toute chaîne du corpus posée dans une page passe donc par ici. La règle
/// n'existe qu'une fois, et c'est la condition pour qu'elle reste vraie.
pub fn composer(texte: &str) -> String {
    const FINE: char = '\u{202F}';
    const INSECABLE: char = '\u{00A0}';

    let mut sortie = String::with_capacity(texte.len() + 8);
    let mut caracteres = texte.chars().peekable();

    while let Some(caractere) = caracteres.next() {
        if caractere == ' ' {
            match caracteres.peek() {
                Some(';' | '!' | '?' | '»') => {
                    sortie.push(FINE);
                    continue;
                }
                Some(':') => {
                    sortie.push(INSECABLE);
                    continue;
                }
                _ => {}
            }
        }
        sortie.push(caractere);
        // Le guillemet ouvrant appelle sa fine **après** lui.
        if caractere == '«' && caracteres.peek() == Some(&' ') {
            caracteres.next();
            sortie.push(FINE);
        }
    }
    sortie
}

fn rendre_un(noeud: &Noeud) -> AnyView {
    match noeud {
        Noeud::Texte(t) => composer(t).into_any(),

        Noeud::Intraduisible { mot, lemme } => view! {
            <a
                href=format!("/fr/lexique/{lemme}")
                class="font-semibold text-accent decoration-accent/40"
            >
                {mot.clone()}
            </a>
        }
        .into_any(),

        Noeud::Important(enfants) => view! {
            <b class="font-semibold text-important">{rendre(enfants)}</b>
        }
        .into_any(),

        // 0,86 × le corps, et l'encre à 62 % : les deux valeurs de
        // `ONTTypography`. Une glose composée autrement ici ferait deux
        // niveaux 2 différents entre le site et l'app.
        Noeud::Glose(enfants) => view! {
            <span class="text-[0.86em] italic text-encre-douce">
                "["{rendre(enfants)}"]"
            </span>
        }
        .into_any(),

        // L'hébreu s'écrit de droite à gauche au milieu d'une phrase française.
        // Sans isolation, la ponctuation qui le suit part du mauvais côté — un
        // point de fin de phrase se retrouve devant le mot. `dir="rtl"` sur
        // l'élément suffit à isoler la séquence.
        Noeud::Hebreu {
            translitteration,
            hebreu,
        } => view! {
            <span class="text-[0.86em] text-encre-douce">
                "("<i>{translitteration.clone()}</i>
                " / "
                // 1,08 — `ONTFonts.hebrewScale`. L'hébreu compose plus petit
                // que le latin à taille égale : sans cette correction, les deux
                // écritures ne s'accordent pas sur une même ligne.
                <span dir="rtl" lang="he" class="font-hebreu text-[1.08em] not-italic">
                    {hebreu.clone()}
                </span>")"
            </span>
        }
        .into_any(),

        // De l'hébreu seul, sans translittération — un suffixe, une racine
        // citée dans une fiche. Même fonte et même correction d'échelle que
        // ci-dessus, mais sans les parenthèses : elles n'encadreraient rien.
        Noeud::HebreuNu(hebreu) => view! {
            <span dir="rtl" lang="he" class="font-hebreu text-[1.08em] not-italic">
                {hebreu.clone()}
            </span>
        }
        .into_any(),

        // Un lien vers une source extérieure. `noopener` parce que `_blank`
        // donne sinon à la page ouverte une prise sur celle-ci.
        Noeud::Lien { href, enfants } => view! {
            <a
                href=href.clone()
                target="_blank"
                rel="noopener noreferrer"
                class="underline decoration-filet underline-offset-4 hover:decoration-accent"
            >
                {rendre(enfants)}
            </a>
        }
        .into_any(),

        Noeud::Emphase(enfants) => view! { <em>{rendre(enfants)}</em> }.into_any(),

        // La coupe d'un parallélisme. Elle est dans le texte, pas dans la mise
        // en page : le second hémistiche commence une ligne, où qu'on soit.
        Noeud::Saut => view! { <br /> }.into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_fine_insecable_precede_le_point_virgule_et_l_exclamation() {
        assert_eq!(composer("se lisent ; les autres"), "se lisent\u{202F}; les autres");
        assert_eq!(composer("lumière !"), "lumière\u{202F}!");
        assert_eq!(composer("vraiment ?"), "vraiment\u{202F}?");
    }

    /// Le deux-points reçoit l'insécable **pleine**, plus large que la fine.
    #[test]
    fn le_deux_points_recoit_l_insecable_pleine() {
        assert_eq!(composer("Il dit : que"), "Il dit\u{A0}: que");
    }

    /// Les guillemets français prennent leur espace à l'intérieur.
    #[test]
    fn les_guillemets_enferment_leurs_espaces() {
        assert_eq!(
            composer("« que la lumière soit »"),
            "«\u{202F}que la lumière soit\u{202F}»"
        );
    }

    /// Ce qui n'appelle pas d'espace n'en reçoit pas — une virgule, un point.
    #[test]
    fn la_ponctuation_simple_est_laissee_telle_quelle() {
        assert_eq!(composer("un, deux. trois"), "un, deux. trois");
    }

    /// Et le texte sans ponctuation double traverse sans être touché — c'est la
    /// grande majorité des fragments.
    #[test]
    fn un_texte_ordinaire_traverse_intact() {
        let texte = "Quand Elohim commença à orchestrer les Cieux et la Terre";
        assert_eq!(composer(texte), texte);
    }
}
