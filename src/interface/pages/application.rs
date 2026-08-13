use leptos::prelude::*;

use crate::interface::design::{Bloc, Hero, TitreDeSection};
use crate::interface::tete::Tete;

/// `/fr/l-app` — la page qui fait installer l'application.
///
/// ## Ce qu'elle doit faire, et dans cet ordre
///
/// Montrer, puis donner le chemin. Une capture d'écran dit en un regard ce
/// qu'un paragraphe met dix lignes à décrire — et celle qu'on montre porte les
/// trois niveaux du texte, c'est-à-dire l'argument entier du projet.
///
/// ## Le QR n'est pas un ornement
///
/// Cette page se lit surtout sur un **grand écran** : c'est là qu'on tombe sur
/// le site. Or l'app s'installe sur un téléphone. Un badge App Store cliqué
/// depuis un ordinateur ouvre une page web — il ne pose rien sur l'appareil qui
/// compte.
///
/// Le QR fait le pont : on sort son téléphone, on vise, c'est installé. Il
/// disparaît en dessous de `sm`, où il n'aurait aucun sens — on ne scanne pas
/// l'écran qu'on tient.
///
/// ## Le badge est l'artwork d'Apple, tel quel
///
/// `app-store-fr.svg` vient de `tools.applemediaservices.com`, en français et
/// en blanc. Les directives marketing d'Apple l'exigent **non modifié** : ni
/// recoloré, ni redessiné, ni retitré. La variante blanche est celle qu'elles
/// prescrivent sur un fond sombre, et la hauteur minimale est respectée.
///
/// C'est pour ça qu'il n'est pas rendu par `Bouton` comme le reste du site : ce
/// n'est pas un bouton, c'est une marque déposée qu'on nous prête.
#[component]
pub fn Application() -> impl IntoView {
    view! {
        <Tete
            titre="L'application"
            description="La Bible ONT sur iPhone et iPad — le corpus hors ligne, les trois \
                         niveaux du texte, le verset du jour. Gratuite, sans publicité."
            chemin="/fr/l-app"
        />

        <Hero sobre=true>
            <p class="text-sm uppercase tracking-capitales text-accent">"Sur votre appareil"</p>
            <h1 class="text-balance">"L'application"</h1>
            <p class="max-w-xl text-encre-douce text-balance">
                "Le corpus dans la poche, hors ligne, avec ses trois niveaux de lecture. "
                "Gratuite, sans publicité, sans achat."
            </p>
        </Hero>

        <Bloc large=true id="installer">
            // Le propos à gauche, l'appareil à droite.
            //
            // C'est aussi l'ordre du balisage, et les deux coïncident : on lit
            // ce dont il s'agit, puis on voit à quoi ça ressemble. L'inverse
            // ferait tomber un lecteur d'écran sur une image avant d'avoir la
            // moindre phrase pour la situer.
            <div class="flex flex-col items-center gap-14 lg:flex-row lg:items-center lg:gap-20">
                <div class="flex flex-col items-center gap-7 text-center lg:items-start lg:text-left">
                    // L'icône et le nom, côte à côte, comme sur la fiche de
                    // l'App Store. C'est ce qu'on reconnaîtra sur l'écran
                    // d'accueil — le montrer ici, c'est promettre exactement ce
                    // qu'on installera.
                    <div class="flex items-center gap-4 sm:gap-5">
                        <img
                            src="/images/touch-icon.png"
                            alt=""
                            aria-hidden="true"
                            width="180"
                            height="180"
                            // `22%` approche le squircle d'iOS d'assez près
                            // pour qu'on lise une icône d'app, et pas une
                            // vignette aux coins arrondis.
                            class="size-16 shrink-0 rounded-[22%] shadow-lg shadow-nuit/60 sm:size-20"
                        />
                        <h2 class="m-0 text-balance">"La Bible ONT"</h2>
                    </div>

                    // La phrase qui lève l'objection avant qu'elle vienne. Sur
                    // une app de texte religieux, « gratuite » ne suffit pas :
                    // c'est « sans achat » qui rassure, parce que c'est là que
                    // les autres attendent le lecteur.
                    <p class="text-lg text-encre">"Gratuite. Sans publicité, sans achat."</p>

                    <p class="text-[0.95em] text-encre-douce text-pretty">
                        "Les intraduisibles en or, les gloses qui éclairent, la "
                        "translittération et l'hébreu — chacun se coupe d'un geste, si l'on "
                        "veut le texte nu."
                    </p>

                    <Installer />
                </div>

                <Ecran />
            </div>
        </Bloc>

        <Bloc eclaire=true>
            <TitreDeSection numero="I" titre="Ce que l'app fait, et que le site ne fait pas" />

            <dl class="m-0 grid gap-8 sm:grid-cols-2">
                <Atout titre="Hors ligne">
                    "Le corpus est dans l'app. Un tunnel, un avion, un village sans réseau : "
                    "le texte est là."
                </Atout>
                <Atout titre="Le verset du jour">
                    "Sur l'écran d'accueil, en widget, et en notification si vous la voulez. "
                    "Le même verset que ce site, le même jour."
                </Atout>
                <Atout titre="Vos notes">
                    "Surlignages et notes sur les versets, gardés sur l'appareil. Un compte, "
                    "facultatif, les porte d'un appareil à l'autre."
                </Atout>
                <Atout titre="Le texte se corrige tout seul">
                    "Une correction de traduction arrive en quelques minutes, sans passer "
                    "par une mise à jour de l'App Store."
                </Atout>
            </dl>
        </Bloc>

        <Bloc>
            <TitreDeSection numero="II" titre="Android" />
            <p class="text-encre-douce text-pretty">
                "Il n'y a pas de version Android, et il serait malhonnête d'écrire "
                "« bientôt » sans date. Ce site, lui, se lit sur n'importe quel téléphone : "
                <a href="/fr/lire">"le corpus entier y est"</a>", avec les mêmes trois niveaux "
                "et les mêmes réglages de lecture."
            </p>
        </Bloc>
    }
}

