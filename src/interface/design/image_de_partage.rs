use leptos::prelude::*;

/// L'image d'un passage, telle qu'elle part dans une conversation.
///
/// ## Ce commentaire disait que c'était impossible
///
/// `selection_de_versets.rs` écrivait : *« Image, qui rend un carré de
/// 1080 px, demande un rendu que le navigateur ne fait pas gratuitement »*.
/// C'est faux, et la phrase avait le tort d'avoir l'air d'une mesure : un
/// `<canvas>` compose et `toBlob` rend un PNG, sans bibliothèque et sans
/// requête. Ce qui coûte n'est pas le rendu, ce sont les **fontes** — et elles
/// sont déjà chargées, puisque la page qui ouvre cette action les emploie.
///
/// ## Ce qu'elle montre, et ce qu'elle tait
///
/// Le **corps de la traduction seul** — ni gloses, ni translittérations, ni
/// hébreu. C'est la règle de l'app, et sa raison est la meilleure de tout ce
/// portage : *l'appareil critique appartient à la liseuse, où il est
/// consultable et attribué. Sorti de là, il devient une affirmation sans
/// recours, illisible pour qui ne connaît pas le projet.*
///
/// La carte porte donc le renvoi et le nom de la traduction : **une image qui
/// circule doit dire d'où elle vient**, sinon elle finit citée de travers.
///
/// ## Les mesures viennent de l'app, pas de l'œil
///
/// Côté 1080, marge 90, filet d'or de 3, renvoi à 40, signature à 38,
/// interligne à 0,42 du corps. Et les cinq paliers de taille, **grossiers
/// volontairement** : son commentaire le dit — *« une taille calculée au
/// caractère près donnerait des images qui ne se ressemblent pas d'un partage
/// à l'autre »*.
///
/// ## Le thème est celui du lecteur
///
/// L'image sort dans la peau où il lit. Les couleurs se lisent sur l'élément
/// rendu plutôt que d'être recopiées : `getComputedStyle` rend la valeur
/// **résolue**, donc celle du thème courant, et une table de littéraux ici
/// divergerait au premier jeton retouché.
#[component]
pub fn ImageDePartage(
    /// Le texte nu du passage — déjà dépouillé de ses niveaux par l'appelant.
    #[prop(into)]
    texte: Signal<String>,
    /// « Bereshit 3:1-3 ».
    #[prop(into)]
    renvoi: Signal<String>,
) -> impl IntoView {
    // Le libellé porte l'accusé de réception, comme « Copier » : dans un
    // navigateur rien ne bouge quand un fichier est rendu, et un geste sans
    // retour se répète.
    let etat = RwSignal::new("Image");

    view! {
        <button
            type="button"
            class="flex-1 rounded-xl px-2 py-2 text-sm uppercase tracking-capitales text-encre-douce transition-colors hover:bg-aubergine/40 hover:text-encre focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
            on:click=move |_| {
                etat.set("…");
                let fait = rendre(&texte.get_untracked(), &renvoi.get_untracked());
                etat.set(if fait { "Enregistrée" } else { "Indisponible" });
                // Le libellé revient de lui-même : un bouton qui garde son
                // accusé de réception cesse d'être un bouton.
                set_timeout(move || etat.set("Image"), std::time::Duration::from_secs(2));
            }
        >
            {move || etat.get()}
        </button>
    }
}

