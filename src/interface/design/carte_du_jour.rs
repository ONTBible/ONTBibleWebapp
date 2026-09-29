use leptos::prelude::*;
use leptos_router::components::A;

use crate::api::VersetDuJourDto;

/// La carte du verset du jour **de l'app** — `ONTDailyCard` dans sa
/// `BurgundyCard`.
///
/// ## Pourquoi il y en a deux sur ce site
///
/// `CarteVersetDuJour` reste, et elle a sa raison : c'est la pièce de
/// l'**accueil**, une page d'édition où le verset est un moment — surface
/// éclairée, halo, massif dans la marge. Elle a été dessinée pour être vue par
/// quelqu'un qui découvre l'ONT.
///
/// Celle-ci est la même chose dans l'autre registre : l'écran d'un lecteur qui
/// revient. L'app la peint en **aplat d'aubergine, tout en or**, et c'est la
/// même surface que partage son widget d'écran d'accueil — ce qui garantit que
/// les deux se ressemblent.
///
/// Les mettre en commun aurait été une fausse économie : elles ne disent pas la
/// même chose au même lecteur.
///
/// ## Le corps seul, et c'est la règle du partage
///
/// `composeBare` éteint les gloses et le niveau 3 avant de composer. C'est le
/// même arbitrage que l'image de partage, et la même raison : *l'appareil
/// critique appartient à la liseuse, où il est consultable et attribué.*
///
/// Ici le site n'a pas à l'éteindre — `api::verset_du_jour` rend déjà le texte
/// nu.
///
/// ## Toute la carte se touche
///
/// L'app en fait un `Button` qui ouvre le passage, avec `.ontPresse`. Pas un
/// lien posé dessous : ce qu'on veut toucher est le verset, et un verset n'a
/// pas de bord.
#[component]
pub fn CarteDuJour(verset: VersetDuJourDto) -> impl IntoView {
    let texte = crate::interface::design::verset::composer(&verset.texte);
    let partage = format!("{} — {}", verset.renvoi, texte);
    let renvoi = verset.renvoi.clone();

    view! {
        <article class="presse survol survol--souleve rounded-carte bg-aubergine px-6 py-6 text-or">
            <A href=verset.chemin.clone() attr:class="block no-underline">
                <p class="m-0 mb-[18px] flex items-center gap-[7px] text-[0.81em] font-semibold uppercase tracking-[0.08em] text-or">
                    // La montagne de la marque, à la taille de l'en-tête —
                    // `ONTMountain(height: 21)` chez l'app.
                    <span aria-hidden="true" class="massif w-[21px] shrink-0"></span>
                    "Verset du jour"
                </p>

                // **En or sur l'aubergine**, comme là-bas : `ink: ONTColors.gold`
                // passé à `composeBare`. La carte entière est une seule encre,
                // ce qui est exactement ce que dit un aplat de marque.
                <p class="m-0 mb-[18px] font-corps text-[1.05em] leading-normal text-or">
                    {texte.clone()}
                </p>

                <p class="m-0 text-[0.93em] font-semibold text-or">{verset.renvoi}</p>
            </A>

            // **La pastille de partage est hors du lien**, et il le faut : un
            // bouton dans un lien est une imbrication invalide, et le
            // navigateur referme le premier en rencontrant le second. C'est la
            // famille du `<p>` dans un `<p>` qui a tué l'hydratation.
            <BoutonDePartage texte=partage renvoi=renvoi />
        </article>
    }
}

/// La pastille « Partager » de la carte.
///
/// Or sur or à 18 % — la seule forme que l'app donne à une action **dans** un
/// aplat de marque. Un bouton plein y ferait un second aplat ; un bouton cerclé
/// y dessinerait un cadre dans un cadre.
///
/// Elle emploie le partage du système quand il existe, et retombe sur le
/// presse-papier sinon — c'est déjà ce que fait la barre de sélection, et la
/// règle est la même : un geste sans retour se répète.
// `texte` et `renvoi` ne servent qu'au navigateur : côté serveur, le clic
// n'existe pas. Le `cfg_attr` plutôt qu'un préfixe `_`, qui les rendrait
// muettes des deux côtés et ferait perdre le nom au lecteur.
#[cfg_attr(not(feature = "hydrate"), allow(unused_variables))]
#[component]
fn BoutonDePartage(#[prop(into)] texte: String, #[prop(into)] renvoi: String) -> impl IntoView {
    let etat = RwSignal::new("Partager");
    let _ = &renvoi;

    view! {
        <button
            type="button"
            class="presse mt-[18px] rounded-full bg-or/18 px-4 py-2 text-[0.88em] font-semibold text-or"
            on:click=move |_| {
                #[cfg(feature = "hydrate")]
                {
                    etat.set(if partager(&texte) { "Copié" } else { "Indisponible" });
                    set_timeout(
                        move || etat.set("Partager"),
                        std::time::Duration::from_secs(2),
                    );
                }
            }
        >
            {move || etat.get()}
        </button>
    }
}

/// Le partage du système, ou le presse-papier.
#[cfg(feature = "hydrate")]
fn partager(texte: &str) -> bool {
    use wasm_bindgen::{JsCast, JsValue};

    let Some(fenetre) = web_sys::window() else {
        return false;
    };
    let navigateur = fenetre.navigator();

    // `navigator.share` n'est pas déclaré par `web-sys` sur toutes les cibles :
    // on le cherche sur l'objet plutôt que de le supposer présent. Une méthode
    // absente à la compilation est une erreur ; absente à l'exécution, c'est un
    // repli — et c'est ce qu'on veut.
    if let Ok(share) = js_sys::Reflect::get(&navigateur, &JsValue::from_str("share")) {
        if share.is_function() {
            let charge = js_sys::Object::new();
            let _ = js_sys::Reflect::set(
                &charge,
                &JsValue::from_str("text"),
                &JsValue::from_str(texte),
            );
            if let Ok(f) = share.dyn_into::<js_sys::Function>() {
                if f.call1(&navigateur, &charge).is_ok() {
                    return true;
                }
            }
        }
    }

    let _ = navigateur.clipboard().write_text(texte);
    true
}
