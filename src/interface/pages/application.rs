use leptos::prelude::*;

use crate::interface::design::{image, Bloc, Bouton, Hero, Lien, TitreDeSection};
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
///
/// ## Pendant la bêta, la page explique au lieu de supposer
///
/// Un badge App Store ne demande rien à personne : on le touche, l'app
/// s'installe. TestFlight, non — il faut d'abord installer une **autre** app,
/// revenir au lien, accepter, puis installer. Quatre gestes là où on en attend
/// un, et rien à l'écran ne les annonce.
///
/// Le lecteur visé n'est pas développeur, et il a tous les âges. La section
/// [`Beta`] pose donc les gestes dans l'ordre, chacun avec ce qu'on voit à
/// l'écran quand on le fait. C'est la section la plus longue de la page, et
/// c'est justifié : celle qui manque est celle où l'on abandonne.
#[component]
pub fn Application() -> impl IntoView {
    // Les sections se numérotent d'après ce qui est **rendu**. La bêta en fait
    // partie tant qu'elle dure ; son départ ne doit pas laisser une page qui
    // commence à « II ».
    let (atouts, android) = if EN_BETA { ("II", "III") } else { ("I", "II") };

    view! {
        <Tete
            titre="L'application"
            description=if EN_BETA {
                "La Bible ONT sur iPhone — le corpus hors ligne, les trois niveaux du texte, \
                 le verset du jour. En bêta publique sur TestFlight, gratuite et sans publicité."
            } else {
                "La Bible ONT sur iPhone et iPad — le corpus hors ligne, les trois niveaux \
                 du texte, le verset du jour. Gratuite, sans publicité."
            }
            chemin="/fr/l-app"
        />

        <Hero sobre=true>
            <p class="text-sm uppercase tracking-capitales text-accent">
                {if EN_BETA { "Bêta publique" } else { "Sur votre appareil" }}
            </p>
            <h1 class="text-balance">"L'application"</h1>
            <p class="max-w-xl text-encre-douce text-balance">
                "Le corpus dans la poche, hors ligne, avec ses trois niveaux de lecture. "
                "Gratuite, sans publicité, sans achat."
            </p>
            {EN_BETA
                .then(|| {
                    view! {
                        <p class="max-w-xl text-encre-douce text-balance">
                            "Elle n'est pas encore sur l'App Store. Elle s'essaie dès maintenant."
                        </p>
                    }
                })}
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
                            src=image("touch-icon.png")
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

        // `EN_BETA` dit **si**, `BETA` dit **quoi**. La section a besoin des
        // deux, et les prendre ici, d'un seul geste, évite qu'ils se
        // contredisent — une section rendue sans lien, ou un lien sans section.
        {BETA.filter(|_| EN_BETA).map(|lien| view! { <Beta lien /> })}

        <Bloc eclaire=true>
            <TitreDeSection numero=atouts titre="Ce que l'app fait, et que le site ne fait pas" />

            <dl class="m-0 grid gap-8 sm:grid-cols-2">
                <Atout titre="Hors ligne">
                    "Le corpus est dans l'app. Un tunnel, un avion, un village sans réseau : "
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
            <TitreDeSection numero=android titre="Android" />
            <p class="text-encre-douce text-pretty">
                "Il n'y a pas de version Android, et il serait malhonnête d'écrire "
                "« bientôt » sans date. Mais ce site "<strong>"s'installe"</strong>", et le "
                "résultat ressemble de près à une app : une icône sur l'écran d'accueil, "
                "un nom, et l'ouverture en plein écran sans barre de navigateur."
            </p>
            <p class="text-encre-douce text-pretty">
                "Dans Chrome, le menu ⋮ propose "<strong>"« Installer l'application »"</strong>
                " — ou "<strong>"« Ajouter à l'écran d'accueil »"</strong>" selon la version. "
                "Firefox et Samsung Internet ont la même entrée."
            </p>
            <p class="text-encre-douce text-pretty">
                "Ce qui manquera : la lecture hors ligne, le widget et les notifications du "
                "verset du jour. Le reste y est — "<Lien href="/fr/lire">"le corpus entier"</Lien>
                ", les trois niveaux, et les mêmes réglages de lecture."
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
///
/// ## L'île se dessine, elle ne se capture pas
///
/// La Dynamic Island n'est **jamais** dans une capture d'écran. C'est une
/// découpe physique de la dalle, doublée d'une surcouche du système : le
/// framebuffer, lui, ne porte que ce que l'app y met. `simctl io screenshot`
/// n'en montrera donc rien, quel que soit l'appareil.
///
/// Ce qu'on voit dans la capture, c'est son **empreinte** : iOS pousse l'heure
/// à gauche et les indicateurs à droite, laissant un trou central. Mesuré sur
/// `app-lecture.webp`, ce trou fait 47 % de la largeur — la preuve que la
/// capture vient bien d'un appareil à île, et la place où la poser.
///
/// Les proportions sont celles de l'appareil, pas un dessin à l'œil : 126 pt de
/// large sur 37,33, à 11 pt du haut, sur un écran de 402 pt. Rapportées à la
/// largeur de l'image, cela donne 31,4 %, un rapport de 126/37,33 et un retrait
/// de 1,26 %. Elles sont en **pourcentages** et non en pixels : le châssis
/// change trois fois de taille selon l'écran (`w-56`, `sm:w-64`, `lg:w-72`), et
/// une valeur fixe serait juste sur un seul.
///
/// Elle est en `nuit` et non en noir pur. Ce n'est pas une licence : l'île est
/// une portion de dalle éteinte, exactement la même matière que la coque qui
/// l'entoure — et le site n'a pas un seul noir pur, un ici serait le premier.
#[component]
fn Ecran() -> impl IntoView {
    view! {
        <div class="relative shrink-0">
            <span
                aria-hidden="true"
                class="pointer-events-none absolute inset-0 -z-10 scale-125 rounded-full bg-aubergine/45 blur-3xl"
            ></span>

            <div class="rounded-[2.4rem] border border-or/15 bg-nuit p-2 shadow-2xl shadow-nuit/70">
                // Le `relative` est sur un conteneur qui épouse l'image, et non
                // sur la coque : l'île se place en part de l'**écran**, pas du
                // châssis, et les deux diffèrent de l'épaisseur du bord.
                <div class="relative w-56 sm:w-64 lg:w-72">
                    <img
                        src=image("app-lecture.webp")
                        alt="Bereshit 3 dans l'application : le texte, ses gloses en retrait, \
                             les intraduisibles en or et l'hébreu vocalisé."
                        width="880"
                        height="1912"
                        // `loading=eager` : elle est le sujet du bloc, et la charger
                        // paresseusement la ferait apparaître après coup.
                        loading="eager"
                        class="block h-auto w-full rounded-[1.9rem]"
                    />
                    <span
                        aria-hidden="true"
                        class="pointer-events-none absolute top-[1.26%] left-1/2 w-[31.4%] -translate-x-1/2 rounded-full bg-nuit aspect-[126/37.33]"
                    ></span>
                </div>
            </div>
        </div>
    }
}

/// Le badge, le QR, et la vérité sur l'état de la fiche.
///
/// ## Trois états, et jamais de faux-semblant
///
/// La fiche existe dans App Store Connect, mais l'adresse publique ne répond
/// qu'une fois la relecture d'Apple passée. Un badge qui mène à une page
/// d'erreur est pire qu'une absence : on l'essaie, et le projet a l'air cassé.
///
/// Entre les deux vit la **bêta publique** : l'app est installable, par
/// TestFlight, et rien sur l'App Store ne le dit. C'est le seul chemin qui
/// existe alors, donc c'est celui que la page montre — avec sa condition
/// d'appareil posée **avant** le bouton et non après, pour que personne ne
/// découvre qu'il lui faut iOS 18 une fois TestFlight installé.
///
/// Deux constantes gouvernent le tout, et elles se lisent dans cet ordre :
/// `PUBLIEE` l'emporte sur `BETA`. Le jour de l'approbation, basculer la
/// première suffit — la bêta s'efface d'elle-même, ici comme dans le reste de
/// la page.
#[component]
fn Installer() -> impl IntoView {
    if !PUBLIEE {
        let Some(lien) = BETA else {
            return view! {
                <p class="rounded-2xl border border-or/30 px-6 py-4 text-sm uppercase tracking-capitales text-accent">
                    "En relecture chez Apple"
                </p>
            }
            .into_any();
        };

        return view! {
            // Le QR passe **sous** les boutons quand la colonne se resserre.
            //
            // À `sm`, le bloc entier est encore sur une colonne — la capture est
            // dessous — et il reste toute la largeur de la page : le QR tient à
            // côté. À `lg`, la capture passe à droite et cette colonne tombe à la
            // moitié de l'écran ; le QR, qui ne peut pas rétrécir sans que sa
            // légende parte sur quatre lignes, la volait aux boutons. « iPhone,
            // iOS 18 ou plus récent » se coupait alors en « plus RÉ-CENT ».
            //
            // Le seuil est donc l'inverse de l'habituel : on repasse en colonne
            // **en montant**, parce que c'est en montant que la place se perd
            // ici. La verticale, elle, ne manque jamais dans ce bloc.
            <div class="flex flex-col items-center gap-8 sm:flex-row sm:items-center lg:flex-col lg:items-start">
                <div class="flex flex-col items-center gap-3 lg:items-start">
                    // La condition d'appareil est **au-dessus** du bouton.
                    //
                    // En dessous, elle se lit après coup — c'est-à-dire une fois
                    // TestFlight installé, la fiche ouverte, et le refus reçu.
                    <p class="m-0 text-sm uppercase tracking-capitales text-encre-douce">
                        "iPhone, iOS 18 ou plus récent"
                    </p>
                    // Les deux boutons sont **empilés**, jamais côte à côte.
                    //
                    // En capitales espacées, les deux pastilles font ensemble
                    // plus de six cents pixels, et la colonne de gauche n'en
                    // offre pas tant sur un grand écran : elles déborderaient, ou
                    // se couperaient chacune en deux lignes. Un `flex-row` qui ne
                    // tient qu'à certaines largeurs n'est pas une mise en page,
                    // c'est un pari.
                    //
                    // Le `nowrap` est hérité, il n'est pas posé sur les boutons —
                    // ceux-ci ne prennent pas de classe, et c'est bien ainsi : le
                    // design system garde sa forme. La ligne au-dessus et celle
                    // en dessous, elles, doivent pouvoir se replier sur un
                    // téléphone, et le `nowrap` s'arrête donc à ce `div`.
                    <div class="flex flex-col items-center gap-3 whitespace-nowrap lg:items-start">
                        <Bouton href=lien principal=true>"Rejoindre la bêta"</Bouton>
                        // La seconde voie, exactement ce que la forme cerclée est
                        // faite pour dire. Le premier bouton propose déjà
                        // d'installer TestFlight quand il manque — mais il le
                        // propose *après* un aller-retour, sur un écran que le
                        // lecteur n'attendait pas. Le lien direct épargne le
                        // détour à qui sait déjà qu'il ne l'a pas.
                        <Bouton href=TESTFLIGHT>"Installer TestFlight"</Bouton>
                    </div>
                    <p class="m-0 text-sm text-encre-douce">
                        "Deux minutes — "<Lien href="#la-beta">"comment ça marche"</Lien>
                    </p>
                </div>

                <Qr fichier="qr-beta.svg" />
            </div>
        }
        .into_any();
    }

    view! {
        // Le QR repasse **sous** le badge à `lg`, comme il repasse sous les
        // boutons dans l'état bêta — et pour la même raison, qui est une
        // mesure et non un goût.
        //
        // À `lg`, la capture passe à droite et cette colonne tombe à la moitié
        // de la mesure large, soit 376 px. Le badge en fait 203 et le QR 224 :
        // côte à côte avec leur écart, 459. Le QR se faisait donc écraser, et
        // sa légende partait sur deux lignes — le seul symptôme visible d'un
        // dépassement qui, lui, ne se voit pas.
        //
        // Le seuil est l'inverse de l'habituel : on repasse en colonne **en
        // montant**, parce que c'est en montant que la place se perd ici.
        <div class="flex flex-col items-center gap-8 sm:flex-row sm:items-center lg:flex-col lg:items-start">
            <a
                href=FICHE
                // Le badge sort du site : `noopener` coupe l'accès de la page
                // ouverte à celle-ci, et c'est la valeur par défaut qu'on ne
                // veut pas laisser au hasard.
                rel="noopener"
                class="block no-underline"
            >
                <img
                    src=image("app-store-fr.svg")
                    alt="Télécharger dans l'App Store"
                    width="127"
                    height="40"
                    // La hauteur minimale du badge est fixée par les directives
                    // d'Apple — 40 px. Ce n'est pas elle qui décide ici : c'est
                    // le QR d'à côté.
                    //
                    // Ce bloc n'avait jamais été **regardé**, la page vivant
                    // en bêta depuis toujours ; entre-temps le QR est passé de
                    // 80 à 224 px. Le badge, resté à 44, faisait le cinquième
                    // de son voisin — l'action principale cinq fois plus
                    // petite que la seconde voie. À 64 px il fait 203 px de
                    // large contre les 224 du QR : deux objets de même poids,
                    // qui est ce que la page dit d'eux.
                    //
                    // Le badge se met à l'échelle, il ne se redessine pas
                    // (§8 quinquies) — une hauteur, et la largeur suit.
                    class="block h-16 w-auto"
                />
            </a>

            <Qr fichier="qr-app.svg" />
        </div>
    }
    .into_any()
}

/// Le pont entre le grand écran et le téléphone.
///
/// Il ne dépend pas de ce vers quoi il mène — la fiche App Store ou la bêta —
/// et c'est tout l'intérêt : les deux états de la page posent le même objet, au
/// même endroit, avec la même légende. Le lecteur qui revient après la
/// publication ne voit rien changer que la destination.
///
/// Inutile sur un téléphone, où il disparaît : on ne scanne pas l'écran qu'on
/// tient. La pastille claire est posée **ici** et non dans le SVG — un QR se lit
/// sur fond clair, mais le décider dans le fichier l'imposerait à tous les
/// emplacements futurs.
///
/// ## 224 px, et la montagne dedans
///
/// Il en faisait 80, puis 144, et il était encore trop petit — à cette taille
/// il se lit comme une vignette technique posée à côté du propos, pas comme la
/// seconde voie qu'il est. Les pages qui font bien ce geste lui donnent deux
/// cents pixels et plus ; le QR y est un **objet**, de la taille d'un bouton.
///
/// Le bénéfice est double, et le second n'est pas un bonus : à 224 px un module
/// fait 5,5 px au lieu de 3,5, et un lecteur qui hésite est un lecteur qui
/// repart. Le dessin lui-même — points pour les données, carrés francs pour les
/// repères — vient de `scripts/qr-app.py` ; ce qui suit ne fait que le poser.
///
/// La montagne n'est **pas** dans le fichier engendré : elle est ici, en
/// masque, superposée au creux. Trois raisons, et la dernière est la vraie :
/// `logomark.svg` est déjà chargé par la page, le masque suit la couleur du
/// texte comme partout ailleurs, et une marque qu'on inline dans un fichier
/// engendré cesse de suivre l'original le jour où l'original bouge.
///
/// Sa largeur — 16 % de la pastille — vaut un peu moins que le creux, qui en
/// fait 22 % : la marge qui reste est celle qui empêche le dessin de toucher
/// les modules et de se lire comme du bruit.
#[component]
fn Qr(fichier: &'static str) -> impl IntoView {
    view! {
        <div class="hidden flex-col items-center gap-2 sm:flex">
            <span class="relative block rounded-3xl bg-encre-vive p-4 text-nuit">
                <img
                    src=image(fichier)
                    alt=""
                    aria-hidden="true"
                    width="41"
                    height="41"
                    class="block size-56"
                />
                // Centré par **translation**, et non par `inset-0 m-auto`.
                //
                // Les deux marchent ici. Celui-ci ne pose qu'un bord sur deux,
                // donc la hauteur se déduit du rapport d'aspect sans que la
                // boîte soit sur-contrainte — et c'est déjà l'idiome des
                // massifs de `hero.rs`, ce qui vaut mieux qu'un second.
                <span
                    aria-hidden="true"
                    class="signe-montagne pointer-events-none absolute top-1/2 left-1/2 w-[16%] -translate-x-1/2 -translate-y-1/2"
                ></span>
            </span>
            <span class="text-[0.7rem] uppercase tracking-capitales text-encre-douce">
                "Visez avec l'appareil photo"
            </span>
        </div>
    }
}

/// Comment on rejoint la bêta — les gestes, dans l'ordre où on les fait.
///
/// ## Pourquoi cette section existe
///
/// Le public de ce site n'est pas un public de développeurs, et TestFlight est
/// un outil de développeurs qu'Apple a ouvert au grand public sans le
/// réécrire. Il demande d'installer une app pour en installer une autre — un
/// détour que rien n'annonce et que personne n'attend.
///
/// Chaque étape dit **ce qu'on voit à l'écran** quand on la fait, et pas
/// seulement ce qu'on doit faire. « Touchez Obtenir » suppose qu'on ait trouvé
/// le bouton ; « le lien ouvre l'App Store sur la fiche de TestFlight » dit où
/// l'on est arrivé, donc permet de reconnaître qu'on s'est trompé.
///
/// ## Ce qui est dit franchement, et pourquoi
///
/// Les quatre-vingt-dix jours, le point orange, la version qui casse. Une bêta
/// tue la confiance quand une surprise arrive sans avoir été annoncée — et
/// toutes celles-là arrivent. Annoncées, ce sont des détails ; découvertes, ce
/// sont des pannes.
#[component]
fn Beta(
    /// Le lien public de la bêta. Il est **passé**, pas relu depuis `BETA` :
    /// une section qui se rendrait sans lien à poser n'aurait aucun sens, et le
    /// type le dit mieux qu'un commentaire.
    lien: &'static str,
) -> impl IntoView {
    view! {
        <Bloc id="la-beta">
            <TitreDeSection numero="I" titre="Rejoindre la bêta, pas à pas" />

            <p class="text-encre-douce text-pretty">
                "L'app n'est pas encore sur l'App Store. Elle s'installe par "
                <Lien href=TESTFLIGHT><strong>"TestFlight"</strong></Lien>
                " — l'application d'Apple qui sert à essayer "
                "une app avant sa sortie. Elle est gratuite, c'est Apple qui la publie, et "
                "il n'y a pas d'autre chemin : ce qui n'est pas publié ne s'installe pas "
                "depuis l'App Store."
            </p>
            <p class="text-encre-douce text-pretty">
                "Tout se passe sur le téléphone, et il faut deux minutes."
            </p>

            <ol class="m-0 mt-10 grid list-none gap-8 p-0">
                <Etape numero="1" titre="Prenez votre iPhone">
                    "TestFlight n'existe pas sur ordinateur. Si vous lisez cette page sur un "
                    "grand écran, visez "<Lien href="#installer">"le code QR plus haut"</Lien>
                    " avec l'appareil photo du "
                    "téléphone : il ouvre la bonne page, directement."
                </Etape>
                <Etape numero="2" titre="Installez TestFlight">
                    "Si vous ne l'avez pas encore : "
                    <Lien href=TESTFLIGHT>"TestFlight sur l'App Store"</Lien>
                    ", puis « Obtenir ». C'est gratuit, et aucun paiement n'est demandé. "
                    "Le lien de la bêta vous y emmène aussi, mais il y passe par un écran "
                    "de plus."
                </Etape>
                <Etape numero="3" titre="Revenez au lien">
                    "Une fois TestFlight installé, "
                    <Lien href=lien>"rouvrez le lien de la bêta"</Lien>
                    " — ou visez le QR à nouveau. Cette fois il ouvre TestFlight, sur "
                    "La Bible ONT."
                </Etape>
                <Etape numero="4" titre="Touchez « Accepter », puis « Installer »">
                    "Deux boutons, l'un après l'autre. L'app se pose sur l'écran d'accueil "
                    "comme n'importe quelle autre, et elle s'ouvre pareil."
                </Etape>
            </ol>

            <div class="mt-14">
                <h3 class="mb-6">"Ce qu'il faut savoir"</h3>

                <dl class="m-0 grid gap-8 sm:grid-cols-2">
                    <Atout titre="Le point orange">
                        "Un point orange se pose à côté du nom, sur l'écran d'accueil. "
                        "Il dit « version d'essai », et il ne dit rien d'autre."
                    </Atout>
                    <Atout titre="Quatre-vingt-dix jours">
                        "Une version d'essai expire au bout de trois mois. TestFlight "
                        "prévient quand la suivante arrive et l'installe — vous n'avez rien "
                        "à refaire, et vos notes restent."
                    </Atout>
                    <Atout titre="Elle peut casser">
                        "C'est une version en cours, et c'est la raison d'être d'une bêta. "
                        "Si quelque chose ne va pas, "
                        <Lien href="/fr/assistance">"dites-le"</Lien>" : c'est le seul moyen que "
                        "ce soit corrigé."
                    </Atout>
                    <Atout titre="Le jour de la sortie">
                        "Quand l'app paraîtra sur l'App Store, installez-la par-dessus "
                        "celle-ci : elle la remplace et garde ce qu'elle contient. Ne "
                        "supprimez pas la version d'essai avant — supprimer une app emporte "
                        "ce qu'elle gardait sur l'appareil."
                    </Atout>
                </dl>

                // Les trois liens qu'une bêta doit poser, et qu'on cherche
                // rarement au bon endroit.
                //
                // Le corpus d'abord : quelqu'un qui n'a pas d'iPhone, ou qui ne
                // veut rien installer, repartirait d'ici les mains vides alors
                // que le texte entier est à deux clics. Les deux pages légales
                // ensuite — elles existent, elles sont exactes, et une bêta est
                // le moment où l'on se demande ce qu'une app fait de ce qu'on
                // lui confie.
                <p class="mt-10 text-[0.95em] text-encre-douce text-pretty">
                    "Sans rien installer, "<Lien href="/fr/lire">"le corpus se lit ici même"</Lien>
                    " — les mêmes textes, les mêmes trois niveaux, "
                    <Lien href="/fr/lexique">"le même lexique"</Lien>". "
                    <Lien href="/fr/confidentialite">"La confidentialité"</Lien>" dit ce que "
                    "l'app garde sur l'appareil et ce qu'elle n'envoie nulle part ; "
                    <Lien href="/fr/conditions">"les conditions"</Lien>" valent pour la bêta "
                    "comme pour la suite."
                </p>
            </div>
        </Bloc>
    }
}

/// Un geste de l'installation.
///
/// Le chiffre n'est **pas** `aria-hidden`, contrairement au chiffre romain de
/// [`TitreDeSection`]. Là-bas il ornemente ; ici il porte l'ordre, et l'ordre
/// est tout ce qui distingue une suite de gestes d'une liste de conseils.
///
/// La puce native est retirée — on dessine la sienne — et c'est précisément ce
/// qui fait perdre au `<ol>` sa sémantique de liste dans certains lecteurs
/// d'écran. Le chiffre lu à voix haute la rend, sans dépendre d'eux.
///
/// Et il est **aligné**, pas elzévirien. Le site pose les chiffres du texte
/// courant en elzéviriens — ils montent et descendent comme des lettres, ce qui
/// est juste dans une phrase. Dans une pastille, ce dessin fait tomber le
/// chiffre sous le centre du cercle : la même raison qui donne des chiffres
/// alignés aux tableaux, et c'est la même classe.
#[component]
fn Etape(numero: &'static str, titre: &'static str, children: Children) -> impl IntoView {
    view! {
        <li class="flex gap-5">
            <span class="chiffres-tableau mt-0.5 flex size-9 shrink-0 items-center justify-center rounded-full border border-or/35 text-sm text-accent">
                {numero}
            </span>
            <div>
                <p class="m-0 mb-1.5 text-encre">{titre}</p>
                <p class="m-0 text-[0.95em] text-encre-douce text-pretty">{children()}</p>
            </div>
        </li>
    }
}

/// Vrai depuis le 18 août 2026 : Apple a approuvé la 1.0, et la fiche répond.
///
/// Elle avait été soumise le 13 août, renvoyée le 14 au titre de la
/// Guideline 2.1 — des informations manquaient au dossier de revue, rien dans
/// l'app — puis resoumise et approuvée. La version était en mise en vente
/// automatique : Apple l'a publiée lui-même en clôturant la relecture.
///
/// Le basculement suffisait, et c'est ce que la page promettait : le badge et
/// le QR sont apparus ensemble, et la bêta s'est effacée d'elle-même — sa
/// section, le rappel de l'ouverture, la description de la page et la
/// numérotation des sections, toutes suspendues à [`EN_BETA`].
///
/// `tete::IDENTIFIANT_APP_STORE` s'est rallumé dans le même mouvement : c'est
/// la même nouvelle dite à deux endroits, et laisser l'un en arrière rendrait
/// le site incohérent avec lui-même.
///
/// Le repasser à `false` rend la page à la bêta, tant que [`BETA`] porte un
/// lien. C'est la sortie de secours si la fiche devait être retirée.
const PUBLIEE: bool = true;

/// Le lien public du groupe de test externe, quand il y en a un.
///
/// Il vient d'App Store Connect, onglet TestFlight, groupe « Beta », champ
/// `Public Link`. Il désigne le **groupe** et non une version : envoyer une
/// version nouvelle ne le change pas.
///
/// ## Il ne répond que si le groupe a une version approuvée
///
/// C'est la seule chose à surveiller. Sans version approuvée pour le test
/// externe, App Store Connect l'annonce lui-même — « Testers cannot join public
/// link until this group has an approved build » — et le lien mène alors à une
/// page qui refuse poliment. C'est exactement le défaut qu'on évite sur le
/// badge App Store.
///
/// Le remettre à `None` le rend à la mention « en relecture chez Apple », et
/// c'est une seule ligne.
const BETA: Option<&str> = Some("https://testflight.apple.com/join/RAe4uzMu");

/// TestFlight sur l'App Store, par son identifiant.
///
/// C'est une fiche d'Apple, publiée depuis 2015 : elle répond, elle est
/// gratuite, et son identifiant ne bougera pas. Rien à voir avec `FICHE`, qui
/// attend encore une approbation.
///
/// ## Pourquoi le lien direct existe, alors que la bêta y mène déjà
///
/// Le lien de bêta ouvert sans TestFlight installé affiche une page qui propose
/// de l'installer. Ça marche — mais ça fait un aller-retour, sur un écran que le
/// lecteur n'attendait pas, et c'est précisément le moment où l'on croit s'être
/// trompé de lien. Qui sait déjà qu'il ne l'a pas gagne à y aller droit.
const TESTFLIGHT: &str = "https://apps.apple.com/fr/app/testflight/id899247664";

/// La bêta est le chemin du moment : elle existe, et l'App Store n'a pas encore
/// pris le relais.
///
/// La page s'en sert pour trois choses qui doivent basculer **ensemble** — le
/// rappel de l'ouverture, la description de la page, et la numérotation des
/// sections. Les écrire chacune avec sa propre condition les ferait diverger le
/// jour où l'une des deux constantes change.
const EN_BETA: bool = !PUBLIEE && BETA.is_some();

/// La fiche par son identifiant, jamais par un nom lisible : Apple recompose
/// ceux-là quand le titre change, l'identifiant ne bouge pas. Le même que celui
/// du QR — voir `scripts/qr-app.py`.
const FICHE: &str = "https://apps.apple.com/fr/app/id6801192372";