/// Le corps du texte, selon la longueur du passage.
///
/// **Les cinq paliers de l'app**, aux bornes près — `ONTShareImage.size`. Ils
/// sont grossiers volontairement, et son commentaire dit pourquoi : *« une
/// taille calculée au caractère près donnerait des images qui ne se
/// ressemblent pas d'un partage à l'autre »*.
///
/// Sans eux, cinq versets débordent du carré et l'image part tronquée.
///
/// Hors du rendu, et sans `cfg` : c'est de l'arithmétique, elle n'a besoin
/// d'aucun navigateur — donc elle s'éprouve, et c'est la seule partie de cette
/// pièce qui le peut. Le reste demande un canvas.
// Employée par le rendu, qui n'existe que côté navigateur, et par ses deux
// épreuves. Côté serveur elle ne sert donc à rien — mais elle **compile**, et
// c'est ce qu'on veut : une table de paliers ne doit pas pouvoir diverger
// entre deux cibles.
#[cfg_attr(not(any(feature = "hydrate", test)), allow(dead_code))]
fn palier(signes: usize) -> f64 {
    match signes {
        ..120 => 78.0,
        ..260 => 62.0,
        ..460 => 50.0,
        ..760 => 40.0,
        _ => 32.0,
    }
}

/// Compose la carte et la propose au téléchargement.
///
/// Rend `false` quand le navigateur ne donne pas de contexte 2D — ce qui
/// arrive derrière certaines protections anti-empreinte. **Le bouton le dit**
/// plutôt que de ne rien faire : une action qui échoue en silence se réessaie.
#[cfg(feature = "hydrate")]
fn rendre(texte: &str, renvoi: &str) -> bool {
    let Some(toile) = composer(texte, renvoi) else {
        return false;
    };
    telecharger(&toile, renvoi)
}

