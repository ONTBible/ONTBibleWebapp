use leptos::prelude::*;

use crate::api::verset_du_jour;
use crate::interface::design::{
    Bloc, Bouton, CarteVersetDuJour, Chiffres, Comparaison, Hero, LegendeNiveaux, Portrait,
    TitreDeSection,
};
use crate::interface::echantillon::{bereshit_1_1, SEGOND_1910, SEGOND_SOURCE};
use crate::interface::tete::Tete;

/// L'accueil.
///
/// ## Pourquoi une page longue et non un sommaire
///
/// La première version était trois liens vers trois pages maigres. Un sommaire
/// demande au lecteur de choisir avant de savoir de quoi il s'agit — et sur un
/// projet dont personne n'a entendu parler, il repart.
///
/// Cette page **montre** au lieu d'annoncer, dans cet ordre : l'affirmation,
/// puis la démonstration sur un verset réel, puis la clé de lecture, puis le
/// verset du jour, puis l'état honnête du chantier, puis qui écrit. Les pages
/// de fond ne sont proposées qu'après — quand on a une raison d'y aller.
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
            <h1 class="text-balance">"Le cosmos hébreu n'est pas une usine."</h1>
            <p class="text-xl leading-snug text-balance text-accent">"C'est un Temple."</p>
            <p class="text-encre text-balance">
                "Une restitution française du corpus hébreu et araméen antique, "
                "fondée sur l'ontologie hébraïque fonctionnelle."
            </p>
            <div class="flex flex-wrap justify-center gap-4">
                <Bouton href="#la-demonstration" principal=true>"Voir un verset"</Bouton>
                <Bouton href="/fr/le-pourquoi">"Le pourquoi"</Bouton>
            </div>
        </Hero>

        // ── La démonstration ──────────────────────────────────────────────
        //
        // C'est le cœur de la page. Six lignes valent mieux qu'un essai : on
        // voit ce que la traduction classique laisse tomber avant de
        // comprendre pourquoi.
        <Bloc id="la-demonstration">
            <TitreDeSection numero="I" titre="Le même verset, deux mondes" />
            // La phrase commence par une lettre, et ce n'est pas un hasard :
            // `::first-letter` embarque la ponctuation qui précède, donc un
            // guillemet ouvrant se retrouverait dans la lettrine — et la
            // ponctuation suspendue le jetterait dans la marge par-dessus.
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

            <p class="mt-8">
                <i>"Bara"</i> " n'est pas un acte d'artisan. C'est un acte de roi : inaugurer "
                "un espace, attribuer des rôles, mettre en fonction. Le cosmos ne sort pas "
                "d'une usine — il est inauguré comme on inaugure un Temple."
            </p>
        </Bloc>

        // ── La clé de lecture ─────────────────────────────────────────────
        <Bloc eclaire=true>
            <TitreDeSection numero="II" titre="Trois niveaux, jamais confondus" />
            <p>
                "Une restitution ne peut pas tout dire dans la même ligne. L'ONT sépare ce "
                "que l'hébreu dit, ce qu'il portait implicitement pour son lecteur, et ce "
                "qu'il dit littéralement."
            </p>
            <LegendeNiveaux />
        </Bloc>

        // ── Le verset du jour ─────────────────────────────────────────────
        <Bloc>
            <TitreDeSection numero="III" titre="Aujourd'hui" />
            <p>
                "Le verset du jour n'est pas tiré au sort : c'est une fonction de la date. "
                "Le site, l'application et son widget tombent sur le même, le même matin, "
                "sans jamais se parler."
            </p>

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
        </Bloc>

        // ── L'état du chantier ────────────────────────────────────────────
        <Bloc eclaire=true>
            <TitreDeSection numero="IV" titre="Où en est la restitution" />
            <p>
                "Trois livres sur soixante-dix. Le compte est public, et il est tenu par le "
                "pipeline lui-même — ces chiffres viennent du corpus, ils ne sont pas "
                "recopiés à la main."
            </p>
            <Chiffres />
            <p class="mt-8 text-sm text-encre-douce">
                "Une unité verrouillée a été relue et validée : elle fait référence. "
                "Une unité qui ne l'est pas est un brouillon, et le dit."
            </p>
        </Bloc>

        // ── Qui écrit ─────────────────────────────────────────────────────
        <Bloc>
            <TitreDeSection numero="V" titre="Qui traduit" />
            // Le portrait sort de la colonne de lecture par la gauche.
            //
            // La mesure borne le **texte** ; elle n'a aucune raison de borner
            // une image, et la marge du site est de toute façon vide. Sur grand
            // écran, le portrait s'y avance et gagne le tiers de taille qui lui
            // manquait, sans que la ligne de texte s'allonge d'un signe.
            //
            // En dessous, il n'y a plus de marge à occuper : il repasse dans la
            // colonne, puis au-dessus du texte sur téléphone.
            <div class="md:grid md:grid-cols-[auto_1fr] md:items-end md:gap-10">
                <div class="mx-auto w-52 md:mx-0 md:w-60 lg:-ml-40 lg:w-80">
                    <Portrait
                        source="/images/portrait-640.webp"
                        source_large="/images/portrait-1024.webp"
                        texte="Gloire Bikouta"
                        largeur_rendue="(min-width: 64rem) 20rem, (min-width: 48rem) 15rem, 13rem"
                    />
                </div>
                <div class="mt-8 md:mt-0 md:pb-4">
                    <p class="font-titre text-xl text-encre-vive">"Gloire Bikouta"</p>
                    <p class="mt-4">
                        "Ni chercheur, ni chaire, ni juif du Second Temple."
                    </p>
                    <p>
                        "Il ne fait pas une traduction de plus. Il restitue — et il laisse "
                        "debout ce qui ne se traduit pas."
                    </p>
                    <Bouton href="/fr/l-auteur">"D'où vient ce travail"</Bouton>
                </div>
            </div>
        </Bloc>

        // ── Pour aller plus loin ──────────────────────────────────────────
        <Bloc eclaire=true large=true>
            <div class="flex flex-col items-center gap-6 text-center">
                <p class="text-lg text-balance">
                    "L'ONT affirme, il ne polémique pas. Ce qu'elle refuse tient en cinq lignes."
                </p>
                <div class="flex flex-wrap justify-center gap-4">
                    <Bouton href="/fr/ce-que-l-ont-n-est-pas">"Ce que l'ONT n'est pas"</Bouton>
                    <Bouton href="https://github.com/ONTBible">"Le corpus et le code"</Bouton>
                </div>
            </div>
        </Bloc>
    }
}
