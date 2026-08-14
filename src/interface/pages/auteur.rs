use leptos::prelude::*;

use crate::interface::design::{image, Bloc, Exergue, Hero, Portrait, TitreDeSection};
use crate::interface::tete::Tete;

/// « L'auteur ».
///
/// ## Premier jet — à relire avant publication
///
/// Cette page est écrite dans le registre de Gloire Bikouta, pas dans celui
/// d'une notice : parataxe, phrases courtes et verbales, aucune atténuation,
/// termes hébreux laissés debout sans glose. C'est déjà sa façon d'écrire en
/// privé ; le site doit sonner comme lui.
///
/// Le récit d'origine est une **version publique** tirée d'une page privée, et
/// non une reprise de celle-ci. La décision de rendre la vision publique est
/// la sienne, prise le 12 août 2026. Le texte, lui, attend sa relecture — rien
/// d'ici ne doit être mis en ligne avant.
///
/// ## L'ouverture est le portrait
///
/// C'est la seule page du site dont l'ouverture porte une image. Ailleurs, un
/// visage dans un lieu détournerait l'attention du lieu ; ici, c'est le sujet.
#[component]
pub fn Auteur() -> impl IntoView {
    view! {
        <Tete
            titre="L'auteur"
            description="Gloire Bikouta — l'origine de l'ONT, ce qu'il restitue, et comment \
                         il travaille."
            chemin="/fr/l-auteur"
        />

        <Hero sobre=true>
            // Pas « L'auteur » : la navigation le dit déjà, et souligné juste
            // au-dessus. Un rappel qui répète le lien d'où l'on vient se lit
            // comme un doublon, pas comme un repère.
            <p class="text-sm uppercase tracking-capitales text-accent">"Qui traduit"</p>
            // Plus petit sur téléphone : à 14 rem, le portrait poussait le nom
            // sous la barre de Safari et l'ouverture ne tenait plus sur un
            // écran. C'est la hauteur disponible qui commande, pas la largeur.
            <div class="w-32 sm:w-56">
                <Portrait
                    source=image("portrait-640.webp")
                    source_large=image("portrait-1024.webp")
                    texte="Gloire Bikouta"
                    largeur_rendue="(min-width: 40rem) 14rem, 8rem"
                />
            </div>
            <h1 class="text-balance">"Gloire Bikouta"</h1>
            <p class="max-w-xl text-encre-douce text-balance">
                "Ni chercheur, ni chaire, ni juif du Second Temple."
            </p>
        </Hero>

        // Le récit d'origine est le moment de la page : c'est le seul endroit
        // du site où l'auteur parle de ce qu'il a reçu, et le registre doit
        // changer là.
        <Bloc eclaire=true>
            <TitreDeSection numero="I" titre="Ce que j'ai reçu" />
            <p class="lettrine">
                "J'ai reçu un manteau d'antiquité. Et l'ordre qui allait avec : "
                "ramener ce qui a été perdu depuis d'anciens temps."
            </p>
            <Exergue>
                "Les choses dont le monde n'a même pas conscience qu'il ne sait pas."
            </Exergue>
            <p>
                "J'ai cru que cela ne devait pas être écrit. Je l'ai gardé deux ans. "
                "Puis je l'ai écrit. Et j'ai commencé à traduire."
            </p>
        </Bloc>

        <Bloc>
            <TitreDeSection numero="II" titre="Ce que je fais" />
            <p>"Je ne fais pas une traduction de plus. Je restitue."</p>
            <p>
                "Le texte hébreu disait quelque chose à ses lecteurs. Ce quelque chose est "
                "encore là. Il est sous des couches — du grec, du latin, des siècles. "
                "Je retire les couches."
            </p>
            <p>
                "Et je laisse debout ce qui ne se traduit pas. "
                "Elohim, ruach, nefesh, kavod, tsedaqah. "
                "Un mot mal traduit vaut moins qu'un mot laissé en hébreu."
            </p>
        </Bloc>

        <Bloc eclaire=true>
            <TitreDeSection numero="III" titre="Comment je travaille" />
            <p>"Je travaille avec Claude. Je ne le cache pas, et je l'assume."</p>
            <p>
                "Je dirige. Je fixe les orientations, je tranche les termes fondateurs, je "
                "verrouille les unités. Claude propose, signale, exécute. "
                "Il ne décide seul sur aucun terme."
            </p>
            <p>
                "Chaque unité verrouillée est passée par moi. Une unité qui ne l'est pas "
                "reste un brouillon, et le site le dit."
            </p>
        </Bloc>

        <Bloc>
            <TitreDeSection numero="IV" titre="Mon critère" />
            <p>
                "L'ontologie hébraïque antique fonctionnelle. "
                "Pas le canon rabbinique. Pas la tradition ecclésiastique. Pas l'académie."
            </p>
            <p>
                "Si une réalité fonctionne d'une manière distincte, elle mérite un nom "
                "distinct. C'est le principe de qara : nommer, c'est faire entrer dans "
                "l'existence."
            </p>
        </Bloc>
    }
}