/// Compose la carte sur une toile, et la rend.
///
/// **Séparée du téléchargement**, et ce n'est pas du découpage pour le plaisir.
/// Un téléchargement ne se regarde pas : il faut un clic, un navigateur, et il
/// finit dans un dossier. La composition, elle, se **pose à l'écran** — et
/// c'est la seule façon de voir cette carte sans la cliquer.
///
/// C'est ce qui a permis de la vérifier : `?carte` sur un passage la monte dans
/// la page, en développement seulement, et le simulateur la photographie. Sans
/// cette coupure, la pièce serait partie « compilée mais jamais vue », ce que
/// ce dépôt a déjà payé — le bloc App Store écrit d'avance et jamais rendu.
#[cfg(feature = "hydrate")]
pub(crate) fn composer(texte: &str, renvoi: &str) -> Option<web_sys::HtmlCanvasElement> {
    use wasm_bindgen::JsCast;

    const COTE: f64 = 1080.0;
    const MARGE: f64 = 90.0;

    let Some(document) = web_sys::window().and_then(|f| f.document()) else {
        return None;
    };
    let Ok(toile) = document.create_element("canvas") else {
        return None;
    };
    let Ok(toile) = toile.dyn_into::<web_sys::HtmlCanvasElement>() else {
        return None;
    };
    toile.set_width(COTE as u32);
    toile.set_height(COTE as u32);
    let Ok(Some(ctx)) = toile.get_context("2d") else {
        return None;
    };
    let Ok(ctx) = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>() else {
        return None;
    };

    // Les couleurs du thème **courant**, lues sur la racine.
    let couleur = |nom: &str| -> String {
        web_sys::window()
            .and_then(|f| f.get_computed_style(&document.document_element()?).ok()?)
            .and_then(|s| s.get_property_value(nom).ok())
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| "#18090D".to_string())
    };
    let fond = couleur("--color-nuit");
    let encre = couleur("--color-encre");
    let or = couleur("--color-or");

    ctx.set_fill_style_str(&fond);
    ctx.fill_rect(0.0, 0.0, COTE, COTE);

    // **La composition française traverse le canvas aussi.**
    //
    // Sans elle, la carte renvoyait « au serpent » et son deux-points sur deux
    // lignes : le corpus porte des espaces **ordinaires** devant `; : ! ? »`,
    // et une espace ordinaire est un point de coupure. C'est le §8 bis, et sa
    // règle ne s'arrête pas au HTML — elle vaut partout où ce texte se replie.
    //
    // `verset::composer` rend une `String` : la même règle, au même endroit,
    // pour la page et pour l'image.
    let texte = crate::interface::design::verset::composer(texte);
    let texte = texte.as_str();
    let corps = palier(texte.chars().count());
    let interligne = corps * 1.42;

    ctx.set_fill_style_str(&encre);
    ctx.set_font(&format!("{corps}px Literata, Georgia, serif"));
    ctx.set_text_baseline("alphabetic");

    // Le pliage à la main : `fillText` ne va pas à la ligne.
    let largeur = COTE - 2.0 * MARGE;
    let mut lignes: Vec<String> = Vec::new();
    let mut courante = String::new();
    // **On coupe sur l'espace ordinaire seulement.**
    //
    // `split_whitespace` répond à « où sont les blancs », pas à « où puis-je
    // couper » : il sépare aussi sur U+202F et U+00A0, qui ont la propriété
    // `White_Space` — donc sur les insécables que `composer` vient de poser.
    // La composition aurait été défaite par le pliage, une ligne plus bas.
    for mot in texte.split(' ').filter(|m| !m.is_empty()) {
        let essai = if courante.is_empty() {
            mot.to_string()
        } else {
            format!("{courante} {mot}")
        };
        let trop_large = ctx
            .measure_text(&essai)
            .map(|m| m.width() > largeur)
            .unwrap_or(false);
        if trop_large && !courante.is_empty() {
            lignes.push(std::mem::take(&mut courante));
            courante = mot.to_string();
        } else {
            courante = essai;
        }
    }
    if !courante.is_empty() {
        lignes.push(courante);
    }

    // **Centré dans la hauteur restante**, comme les deux `Spacer` de l'app —
    // et non posé en haut : un bloc court laisserait un trou sous lui.
    let pied = COTE - MARGE - 120.0;
    let hauteur = lignes.len() as f64 * interligne;
    let mut y = ((pied - MARGE - hauteur) / 2.0).max(0.0) + MARGE + corps;
    for ligne in &lignes {
        let _ = ctx.fill_text(ligne, MARGE, y);
        y += interligne;
    }

    // Le filet d'or, puis le renvoi et la signature sur la même ligne de base.
    ctx.set_fill_style_str(&or);
    ctx.fill_rect(MARGE, pied, largeur, 3.0);

    let base = pied + 34.0 + 40.0;
    ctx.set_font("40px Literata, Georgia, serif");
    let _ = ctx.fill_text(renvoi, MARGE, base);

    ctx.set_fill_style_str(&encre);
    ctx.set_global_alpha(0.55);
    ctx.set_font("38px Jost, system-ui, sans-serif");
    ctx.set_text_align("right");
    let _ = ctx.fill_text("La Bible ONT", COTE - MARGE, base);

    Some(toile)
}

/// Propose la toile au téléchargement.
#[cfg(feature = "hydrate")]
fn telecharger(toile: &web_sys::HtmlCanvasElement, renvoi: &str) -> bool {
    use wasm_bindgen::JsCast;

    let Some(document) = web_sys::window().and_then(|f| f.document()) else {
        return false;
    };
    let Ok(donnees) = toile.to_data_url_with_type("image/png") else {
        return false;
    };
    let Ok(lien) = document.create_element("a") else {
        return false;
    };
    let Ok(lien) = lien.dyn_into::<web_sys::HtmlAnchorElement>() else {
        return false;
    };
    lien.set_href(&donnees);
    lien.set_download(&nom_de_fichier(renvoi));
    lien.click();
    true
}

