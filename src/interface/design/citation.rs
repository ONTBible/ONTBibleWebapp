use leptos::prelude::*;

use crate::domaine::texte::Verset as VersetDomaine;
use crate::interface::design::{Lien, Verset};

/// Un ou plusieurs versets tirés du corpus, cités dans une page de prose.
///
/// C'est la différence entre affirmer et montrer. Une page qui explique
/// l'ontologie fonctionnelle sans jamais poser le texte demande qu'on la
/// croie ; celle qui le pose laisse vérifier.
///
/// ## Le renvoi est un lien, et il le faut
///
/// Un verset cité hors de son chapitre est un verset dont on ne peut pas
/// contrôler le voisinage — c'est-à-dire exactement ce que l'ONT reproche à la
/// citation d'autorité. Le renvoi mène donc au chapitre entier, où le lecteur
/// trouve le contexte **et** le statut de l'unité : verrouillée, ou brouillon
/// qui le dit.
///
/// ## Elle déborde la mesure, elle ne l'élargit pas
///
/// Un verset de l'ONT porte ses trois niveaux imbriqués — glose et niveau 3
/// s'insèrent dans la phrase. Il lui faut donc plus de largeur qu'à une ligne
/// de prose : dans la mesure de 38 rem, moins le retrait de la carte, le corps
/// à 21 px se césure à chaque ligne — « fonctionnelle-ment », « primor-diales ».
///
/// Le réflexe est de passer le bloc en `large`. Il ne faut pas : à 52 rem, la
/// **prose** de la section monte à près de 79 signes par ligne — au-delà de la
/// fourchette confortable — et c'est elle qui se met alors à se césurer. On
/// déplace le défaut au lieu de le corriger.
///
/// La citation déborde donc de la colonne des deux côtés, à partir de `lg`, et
/// la prose garde exactement sa mesure. C'est le patron du portrait de
/// l'accueil : la mesure borne le **texte courant**, elle n'a aucune raison de
/// borner ce qui n'en est pas. En dessous de `lg`, il n'y a plus de marge à
/// occuper — elle rentre dans la colonne.
///
/// ## Le texte vient du corpus, jamais d'ici
///
/// Les versets sont lus dans `../ONTBibleApp/dist/` à la compilation, comme
/// tout le reste du site. Recopier une citation dans une page la figerait au
/// jour où on l'a écrite : le vault corrigerait un verset et la page de fond
/// continuerait d'en montrer l'ancienne forme, sans que rien ne le signale.
#[component]
pub fn Citation(
    /// Le renvoi tel qu'on le cite — « Bereshit 1:2 ».
    #[prop(into)]
    renvoi: String,
    /// Le chemin du chapitre dans la liseuse.
    #[prop(into)]
    chemin: String,
    /// Les versets, dans l'ordre où on les cite.
    versets: Vec<VersetDomaine>,
) -> impl IntoView {
    view! {
        <figure class="halo m-0 my-12 rounded-carte border border-or/25 bg-surface-haute px-7 py-8 lg:-mx-16">
            {versets
                .into_iter()
                .map(|v| view! { <Verset verset=v /> })
                .collect_view()}

            <figcaption class="mt-6 text-sm uppercase tracking-capitales text-encre-douce">
                <Lien href=chemin>{renvoi}</Lien>
            </figcaption>
        </figure>
    }
}
