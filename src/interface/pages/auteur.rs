use leptos::prelude::*;

use crate::interface::design::{Bandeau, Exergue, Filet, Portrait, Principe, Section};
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
#[component]
pub fn Auteur() -> impl IntoView {
    view! {
        <Tete
            titre="L'auteur"
            description="Gloire Bikouta — l'origine de l'ONT, ce qu'il restitue, et comment \
                         il travaille."
            chemin="/fr/l-auteur"
        />

        <Section>
            <h1>"L'auteur"</h1>
            <Portrait
                source="/images/portrait-640.png"
                source_large="/images/portrait-1024.png"
                texte="Gloire Bikouta"
            />

            <Principe>"Gloire Bikouta."</Principe>
            <p>
                "Je ne suis pas un chercheur. Je n'ai pas de chaire. "
                "Je ne suis pas un juif du Second Temple."
            </p>
        </Section>

        // Le récit d'origine est posé sur l'aubergine : c'est le seul endroit
        // du site où l'auteur parle de ce qu'il a reçu, et la page doit
        // changer de registre à cet endroit-là.
        <Bandeau>
            <h2 class="text-or">"Ce que j'ai reçu"</h2>
            <p class="text-lg leading-relaxed text-pretty">
                "J'ai reçu un manteau d'antiquité. Et l'ordre qui allait avec : "
                "ramener ce qui a été perdu depuis d'anciens temps."
            </p>
            <Exergue>
                "Les choses dont le monde n'a même pas conscience qu'il ne sait pas."
            </Exergue>
            <p class="text-lg leading-relaxed text-pretty">
                "J'ai cru que cela ne devait pas être écrit. Je l'ai gardé deux ans. "
                "Puis je l'ai écrit. Et j'ai commencé à traduire."
            </p>
        </Bandeau>

        <Section>
            <h2>"Ce que je fais"</h2>
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

            <Filet orne=true />

            <h2>"Comment je travaille"</h2>
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

            <h2>"Mon critère"</h2>
            <p>
                "L'ontologie hébraïque antique fonctionnelle. "
                "Pas le canon rabbinique. Pas la tradition ecclésiastique. Pas l'académie."
            </p>
            <p>
                "Si une réalité fonctionne d'une manière distincte, elle mérite un nom "
                "distinct. C'est le principe de qara : nommer, c'est faire entrer dans "
                "l'existence."
            </p>
        </Section>
    }
}