/// Le nom sous lequel le fichier atterrit chez le lecteur.
///
/// ## L'app n'en a pas, et c'est pour ça que celui-ci n'avait pas été pensé
///
/// Côté app, `ActionTile("Image")` passe un `UIImage` à la feuille de partage :
/// **iOS nomme le fichier lui-même**, et aucune ligne de Swift n'en décide.
/// Un téléchargement de navigateur, lui, *doit* porter un nom — c'est donc une
/// décision propre au web, qu'aucun portage ne pouvait rapporter, et qui
/// s'était prise par défaut dans un `replace` écrit au fil de la plume.
///
/// ## Ce qu'elle produisait, et pourquoi c'était faux
///
/// « Bereshit 3:1-3 » devenait **`Bereshit-3-1-3.png`** : un seul séparateur
/// pour trois rôles — l'espace du livre, le deux-points du verset, le tiret de
/// la plage. La structure du renvoi disparaît, et le lecteur qui retrouve ce
/// fichier six mois plus tard ne sait plus s'il tient *Bereshit 3, versets 1 à
/// 3* ou *Bereshit 3:1, verset 3*.
///
/// C'est la faute de `chiffres.rs` rejouée : une forme juste dans son contexte
/// — un identifiant sans espace — employée là où c'est la lisibilité qui
/// compte.
///
/// ## Ce qu'elle produit, et ce qu'elle refuse
///
/// Seul le deux-points est remplacé, par le **`v` des renvois abrégés** ; les
/// espaces restent, car un nom de fichier en porte sans difficulté sur les
/// trois systèmes, et ce sont eux qui gardent les mots séparés.
///
/// ```text
/// Bereshit 3:1-3        →  Bereshit 3 v1-3.png
/// Bereshit 3:1-3, 7     →  Bereshit 3 v1-3, 7.png
/// Bereshit 3            →  Bereshit 3.png
/// ```
///
/// Le reste du filtrage ne porte pas sur l'élégance mais sur ce qui **casse** :
/// `/` termine un chemin sous Unix, et Windows refuse en plus
/// `\ : * ? " < > |`. Aucun ne peut sortir de `renvoi` aujourd'hui — les
/// identifiants de livres sont des translittérations — mais un livre nommé
/// « Sefar Daniyy'el / Bel » n'est pas absurde, et un nom de fichier tronqué à
/// la barre oblique donne un téléchargement dans un dossier qui n'existe pas.
/// Le filtre est donc écrit contre la classe, pas contre les cas connus.
///
/// ## Hors de `cfg(hydrate)`, et c'est la raison de l'`allow`
///
/// Elle n'est appelée que par `telecharger`, qui n'existe que côté navigateur —
/// donc en SSR elle ne sert à personne, et `rustc` a raison de le dire. On la
/// garde là quand même, parce que **c'est la seule façon d'éprouver cette
/// décision** : la CI compile et teste avec `ssr`, et une fonction rangée sous
/// `cfg(hydrate)` n'y serait jamais exécutée. C'est un décideur pur, il se
/// mesure sans navigateur, et le silence de l'avertissement est le prix de
/// cette mesure — pas un oubli.
#[allow(dead_code)]
fn nom_de_fichier(renvoi: &str) -> String {
    let lisible: String = renvoi
        .replace(':', " v")
        .chars()
        // Les séparateurs de chemin et les caractères que Windows refuse.
        // **Le deux-points n'y est pas**, et il ne faut pas l'y remettre : le
        // `replace` ci-dessus l'a déjà consommé, donc ce bras serait
        // inatteignable — et un bras mort dans une liste de caractères
        // interdits est exactement ce qui fait croire qu'un cas est couvert.
        .map(|c| match c {
            '/' | '\\' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            // Aucun ne peut venir du corpus. Ils sont rendus à l'espace et non
            // au tiret : un caractère invisible remplacé par un signe visible
            // ferait apparaître une ponctuation que le renvoi ne porte pas.
            c if c.is_control() => ' ',
            c => c,
        })
        .collect();
    // Un renvoi sans versets finit par « Bereshit 3 » : le `replace` n'a rien
    // trouvé, donc rien à resserrer. Mais « Bereshit 3: » — un cas que rien
    // n'interdit si la sélection se vide entre le calcul et le clic — donnerait
    // « Bereshit 3 v ». On resserre.
    format!("{}.png", lisible.trim().trim_end_matches(" v").trim())
}

