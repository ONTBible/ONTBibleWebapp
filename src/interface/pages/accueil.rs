use leptos::prelude::*;

use crate::api::verset_du_jour;
use crate::interface::design::{
    image, traverser, Bloc, Bouton, CarteVersetDuJour, Chiffres, Comparaison, Hero, LegendeNiveaux,
    Porte, Portrait, TitreDeSection,
};
use crate::interface::echantillon::{bereshit_1_1, SEGOND_1910, SEGOND_SOURCE};
use crate::interface::tete::Tete;

/// L'accueil.
///
/// ## Le parti pris
///
/// Trois directions lui ont été soumises — une inscription monumentale, une
/// édition critique, un sanctuaire. Il a choisi la troisième, sur les trois
/// points qui la définissent : l'ouverture est un **lieu** où l'on entre, la
/// comparaison oppose un plan en retrait à un plan éclairé, et la profondeur
/// est partout plutôt que réservée à quelques moments.
///
/// L'ordre des sections découle de ce choix. On entre par le verset du jour —
/// ce qu'on est venu chercher — et l'explication vient après, quand on a une
/// raison de la lire. La version précédente ouvrait par la démonstration, ce
/// qui suppose un lecteur déjà convaincu qu'il y a quelque chose à démontrer.
#[component]
pub fn Accueil() -> impl IntoView {
    // `new_blocking` : le serveur attend la valeur et la pose dans le HTML.
    // Sans ça, la carte apparaîtrait après coup — absente pour un moteur de
    // recherche comme pour un lecteur sans JavaScript.
    let verset = Resource::new_blocking(|| (), |_| async { verset_du_jour().await });

    view! {
        <Tete
            titre="La Bible ONT"
            description="Une restitution française du corpus hébreu et araméen antique, \
                         fondée sur l'ontologie hébraïque fonctionnelle. Le cosmos hébreu \
                         n'est pas une usine : c'est un Temple."
            chemin="/fr"
        />

        <Hero>
            <p class="text-sm uppercase tracking-capitales text-accent">"Restitution"</p>
            <h1 class="text-balance">"Le cosmos hébreu n'est pas une usine"</h1>
            <p class="font-titre text-2xl text-accent">"C'est un Temple."</p>
            <Bouton
                href="#aujourd-hui"
                principal=true
                au_clic=Callback::new(traverser("aujourd-hui"))
            >
                "Entrer"
            </Bouton>
        </Hero>

        // ── Le verset du jour, derrière le seuil ──────────────────────────
        //
        // Premier, et c'est délibéré : c'est ce qu'on vient chercher. Le
        // raisonnement suit, pour qui veut savoir d'où il sort.
        //
        // La porte l'**enveloppe** au lieu de le précéder. Le premier montage
        // la posait avant, avec un décor à elle : on ouvrait sur une image,
        // puis on arrivait sur le verset. La porte ne donnait sur rien.
        //
        // Elle ne le cache pas pour autant — il est ici, dans le HTML du
        // serveur, à sa place dans le document. Sans la porte, il est
        // simplement là.
        <Porte id="aujourd-hui">
        <Bloc eclaire=true>
            <TitreDeSection numero="I" titre="Aujourd'hui" />

            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    // Une panne du verset du jour ne doit pas emporter la page :
                    // elle se tait, et le reste tient.
                    match verset.await {
                        Ok(Some(v)) => view! { <CarteVersetDuJour verset=v /> }.into_any(),
                        _ => ().into_any(),
                    }
                })}
            </Suspense>

            <p class="mt-10 text-sm text-encre-douce">
                "Une fonction de la date, non un tirage : le site, l'application et son widget "
                "tombent sur le même verset le même matin, sans jamais se parler."
            </p>
        </Bloc>
        </Porte>

        // ── La démonstration ──────────────────────────────────────────────
        <Bloc large=true>
            <TitreDeSection numero="II" titre="Le même verset, deux mondes" />

            // La phrase commence par une lettre, et ce n'est pas un hasard :
            // `::first-letter` embarque la ponctuation qui précède, donc un
            // guillemet ouvrant se retrouverait dans la lettrine.
            <p class="lettrine">
                "Créer suppose un atelier, de la matière, un avant et un après. "
                "Rien de tout cela n'est dans le verbe hébreu."
            </p>

            <Comparaison
                renvoi="Bereshit 1:1"
                classique=SEGOND_1910
                source=SEGOND_SOURCE
                ont=bereshit_1_1()
            />

            <p class="mt-10">
                <i>"Bara"</i> " n'est pas un acte d'artisan. C'est un acte de roi : inaugurer "
                "un espace, attribuer des rôles, mettre en fonction. Le cosmos ne sort pas "
                "d'une usine — il est inauguré comme on inaugure un Temple."
            </p>
        </Bloc>

        // ── La clé de lecture ─────────────────────────────────────────────
        <Bloc>
            <TitreDeSection numero="III" titre="Trois niveaux, jamais confondus" />
            <p>
                "Une restitution ne peut pas tout dire dans la même ligne. L'ONT sépare ce "
                "que l'hébreu dit, ce qu'il portait implicitement pour son lecteur, et ce "
                "qu'il dit littéralement."
            </p>
            <LegendeNiveaux />
            <p class="mt-10">
                <Bouton href="/fr/lexique">"Les intraduisibles"</Bouton>
            </p>
        </Bloc>

        // ── L'état du chantier ────────────────────────────────────────────
        <Bloc eclaire=true large=true>
            <TitreDeSection numero="IV" titre="Où en est la restitution" />
            <p>
                "Trois livres sur soixante-dix. Le compte est public, et il est tenu par le "
                "pipeline lui-même — ces chiffres viennent du corpus, ils ne sont pas "
                "recopiés à la main."
            </p>
            <Chiffres />
            <p class="mt-10 text-sm text-encre-douce">
                "Une unité verrouillée a été relue et validée : elle fait référence. "
                "Une unité qui ne l'est pas est un brouillon, et le dit."
            </p>
            <p class="mt-8">
                <Bouton href="/fr/lire" principal=true>"Entrer dans le corpus"</Bouton>
            </p>
        </Bloc>

        // ── Qui écrit ─────────────────────────────────────────────────────
        <Bloc large=true>
            <TitreDeSection numero="V" titre="Qui traduit" />
            <div class="md:grid md:grid-cols-[auto_1fr] md:items-end md:gap-10">
                <div class="mx-auto w-52 md:mx-0 md:w-64 lg:-ml-32 lg:w-80">
                    <Portrait
                        source=image("portrait-640.webp")
                        source_large=image("portrait-1024.webp")
                        texte="Gloire Bikouta"
                        largeur_rendue="(min-width: 64rem) 20rem, (min-width: 48rem) 16rem, 13rem"
                    />
                </div>
                <div class="mt-8 md:mt-0 md:pb-4">
                    <p class="font-titre text-xl text-encre-vive">"Gloire Bikouta"</p>
                    <p class="mt-4">"Ni chercheur, ni chaire, ni juif du Second Temple."</p>
                    <p>
                        "Il ne fait pas une traduction de plus. Il restitue — et il laisse "
                        "debout ce qui ne se traduit pas."
                    </p>
                    // Le lien vers la page de l'auteur reviendra avec elle.
                </div>
            </div>
        </Bloc>

        // ── Pour aller plus loin ──────────────────────────────────────────
        <Bloc>
            <div class="flex flex-col items-center gap-8 text-center">
                <p class="text-lg text-balance">
                    "L'ONT affirme, il ne polémique pas. Ce qu'elle refuse tient en cinq lignes."
                </p>
                <div class="flex flex-wrap justify-center gap-4">
                    <Bouton href="/fr/le-pourquoi">"Le pourquoi"</Bouton>
                    <Bouton href="/fr/ce-que-l-ont-n-est-pas">"Ce que l'ONT n'est pas"</Bouton>
                </div>
            </div>
        </Bloc>
    }
}
