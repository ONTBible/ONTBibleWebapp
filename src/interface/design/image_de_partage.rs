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
    use wasm_bindgen::{JsCast, JsValue};

    const COTE: f64 = 1080.0;
    const MARGE: f64 = 90.0;

    let Some(document) = web_sys::window().and_then(|f| f.document()) else {
        return false;
    };
    let Ok(toile) = document.create_element("canvas") else {
        return false;
    };
    let Ok(toile) = toile.dyn_into::<web_sys::HtmlCanvasElement>() else {
        return false;
    };
    toile.set_width(COTE as u32);
    toile.set_height(COTE as u32);
    let Ok(Some(ctx)) = toile.get_context("2d") else {
        return false;
    };
    let Ok(ctx) = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>() else {
        return false;
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

    ctx.set_fill_style(&JsValue::from_str(&fond));
    ctx.fill_rect(0.0, 0.0, COTE, COTE);

    let corps = palier(texte.chars().count());
    let interligne = corps * 1.42;

    ctx.set_fill_style(&JsValue::from_str(&encre));
    ctx.set_font(&format!("{corps}px Literata, Georgia, serif"));
    ctx.set_text_baseline("alphabetic");

    // Le pliage à la main : `fillText` ne va pas à la ligne.
    let largeur = COTE - 2.0 * MARGE;
    let mut lignes: Vec<String> = Vec::new();
    let mut courante = String::new();
    for mot in texte.split_whitespace() {
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
    ctx.set_fill_style(&JsValue::from_str(&or));
    ctx.fill_rect(MARGE, pied, largeur, 3.0);

    let base = pied + 34.0 + 40.0;
    ctx.set_font("40px Literata, Georgia, serif");
    let _ = ctx.fill_text(renvoi, MARGE, base);

    ctx.set_fill_style(&JsValue::from_str(&encre));
    ctx.set_global_alpha(0.55);
    ctx.set_font("38px Jost, system-ui, sans-serif");
    ctx.set_text_align("right");
    let _ = ctx.fill_text("La Bible ONT", COTE - MARGE, base);

    // Le téléchargement : une ancre `download` sur l'URL de données.
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
    lien.set_download(&format!("{}.png", renvoi.replace([' ', ':'], "-")));
    lien.click();
    true
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