/// Un argument, en deux lignes.
///
/// Un `<dl>` et non une suite de `<div>` : c'est une liste de termes et de
/// définitions, et le dire dans le balisage donne gratuitement la bonne
/// structure à un lecteur d'écran.
#[component]
fn Atout(titre: &'static str, children: Children) -> impl IntoView {
    view! {
        <div>
            <dt class="mb-2 text-sm uppercase tracking-capitales text-accent">{titre}</dt>
            <dd class="m-0 text-[0.95em] text-encre-douce text-pretty">{children()}</dd>
        </div>
    }
}

/// La capture, dans un châssis d'appareil.
///
/// ## Pourquoi un cadre ici, alors que le portrait n'en a pas
///
/// Le §5 pose qu'une image du site ne s'enferme pas dans un rectangle : c'est
/// vrai d'une photographie, dont le cadre se voit plus que le sujet.
///
/// Une capture d'écran est le cas contraire. Posée à plat, elle se lit comme un
/// fichier — et le premier essai le montrait : une plaque claire, sans bord,
/// portant sa propre barre d'état à côté de celle du téléphone qui la regarde.
/// Le châssis dit ce que c'est. Il n'enferme pas le sujet, il le nomme.
///
/// Il reste mince : un filet d'or à 15 %, la nuit pour la coque, et le rayon
/// intérieur ajusté à l'extérieur moins l'épaisseur — sans quoi le coin de
/// l'écran ne suit pas celui de la coque et l'objet cesse d'être un objet.
///
/// La lueur d'aubergine, elle, reste : c'est le patron du portrait, et elle
/// évite que le châssis flotte sur du vide.
#[component]
fn Ecran() -> impl IntoView {
    view! {
        <div class="relative shrink-0">
            <span
                aria-hidden="true"
                class="pointer-events-none absolute inset-0 -z-10 scale-125 rounded-full bg-aubergine/45 blur-3xl"
            ></span>

            <div class="rounded-[2.4rem] border border-or/15 bg-nuit p-2 shadow-2xl shadow-nuit/70">
                <img
                    src="/images/app-lecture.webp"
                    alt="Bereshit 3 dans l'application : le texte, ses gloses en retrait, \
                         les intraduisibles en or et l'hébreu vocalisé."
                    width="880"
                    height="1912"
                    // `loading=eager` : elle est le sujet du bloc, et la charger
                    // paresseusement la ferait apparaître après coup.
                    loading="eager"
                    class="block h-auto w-56 rounded-[1.9rem] sm:w-64 lg:w-72"
                />
            </div>
        </div>
    }
}

/// Le badge, le QR, et la vérité sur l'état de la fiche.
///
/// ## Tant que l'app n'est pas publiée, on ne fait pas semblant
///
/// La fiche existe dans App Store Connect, mais l'adresse publique ne répond
/// qu'une fois la relecture d'Apple passée. Un badge qui mène à une page
/// d'erreur est pire qu'une absence : on l'essaie, et le projet a l'air cassé.
///
/// `PUBLIEE` bascule le jour de l'approbation, et c'est la seule chose à
/// changer.
#[component]
fn Installer() -> impl IntoView {
    if !PUBLIEE {
        return view! {
            <p class="rounded-2xl border border-or/30 px-6 py-4 text-sm uppercase tracking-capitales text-accent">
                "En relecture chez Apple"
            </p>
        }
        .into_any();
    }

    view! {
        <div class="flex flex-col items-center gap-8 sm:flex-row sm:items-center lg:items-start">
            <a
                href=FICHE
                // Le badge sort du site : `noopener` coupe l'accès de la page
                // ouverte à celle-ci, et c'est la valeur par défaut qu'on ne
                // veut pas laisser au hasard.
                rel="noopener"
                class="block no-underline"
            >
                <img
                    src="/images/app-store-fr.svg"
                    alt="Télécharger dans l'App Store"
                    width="127"
                    height="40"
                    // La hauteur minimale du badge est fixée par les directives
                    // d'Apple. 40 px la respecte avec de la marge.
                    class="block h-11 w-auto"
                />
            </a>

            // Inutile sur un téléphone : on ne scanne pas l'écran qu'on tient.
            <div class="hidden flex-col items-center gap-2 sm:flex">
                <span class="rounded-xl bg-encre-vive p-2.5 text-nuit">
                    <img
                        src="/images/qr-app.svg"
                        alt=""
                        aria-hidden="true"
                        width="37"
                        height="37"
                        class="block size-20"
                    />
                </span>
                <span class="text-[0.7rem] uppercase tracking-capitales text-encre-douce">
                    "Visez avec l'appareil photo"
                </span>
            </div>
        </div>
    }
    .into_any()
}

/// Faux tant qu'Apple n'a pas approuvé la version 1.0.
///
/// Soumise le 13 août 2026. À passer à `true` le jour de l'approbation — c'est
/// tout ce qu'il y a à faire, le badge et le QR apparaissent ensemble.
const PUBLIEE: bool = false;

/// La fiche par son identifiant, jamais par un nom lisible : Apple recompose
/// ceux-là quand le titre change, l'identifiant ne bouge pas. Le même que celui
/// du QR — voir `scripts/qr-app.py`.
const FICHE: &str = "https://apps.apple.com/fr/app/id6801192372";
