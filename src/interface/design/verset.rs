use leptos::prelude::*;

use crate::domaine::texte::{CibleDuNiveauTrois, Noeud, Verset as VersetDomaine};

/// Un verset de l'ONT, avec ses trois niveaux.
///
/// C'est le composant le plus important du site : il est la seule chose qui
/// montre ce que la traduction fait. Le décrire ne suffit pas — il faut le
/// voir sur un vrai verset.
///
/// Deux marquages, deux promesses, et il ne faut jamais les confondre :
/// **l'or promet une fiche** et le lien la tient ; le bordeaux clair marque un
/// une accentuation et ne promet rien, donc il n'est pas cliquable. Un bordeaux
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

/// La part latine du niveau 3 — un lien quand elle ouvre une fiche, du texte
/// sinon.
///
/// **Seule la part latine devient cliquable.** L'hébreu reste hors du lien : il
/// se compose en RTL, et une zone cliquable à cheval sur la barre oblique
/// traverserait deux directions d'écriture.
///
/// **La couleur dit d'avance ce qui répond.** Sur 2086 translittérations, 829
/// ouvrent une fiche et 1257 non ; laissées toutes grises, elles obligeraient
/// le lecteur à essayer sur chacune pour savoir sur laquelle essayer. Les
/// teintes sont celles du corps du texte — l'accent pour un intraduisible, la
/// teinte des Shemot pour un nom propre —, et elles disent la même chose :
/// ceci ouvre, et voilà quoi.
///
/// L'italique reste dans les trois cas : c'est la marque du niveau 3, et elle
/// ne dépend pas de ce que le mot ouvre.
fn rendre_la_translitteration(mot: &str, cible: Option<&CibleDuNiveauTrois>) -> AnyView {
    let (lemme, teinte) = match cible {
        Some(CibleDuNiveauTrois::Terme(l)) => (l, "text-accent decoration-accent/40"),
        Some(CibleDuNiveauTrois::Shem(l)) => (l, "text-shem decoration-shem/40"),
        None => return view! { <i>{mot.to_string()}</i> }.into_any(),
    };
    let mot = mot.to_string();
    view! {
        <a href=format!("/fr/lexique/{lemme}") class=teinte>
            <i>{mot}</i>
        </a>
    }
    .into_any()
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

        // Un Shem se rend comme un intraduisible — même graisse, même
        // soulignement au survol — et change de couleur seulement. Les deux
        // promettent une fiche ; leur forme doit dire la même promesse, faute
        // de quoi le lecteur apprendrait deux gestes pour un seul.
        Noeud::Shem { mot, lemme } => view! {
            <a
                href=format!("/fr/lexique/{lemme}")
                class="font-semibold text-shem decoration-shem/40"
            >
                {mot.clone()}
            </a>
        }
        .into_any(),

        Noeud::Accentuation(enfants) => view! {
            <b class="font-semibold text-accentuation">{rendre(enfants)}</b>
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
            cible,
        } => view! {
            <span class="text-[0.86em] text-encre-douce">
                "("{rendre_la_translitteration(translitteration, cible.as_ref())}
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
        Noeud::Lien { href, enfants } => {
            // **Un renvoi interne n'ouvre pas d'onglet.**
            //
            // Le pipeline lie « Bereshit 9:5 » vers l'unité qui le contient —
            // service que seule la machine peut rendre, puisque les unités ONT
            // ne coïncident pas avec les chapitres reçus. Mais il l'écrit en
            // adresse **absolue**, pour qu'une liseuse installée qui ne sait
            // pas intercepter ouvre quand même quelque chose d'utile.
            //
            // Ici, on est déjà sur le site : on rend le chemin relatif et on
            // navigue en place. Envoyer le lecteur dans un onglet neuf pour
            // aller trois écrans plus loin dans le même livre serait absurde.
            let (cible, externe) = match classer(href) {
                Destination::Interne(chemin) => (chemin, false),
                Destination::Externe(adresse) => (adresse, true),
                // Un schéma qu'on ne sert pas ne devient pas un lien : le
                // libellé reste, et il reste lisible.
                Destination::Refusee => {
                    return view! { <span>{rendre(enfants)}</span> }.into_any();
                }
            };
            view! {
                <a
                    href=cible
                    target=externe.then_some("_blank")
                    rel=externe.then_some("noopener noreferrer")
                    class="underline decoration-filet underline-offset-4 hover:decoration-accent"
                >
                    {rendre(enfants)}
                </a>
            }
            .into_any()
        }

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
        assert_eq!(
            composer("se lisent ; les autres"),
            "se lisent\u{202F}; les autres"
        );
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

/// Aucun lien du corpus ne mène nulle part.
///
/// ## Le défaut qu'il attrape n'existe pas encore, et c'est le moment de le
/// poser
///
/// Le pipeline écrit ses renvois en adresse **absolue** — « Bereshit 9:5 » vers
/// l'unité qui le contient — et le rendu s'appuie là-dessus : ce qui ne commence
/// pas par le domaine est traité comme un lien **extérieur**, donc ouvert dans
/// un onglet neuf avec `noopener`.
///
/// Une couche de noms propres arrive, décidée le 29 août 2026 : les **Shemot**
/// deviendront touchables, avec une fiche chacun. Le pipeline les rend
/// aujourd'hui en `link` avec un `href` **relatif** — `"Qayin"`, `"Chavah"` —
/// en attendant un type propre.
///
/// Si cela arrivait dans `dist/` sans que le rendu ait bougé, chacun de ces noms
/// deviendrait **un lien souligné qui ouvre un onglet neuf vers une page qui
/// n'existe pas**. Mesuré sur le pilote : 131 occurrences dans *Bereshit* 4.
///
/// Et rien ne le signalerait. La construction du corpus reste verte — le
/// tokeniseur ne se plaint pas d'un lien qui ne mène nulle part, il n'a pas à
/// le savoir. C'est un contrôle exact sur une question qu'on n'avait pas posée.
///
/// ## Ce test **doit** rougir quand la couche arrive
///
/// C'est son objet. Tant que le rendu ne sait pas distinguer un **Shem** d'un
/// renvoi, un corpus qui en porte est un corpus que le site rendrait mal — et il
/// vaut mieux l'apprendre ici qu'en voyant 131 liens morts en ligne.
#[cfg(all(test, feature = "ssr"))]
mod liens {
    use crate::application::ports::Corpus;
    use crate::domaine::texte::Noeud;
    use crate::infrastructure::corpus::CorpusEmbarque;

    /// Tout `href` du corpus est absolu.
    #[test]
    fn aucun_lien_du_corpus_n_est_relatif() {
        let corpus = CorpusEmbarque::charger().expect("le corpus s'ouvre");

        let mut relatifs: Vec<String> = Vec::new();
        for ensemble in corpus.sommaire() {
            for section in &ensemble.sections {
                for entree in &section.livres {
                    let Some(livre) = corpus.livre(&entree.id) else {
                        continue;
                    };
                    for unite in livre.intro.iter().chain(livre.chapitres.iter()) {
                        for verset in unite.versets() {
                            releve(&verset.noeuds, &mut relatifs);
                        }
                    }
                }
            }
        }

        relatifs.sort();
        relatifs.dedup();
        assert!(
            relatifs.is_empty(),
            "{} lien(s) du corpus portent un `href` relatif : {:?}\n\
             Le rendu les traite comme extérieurs — onglet neuf, `noopener` — et \
             ils mènent à une page qui n'existe pas.\n\
             Si c'est la couche des Shemot qui arrive, c'est le rendu qu'il faut \
             faire avant, pas ce test qu'il faut assouplir.",
            relatifs.len(),
            relatifs.iter().take(6).collect::<Vec<_>>()
        );
    }

    fn releve(noeuds: &[Noeud], relatifs: &mut Vec<String>) {
        for n in noeuds {
            match n {
                Noeud::Lien { href, enfants } => {
                    if !href.starts_with("http") {
                        relatifs.push(href.clone());
                    }
                    releve(enfants, relatifs);
                }
                // `Intraduisible` porte un mot et un lemme, pas d'enfants :
                // il ne peut donc pas contenir de lien.
                Noeud::Accentuation(enfants) | Noeud::Glose(enfants) | Noeud::Emphase(enfants) => {
                    releve(enfants, relatifs)
                }
                _ => {}
            }
        }
    }
}

/// Où mène un lien du corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Destination {
    /// Le site lui-même — on rend le chemin et on navigue en place.
    Interne(String),
    /// Ailleurs — onglet neuf, `noopener noreferrer`.
    Externe(String),
    /// Un schéma qu'on ne sert pas. Le lien n'est pas rendu.
    Refusee,
}

/// L'origine du site, telle qu'elle s'écrit dans le corpus.
const SITE: &str = "https://ontbible.com";

/// Classe un lien du corpus par son **origine**, et non par son préfixe.
///
/// ## Ce que le préfixe laissait passer
///
/// La version d'avant faisait `href.strip_prefix("https://ontbible.com")`. Un
/// préfixe textuel n'est pas une origine :
///
/// * `https://ontbible.com.exemple.net/x` **commence** par la chaîne, sans être
///   le site. Il était classé interne, rendu en adresse relative, et privé du
///   `noopener` que tout lien étranger doit porter ;
/// * `javascript:` et `data:` ne commençaient par rien de connu, donc partaient
///   en lien **externe intact**. Le corpus est éditorial et rien de tel n'y
///   figure — l'audit le dit et je l'ai vérifié —, mais une garde qui dépend du
///   contenu n'en est pas une : le vault gagne des liens à chaque parashah, et
///   personne ne relit un `href` en écrivant de la prose.
///
/// ## Ce qui fait une origine
///
/// L'adresse est interne si elle vaut l'origine **exactement**, ou si le
/// caractère qui suit est `/`, `?` ou `#`. C'est ce que dit la norme d'URL, et
/// c'est ce qui distingue `ontbible.com/fr` de `ontbible.com.exemple.net`.
///
/// ## Les schémas servis
///
/// `http`, `https`, `mailto`, et les chemins relatifs. Tout le reste est refusé
/// — liste blanche et non liste noire : une liste noire oublie toujours le
/// schéma qu'on n'a pas imaginé, et il en naît.
pub fn classer(href: &str) -> Destination {
    let brut = href.trim();

    // Les blancs et les caractères de contrôle se glissent entre le schéma et
    // ses deux-points — `java\nscript:` — et certains analyseurs les ignorent.
    // On refuse plutôt que de deviner lequel le navigateur emploiera.
    if brut.chars().any(|c| c.is_control()) {
        return Destination::Refusee;
    }

    if let Some(reste) = brut.strip_prefix(SITE) {
        if reste.is_empty() {
            return Destination::Interne("/".to_string());
        }
        if reste.starts_with('/') || reste.starts_with('?') || reste.starts_with('#') {
            return Destination::Interne(reste.to_string());
        }
        // `ontbible.com.exemple.net` — le préfixe est là, l'origine non.
        return Destination::Externe(brut.to_string());
    }

    // Un schéma se lit avant le premier `:`, et seulement s'il précède le
    // premier `/`, `?` ou `#` — sinon `bereshit-1?v=1:2` passerait pour un
    // schéma nommé « bereshit-1?v=1 ».
    let fin = brut
        .find(|c| c == '/' || c == '?' || c == '#')
        .unwrap_or(brut.len());
    match brut[..fin].find(':') {
        None => {
            // Aucun schéma : un chemin. Relatif ou absolu, il reste chez nous.
            if brut.starts_with('/') {
                Destination::Interne(brut.to_string())
            } else {
                Destination::Externe(brut.to_string())
            }
        }
        Some(i) => {
            let schema = brut[..i].to_ascii_lowercase();
            match schema.as_str() {
                "http" | "https" | "mailto" => Destination::Externe(brut.to_string()),
                _ => Destination::Refusee,
            }
        }
    }
}

#[cfg(test)]
mod origines {
    use super::*;

    #[test]
    fn le_site_est_interne_et_rendu_en_chemin() {
        assert_eq!(
            classer("https://ontbible.com/fr/lire/bereshit/bereshit-1"),
            Destination::Interne("/fr/lire/bereshit/bereshit-1".into())
        );
        assert_eq!(
            classer("https://ontbible.com"),
            Destination::Interne("/".into())
        );
        assert_eq!(
            classer("https://ontbible.com/fr?v=1"),
            Destination::Interne("/fr?v=1".into())
        );
    }

    /// **Le défaut que le préfixe laissait passer.**
    ///
    /// `ontbible.com.exemple.net` commence par notre origine sans en être une.
    /// Classé interne, il était rendu en adresse relative et perdait son
    /// `noopener`.
    #[test]
    fn un_domaine_qui_commence_comme_le_notre_reste_etranger() {
        for adresse in [
            "https://ontbible.com.exemple.net/x",
            "https://ontbible.commerce.fr",
            "https://ontbible.com@exemple.net/",
        ] {
            assert_eq!(
                classer(adresse),
                Destination::Externe(adresse.into()),
                "« {adresse} » doit rester externe"
            );
        }
    }

    /// **Un schéma qu'on ne sert pas ne devient pas un lien.**
    ///
    /// Liste blanche et non liste noire : une liste noire oublie toujours le
    /// schéma qu'on n'a pas imaginé.
    #[test]
    fn les_schemas_non_servis_sont_refuses() {
        for adresse in [
            "javascript:alert(1)",
            "JavaScript:alert(1)",
            "data:text/html;base64,PHNjcmlwdD4=",
            "vbscript:msgbox",
            "file:///etc/passwd",
            "ont://read/bereshit",
        ] {
            assert_eq!(
                classer(adresse),
                Destination::Refusee,
                "« {adresse} » ne doit pas devenir un lien"
            );
        }
    }

    /// Un caractère de contrôle sépare le schéma de ses deux-points chez
    /// certains analyseurs et pas chez d'autres. On refuse plutôt que de parier.
    #[test]
    fn un_caractere_de_controle_fait_refuser() {
        assert_eq!(classer("java\nscript:alert(1)"), Destination::Refusee);
        assert_eq!(classer("java\tscript:alert(1)"), Destination::Refusee);
    }

    #[test]
    fn les_schemas_servis_passent() {
        assert_eq!(
            classer("https://sefaria.org/x"),
            Destination::Externe("https://sefaria.org/x".into())
        );
        assert_eq!(
            classer("mailto:contact@ontbible.com"),
            Destination::Externe("mailto:contact@ontbible.com".into())
        );
    }

    /// Un chemin sans schéma reste chez nous — et `bereshit-1?v=1:2` n'est pas
    /// un schéma nommé « bereshit-1?v=1 ».
    #[test]
    fn un_chemin_n_est_pas_un_schema() {
        assert_eq!(
            classer("/fr/lexique"),
            Destination::Interne("/fr/lexique".into())
        );
        assert_eq!(
            classer("/fr/lire/bereshit/bereshit-1?v=1:2"),
            Destination::Interne("/fr/lire/bereshit/bereshit-1?v=1:2".into())
        );
    }
}