#[cfg(not(feature = "hydrate"))]
fn rendre(_texte: &str, _renvoi: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::palier;

    /// ## Les bornes sont celles de l'app, et une borne se trompe d'un
    ///
    /// `ONTShareImage.size` s'écrit en `case ..<120`, c'est-à-dire **exclusif**.
    /// Un `0..=119` rend la même chose ; un `0..=120` décale tout d'un
    /// caractère, et le défaut ne se verrait que sur un passage dont la
    /// longueur tombe pile sur une borne — c'est-à-dire jamais, jusqu'au jour
    /// où si.
    /// Le nom de fichier garde la **structure** du renvoi.
    ///
    /// La version d'avant écrasait l'espace, le deux-points et le tiret de
    /// plage sur un seul signe : « Bereshit 3:1-3 » sortait
    /// `Bereshit-3-1-3.png`, où plus rien ne dit ce qui est un chapitre et ce
    /// qui est un verset. Cette épreuve porte les trois formes que
    /// `selection::libelle` peut produire, donc les trois qui arrivent ici.
    #[test]
    fn le_nom_de_fichier_garde_la_structure_du_renvoi() {
        for (renvoi, attendu) in [
            ("Bereshit 3:1-3", "Bereshit 3 v1-3.png"),
            ("Bereshit 3:1-3, 7", "Bereshit 3 v1-3, 7.png"),
            ("Bereshit 3:7", "Bereshit 3 v7.png"),
            // Une sélection vide : `renvoi` se tait sur les versets, et le nom
            // ne doit pas inventer un « v » sans numéro.
            ("Bereshit 3", "Bereshit 3.png"),
            // Le cas limite qu'aucun clic ne produit aujourd'hui, mais que
            // rien n'interdit si la sélection se vide entre le calcul du
            // renvoi et le clic : le « v » orphelin se retire.
            ("Bereshit 3:", "Bereshit 3.png"),
        ] {
            assert_eq!(super::nom_de_fichier(renvoi), attendu, "pour « {renvoi} »");
        }
    }

    /// Aucun nom ne peut porter un séparateur de chemin.
    ///
    /// **La garde est écrite contre la classe, pas contre les cas connus.**
    /// Aucun identifiant de livre ne porte de barre oblique aujourd'hui — ce
    /// sont des translittérations de l'hébreu —, et c'est précisément pourquoi
    /// personne ne le vérifierait le jour où un titre composé en porterait
    /// une. Un nom tronqué à la barre oblique donne un téléchargement dans un
    /// dossier qui n'existe pas, et le navigateur ne dit rien.
    #[test]
    fn aucun_nom_de_fichier_ne_porte_de_separateur_de_chemin() {
        let nom = super::nom_de_fichier("Sefar Daniyy'el / Bel 3:1-3");
        assert_eq!(nom, "Sefar Daniyy'el - Bel 3 v1-3.png");
        for interdit in ['/', '\\', ':', '*', '?', '"', '<', '>', '|'] {
            let mesure = super::nom_de_fichier(&format!("Livre{interdit}X 1:1"));
            assert!(
                !mesure.contains(interdit),
                "« {interdit} » a survécu dans « {mesure} »"
            );
        }
    }

    #[test]
    fn les_paliers_sont_ceux_de_l_app() {
        for (signes, attendu) in [
            (0, 78.0),
            (119, 78.0),
            (120, 62.0),
            (259, 62.0),
            (260, 50.0),
            (459, 50.0),
            (460, 40.0),
            (759, 40.0),
            (760, 32.0),
            (5000, 32.0),
        ] {
            assert_eq!(palier(signes), attendu, "à {signes} signes");
        }
    }

    /// Le corps **décroît**, il ne remonte jamais.
    ///
    /// Une table écrite à la main peut s'inverser sans qu'on le voie — c'est
    /// arrivé à l'échelle typographique du §5, dont deux paliers se
    /// croisaient. La propriété se vérifie sans connaître les valeurs.
    #[test]
    fn un_passage_plus_long_ne_grossit_jamais() {
        let mut precedent = f64::INFINITY;
        for signes in 0..1000 {
            let taille = palier(signes);
            assert!(taille <= precedent, "le corps remonte à {signes} signes");
            precedent = taille;
        }
    }
}

/// **Le banc**, et il porte sa marque.
///
/// `?carte` sur un passage monte la carte dans la page au lieu de la
/// télécharger. C'est le seul moyen de la **regarder** : un téléchargement
/// demande un clic, un navigateur, et finit dans un dossier — rien de tout ça
/// n'entre dans une capture.
///
/// ## Trois gardes, et la troisième est la leçon du dépôt
///
/// - **`debug_assertions` seulement.** En production, la fonction n'existe
///   pas : un banc qui peut s'allumer sur un site en ligne finit allumé ;
/// - **et `debug_assertions` des deux côtés.** Le premier jet le compilait
///   `all(debug_assertions, feature = "hydrate")` — donc présent dans le WASM
///   et **absent du rendu du serveur**. Les deux arbres auraient divergé à
///   l'hydratation, ce qui est exactement la panne qu'on venait de corriger
///   une heure plus tôt : le navigateur trouve un nœud là où le serveur n'a
///   rien écrit, le WASM meurt, plus rien ne répond.
///
///   Un outil de diagnostic qui cause la panne qu'il sert à diagnostiquer est
///   la pire forme possible. Le `<Show>` est donc rendu **identique des deux
///   côtés** — `voulu()` lit la même adresse ici et là — et seul le dessin,
///   qui vit dans un effet, appartient au navigateur. Les effets ne
///   s'exécutent pas sur le serveur : il n'y a rien à garder de plus.
/// - **une étiquette visible.** Le §5 le dit d'un banc précédent qui a coûté
///   dix minutes de vidéo d'un défaut inexistant : *« un outil qui imite le
///   produit doit porter sa marque »*. Celui-ci écrit ce qu'il est ;
/// - **il ne remplace rien.** La page reste entière dessous ; la carte s'ajoute
///   en tête. Un banc qui se substitue à la page mesure le banc.
// Les deux props ne servent qu'au navigateur : côté serveur, le dessin n'a pas
// lieu. Le `cfg_attr` vaut mieux qu'un préfixe `_`, qui les rendrait muettes
// des deux côtés et ferait perdre le nom au lecteur — c'est l'idiome que
// `selection_de_versets` emploie déjà.
#[cfg_attr(not(feature = "hydrate"), allow(unused_variables))]
#[cfg(debug_assertions)]
#[component]
pub fn BancDeLaCarte(
    #[prop(into)] texte: Signal<String>,
    #[prop(into)] renvoi: Signal<String>,
) -> impl IntoView {
    use leptos_router::hooks::use_query_map;

    let demande = use_query_map();
    let voulu = move || demande.read().get("carte").is_some();
    let hote = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        if !voulu() {
            return;
        }
        let Some(_hote) = hote.get() else { return };
        #[cfg(feature = "hydrate")]
        {
            let Some(toile) = composer(&texte.get(), &renvoi.get()) else {
                return;
            };
            // La toile fait 1080 ; on la montre à la largeur disponible.
            let _ = toile.set_attribute("style", "width:100%;height:auto;display:block");
            _hote.set_inner_html("");
            let _ = _hote.append_child(&toile);
        }
    });

    view! {
        <Show when=voulu>
            <div class="mb-8 rounded-bloc border border-accentuation/50 p-2">
                <p class="m-0 mb-2 text-sm uppercase tracking-capitales text-accentuation">
                    "Banc — la carte de partage, telle qu'elle sortirait"
                </p>
                <div node_ref=hote></div>
            </div>
        </Show>
    }
}
