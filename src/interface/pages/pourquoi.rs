use leptos::prelude::*;

use crate::api::versets;
use crate::interface::design::{
    Bloc, Chronologie, Citation, Comparaison, Correspondance, Correspondances, Exergue, Hero,
    Jalon, LegendeNiveaux, Lien, Nom, Principe, Terme, Titre, TitreDeSection,
};
use crate::interface::echantillon::{bereshit_1_1, SEGOND_1910, SEGOND_SOURCE};
use crate::interface::tete::Tete;

/// « Le pourquoi » — la page de fond du site.
///
/// ## Ce qu'elle doit faire, et que la version d'avant ne faisait pas
///
/// La version précédente tenait en trois blocs de quatre phrases. Elle
/// **affirmait** que le cosmos hébreu est un Temple, sans jamais donner au
/// lecteur de quoi le constater — ni l'histoire qui explique pourquoi il ne le
/// voit plus, ni le texte qui le montre, ni la méthode qui le restitue.
///
/// ## Le cadre se pose en premier, et explicitement
///
/// C'est la leçon d'une épreuve qu'il a menée lui-même : il a demandé à un
/// modèle de langage de juger le site. Le retour a été sévère. Il lui a ensuite
/// demandé de relire **depuis l'ontologie hébraïque antique**, et le jugement a
/// changé — alors que rien du site n'avait bougé.
///
/// Une machine entraînée sur l'écrit du monde retombe donc par défaut dans le
/// cadre grec, et rien sur le site ne l'en sortait. Un lecteur humain n'a
/// aucune raison de faire mieux. D'où l'ordre de cette page : le cadre d'abord,
/// nommé, situé et assumé comme le **bon** — le reste ne se comprend pas sans
/// lui.
///
/// ## Ce n'est pas une polémique
///
/// Le vault le pose en règle (§10) : l'ONT affirme, il ne polémique pas. La
/// page constate donc une dégradation, avec ses dates et ses mots ; elle
/// n'accuse personne. Les traducteurs d'Alexandrie ont fait ce qu'il fallait
/// faire. Ce qui s'est perdu, c'est le souvenir qu'une traduction avait eu
/// lieu.
///
/// ## Les versets viennent du corpus
///
/// Ils sont lus dans `../ONTBibleApp/dist/`, jamais recopiés ici : une
/// citation figée dans la page continuerait de montrer l'ancienne forme d'un
/// verset que le vault aurait corrigé.
#[component]
pub fn Pourquoi() -> impl IntoView {
    view! {
        <Tete
            // « Le pourquoi » ne se cherche pas — c'est un intitulé interne.
            // Le titre indexable nomme la chose : c'est la page où l'on arrive
            // en cherchant l'ontologie hébraïque, et elle est seule à répondre.
            titre="L'ontologie hébraïque antique"
            // Cent cinquante-huit signes. L'ancienne en faisait trois cent
            // soixante-sept — plus du double de ce qu'un moteur affiche, donc
            // la Septante et l'hellénisation, qui sont l'argument, tombaient
            // hors du cadre sans que rien ne le signale.
            description="Une chose n'existe pas parce qu'elle a une substance, mais parce \
                         qu'elle tient une fonction dans un ordre. Ce que l'hellénisation \
                         a recouvert."
            chemin="/fr/le-pourquoi"
        />

        <Hero sobre=true>
            <p class="text-sm uppercase tracking-capitales text-accent">"Le cadre"</p>
            <h1 class="text-balance">"Le réel ne fonctionne pas en substances"</h1>
            <p class="max-w-xl text-encre-douce text-balance">
                "Il fonctionne en fonctions. C'est l'ontologie hébraïque antique. "
                "Et ce n'est pas un cadre parmi d'autres — c'est le bon."
            </p>
        </Hero>

        // ── I ─────────────────────────────────────────────────────────────
        //
        // Le cadre en premier. Sans lui, tout ce qui suit se lit depuis le
        // cadre du lecteur, et se juge donc faux.
        <Bloc eclaire=true>
            <TitreDeSection numero="I" titre="Ce texte ne se lit pas d'où vous êtes" />

            <p class="lettrine">
                "Un cadre ne se voit pas. On ne le regarde pas\u{a0}: on regarde avec. "
                "C'est ce qui le rend si difficile à déposer — il ne se présente jamais "
                "comme un point de vue, il se présente comme le réel."
            </p>
            <p>
                "Voici comment nous l'avons mesuré. Nous avons demandé à une intelligence "
                "artificielle de juger ce site. Le retour a été sévère\u{202f}: les choix "
                "paraissaient arbitraires, la terminologie inutilement opaque, la démarche "
                "discutable. Nous lui avons ensuite demandé de relire la même chose depuis "
                "l'ontologie hébraïque antique fonctionnelle."
            </p>
            <p>
                "Le jugement a changé. Pas une ligne du site n'avait bougé."
            </p>

            <Exergue>
                "Ce qui avait changé, c'était le cadre. "
                "Et le cadre n'est pas ce qu'on lit — c'est ce avec quoi on lit."
            </Exergue>

            <p>
                "Une machine entraînée sur presque tout ce que l'humanité a écrit retombe donc "
                "par défaut dans le cadre grec. Elle ne le choisit pas\u{202f}; elle ne sait "
                "même pas qu'elle en a un. Si elle n'en sort pas seule, un lecteur n'en sortira "
                "pas non plus."
            </p>
            <p>
                "D'où cette page, et d'où son ordre. On ne peut pas juger une restitution "
                "depuis le cadre qu'elle défait. Le cadre vient donc en premier."
            </p>
        </Bloc>

        // ── II ────────────────────────────────────────────────────────────
        //
        // Le mot est dans le nom du projet, et il n'avait jamais été expliqué
        // nulle part sur le site. Or c'est l'étage où tout se joue : une
        // traduction change des mots, l'ONT change ce que les mots supposent.
        <Bloc>
            <TitreDeSection numero="II" titre="Ce que le mot «\u{202f}ontologie\u{202f}» attrape" />

            <p class="lettrine">
                "Le mot a l'air d'un terme de séminaire. Il ne l'est pas. Il nomme la question "
                "la plus élémentaire qu'on puisse poser, et la seule à laquelle tout le monde "
                "a déjà répondu sans y avoir jamais pensé\u{a0}: "
                <i>"qu'est-ce que ça veut dire, pour une chose, d'exister\u{202f}?"</i>
            </p>
            <p>
                "Vous avez une réponse. Elle est en vous depuis avant que vous sachiez lire, "
                "vous ne l'avez pas choisie, et vous ne vous souvenez pas de l'avoir apprise. "
                "C'est précisément ce qui la rend puissante."
            </p>
            <p>
                "Car une ontologie n'est pas une opinion sur le monde. C'est ce qui décide "
                "quelles opinions sont seulement "
                <i>"formulables"</i>
                ". Elle se tient en amont du vrai et du faux\u{202f}: elle fixe ce qui compte "
                "comme une chose, comme un événement, comme une cause — et ce qui, dit dans "
                "ses termes, n'a tout simplement aucun sens."
            </p>

            <Exergue>
                "On ne discute pas d'une ontologie. On discute "
                "à l'intérieur de la sienne, sans savoir qu'on y est."
            </Exergue>

            <p>
                "Prenez la phrase que vous lirez deux sections plus bas\u{202f}: la Terre "
                "existait, et elle n'existait pas. Dans une ontologie de la substance, c'est "
                "une contradiction — la phrase est fautive, il n'y a rien à comprendre. Dans "
                "une ontologie fonctionnelle, c'est une description exacte, et même précise. "
                "Le désaccord ne porte pas sur la Terre. Il porte sur ce que veut dire "
                "«\u{202f}exister\u{202f}»."
            </p>
            <p>
                "D'où la conséquence, et elle est la raison d'être de tout ce projet. Ce qui "
                "sépare l'ONT des traductions n'est pas un désaccord d'interprétation. Deux "
                "lecteurs peuvent s'accorder sur chaque mot d'une phrase, sur sa grammaire et "
                "sur son contexte, et n'avoir pas lu la même chose — parce qu'ils ne mettent "
                "pas la même réalité sous le verbe être. Le désaccord est un étage en dessous "
                "de l'interprétation, et c'est pour cela que la discussion ordinaire ne "
                "l'atteint jamais."
            </p>
            <p>
                "Il faut assumer une ironie au passage\u{202f}: «\u{202f}ontologie\u{202f}» "
                "est un mot grec — "
                <i lang="grc">"ta onta"</i>
                ", les étants, et "
                <i lang="grc">"logos"</i>
                ". Nous empruntons donc son mot à la pensée dont nous défaisons le cadre. "
                "C'est assumé\u{202f}: c'est le seul mot dont le français dispose pour "
                "désigner cet étage, et refuser de l'employer reviendrait à ne pas pouvoir "
                "dire de quoi il s'agit. On lui emprunte le mot. Rien d'autre."
            </p>

            <Principe chute=true>
                "Et voilà pourquoi ce travail s'appelle l'ONT. Le nom dit à quel étage il se "
                "fait. Une traduction change des mots\u{202f}; celle-ci change ce que les mots "
                "supposent — c'est ce qui en fait une restitution, et non une version de plus."
            </Principe>
        </Bloc>

        // ── III ───────────────────────────────────────────────────────────
        <Bloc>
            <TitreDeSection numero="III" titre="Nous descendons des Grecs" />

            <p class="lettrine">
                "Notre pensée pose une question avant toutes les autres, et elle la pose si "
                "vite qu'on ne l'entend plus\u{a0}: "
                <i>"qu'est-ce que c'est\u{202f}?"</i>
                " De quoi est-ce fait. Quelle est sa nature. Ce qu'une chose "
                <i>"est"</i>
                ", indépendamment de ce qu'elle fait et de qui l'entoure."
            </p>
            <p>
                "C'est une question grecque. Elle a un âge, un lieu, des auteurs. Et elle a "
                "laissé ses mots dans notre langue, si profondément qu'on les prend pour des "
                "outils neutres. "
                <i>"Substance"</i>
                " — ce qui se tient dessous, le socle qui reste quand tout le reste change. "
                <i>"Essence"</i>
                " — ce que la chose est en elle-même. "
                <i>"Matière"</i>
                " et "
                <i>"forme"</i>
                ". "
                <i>"Âme"</i>
                " et "
                <i>"corps"</i>
                ", deux substances de nature différente logées l'une dans l'autre."
            </p>
            <p>
                "Ce vocabulaire décide d'avance ce qu'une chose peut être. Il pose qu'il y a "
                "un dessous stable, et que la fonction n'en est qu'un accident. Il pose que "
                "l'être précède le rôle."
            </p>
            <p>
                "Nous croyons poser des questions sans présupposé. Nous posons des questions "
                "grecques."
            </p>
        </Bloc>

        // ── III ───────────────────────────────────────────────────────────
        // Mesure **normale**, bien que la section cite le corpus : c'est
        // `Citation` qui déborde de la colonne, et non le bloc qui s'élargit.
        // Élargir le bloc corrigeait la césure du verset et la donnait à la
        // prose — 79 signes par ligne à 21 px.
        <Bloc>
            <TitreDeSection numero="IV" titre="La question hébraïque n'est pas celle-là" />

            <p class="lettrine">
                "Le monde hébreu antique ne demande pas de quoi une chose est faite. Il "
                "demande quel rôle elle tient, quel nom elle porte, quelle place elle occupe "
                "dans un ordre. Une chose existe quand elle a une fonction assignée. Tant "
                "qu'elle n'en a pas, elle est là — et elle n'existe pas."
            </p>
            <p>
                "Cette phrase paraît absurde en français. Le texte, lui, l'écrit noir sur "
                "blanc, dès le deuxième verset de "
                <Nom>"Bereshit"</Nom>
                " — le livre que le français appelle la Genèse\u{202f}:"
            </p>

            {citer("Bereshit 1:2", "bereshit", "bereshit-1", vec![2])}

            <p>
                "La Terre "
                <i>"existait déjà"</i>
                ". La matière est là. Les eaux sont là. Et le texte dit pourtant qu'elle est "
                <Terme lemme="tohu-vavohu">"tohu vavohu"</Terme>
                " — sans ordre, sans fonction, sans habitant. Présente matériellement, "
                "inexistante fonctionnellement. Aucun cadre grec ne peut porter cette phrase "
                "sans la casser\u{202f}: dans la substance, ce qui est là "
                <i>"est"</i>
                ", et il n'y a rien à ajouter."
            </p>
            <p>
                "Ce qui manque, le texte va le donner\u{202f}: des limites, des séparations, "
                "des rôles — et des noms."
            </p>

            {citer("Bereshit 1:10", "bereshit", "bereshit-1", vec![10])}

            <p>
                "Nommer n'est pas étiqueter une chose déjà là. C'est "
                <Terme lemme="qara">"qara"</Terme>
                ", l'acte souverain par excellence\u{202f}: faire entrer dans l'existence "
                "fonctionnelle. Ce qui était "
                <Terme lemme="tohu-vavohu">"tohu vavohu"</Terme>
                " au verset 2 entre dans son existence accomplie au verset 10, et rien n'a "
                "été fabriqué entre les deux."
            </p>

            <Principe chute=true>
                "C'est pourquoi une réalité qui fonctionne autrement mérite un nom à elle. "
                "Nommer, dans ce cadre, n'est pas une commodité de vocabulaire — c'est le "
                "geste qui fait exister."
            </Principe>
        </Bloc>

        // ── IV ────────────────────────────────────────────────────────────
        //
        // La section qu'il a demandée explicitement, et la plus exposée : elle
        // affirme un cadre vrai, pas un cadre équivalent. Elle affirme et ne
        // polémique pas — c'est la règle §10 du vault.
        // ── V ─────────────────────────────────────────────────────────────
        //
        // Sa formulation : « l'hébreu n'est pas une langue qui définit le réel,
        // c'est une langue qui le constitue ». C'est la preuve interne du
        // cadre — elle ne vient pas du dehors, elle vient de la langue.
        <Bloc>
            <TitreDeSection numero="V" titre="Une langue qui constitue, non qui définit" />

            <p class="lettrine">
                "La preuve du cadre se trouve à l'intérieur du cadre, et c'est la langue "
                "elle-même. Le grec et l'hébreu ne font pas le même travail. Le grec "
                <i>"définit"</i>
                " le réel. L'hébreu le "
                <i>"constitue"</i>
                "."
            </p>
            <p>
                "Définir, en grec, se dit "
                <i lang="grc">"horismos"</i>
                " — de "
                <i lang="grc">"horos"</i>
                ", la borne, la pierre plantée en limite d'un champ. Définir, c'est tracer "
                "une frontière autour d'une chose pour la séparer de ce qu'elle n'est pas. Et "
                "pour tracer une frontière, il faut se tenir "
                <i>"dehors"</i>
                ". Une langue de définition suppose donc deux choses\u{202f}: un réel déjà "
                "là, complet, et un locuteur en surplomb qui le découpe."
            </p>
            <p>
                "L'hébreu du corpus ne fait pas cela. Sa parole n'est pas descriptive, elle "
                "est "
                <i>"performative"</i>
                "\u{202f}: elle accomplit ce qu'elle énonce. "
                <Terme lemme="elohim">"Elohim"</Terme>
                " ne dit pas que la Lumière est — il la formule, et elle advient. Nommer "
                "n'étiquette pas ce qui se trouvait déjà là\u{202f}: "
                <Terme lemme="qara">"qara"</Terme>
                " fait entrer dans l'existence. La parole ne vient pas après le réel pour en "
                "rendre compte. Elle est le mouvement par lequel il tient."
            </p>

            <Exergue>
                "Le grec parle sur le monde. L'hébreu parle le monde."
            </Exergue>

            <p>
                "Et la preuve n'est pas une interprétation\u{202f}: elle est dans le lexique. "
                "En hébreu, "
                <Terme lemme="davar">"davar"</Terme>
                " veut dire la parole "
                <i>"et"</i>
                " la chose — le même mot, sans métaphore. Il nous faut deux mots français "
                "parce que nous tenons pour évident qu'il s'agit de deux réalités. L'hébreu ne "
                "les distingue pas, et ce n'est pas une pauvreté de vocabulaire\u{202f}: "
                "c'est que dans ce cadre elles ne sont pas séparées. Une langue garde dans ses "
                "mots l'ontologie de ceux qui l'ont parlée."
            </p>
            <p>
                "Et cela tient à une différence plus profonde encore, qui touche la façon dont "
                "chaque langue "
                <i>"porte"</i>
                " le sens. Le grec — et le français après lui — "
                <i>"analyse"</i>
                ". Le mot vient d'"
                <i lang="grc">"analusis"</i>
                ", délier, défaire en éléments séparés. Une pensée qui analyse multiplie les "
                "mots pour multiplier les distinctions, et chaque mot occupe alors une case "
                "close\u{202f}: le grec a "
                <i lang="grc">"erōs"</i>
                ", "
                <i lang="grc">"philia"</i>
                " et "
                <i lang="grc">"agapē"</i>
                " là où nous disons l'amour, "
                <i lang="grc">"chronos"</i>
                " et "
                <i lang="grc">"kairos"</i>
                " là où nous disons le temps. L'ensemble forme une grille, et la précision "
                "vient de la finesse du découpage."
            </p>
            <p>
                "L'hébreu ne découpe pas. Il est "
                <i>"holistique"</i>
                " — d'"
                <i lang="grc">"holos"</i>
                ", le tout, et l'ironie du mot grec est la même que pour "
                "«\u{202f}ontologie\u{202f}». Un mot hébreu n'est pas une case, c'est un "
                <b>"champ"</b>
                ". Il ne porte pas un sens que le contexte préciserait\u{202f}: il porte un "
                "domaine entier, dont le contexte éclaire une région "
                <i>"sans jamais éteindre les autres"</i>
                "."
            </p>
            <p>
                "C'est très exactement ce que le français ne sait pas faire. "
                <Terme lemme="ruach">"Ruach"</Terme>
                " est souffle, vent et esprit — non pas tour à tour selon le passage, mais "
                "les trois "
                <i>"ensemble"</i>
                ", indissociablement. Quand le texte dit que le "
                <Terme lemme="ruach">"Ruach"</Terme>
                " d'"
                <Terme lemme="elohim">"Elohim"</Terme>
                " couvait la face des eaux, il dit un vent qui souffle et une présence qui "
                "agit, dans le même mot et sans les distinguer. Le traducteur doit choisir. "
                "Choisir, ici, c'est amputer."
            </p>
            <p>
                "Et le mécanisme de ces champs est dans la structure même de la langue\u{202f}: "
                "la "
                <b>"racine"</b>
                ". Trois consonnes portent un domaine, et tout ce qui en dérive y participe. "
                "De "
                <i>"k-b-d"</i>
                ", être lourd, viennent la "
                <Terme lemme="kavod">"kavod"</Terme>
                " — le poids d'une réalité dans l'ordre — et "
                <i>"kaved"</i>
                ", le foie, l'organe lourd du corps. Un lecteur hébreu entend la parenté en "
                "lisant. Le français lit «\u{202f}gloire\u{202f}» et «\u{202f}foie\u{202f}» "
                "et n'entend rien du tout, parce que le lien n'est pas une curiosité "
                "d'étymologiste\u{202f}: il est vivant, actif à chaque emploi."
            </p>
            <p>
                "Jusqu'aux lettres, qui ne sont pas de purs signes de son. Chacune porte un "
                "nom qui est un mot, et une forme qui vient du dessin de la chose\u{202f}: "
                <i>"aleph"</i>
                " le bœuf, "
                <i>"beth"</i>
                " la maison, "
                <i>"yod"</i>
                " la main, "
                <i>"ayin"</i>
                " l'œil. Une langue dont les éléments les plus petits portent déjà du sens ne "
                "peut pas fonctionner comme une langue dont les lettres n'en portent aucun."
            </p>
            <p>
                "On comprend alors pourquoi l'ONT ne peut pas tenir sur une seule ligne. "
                "Traduire un champ par une case est ce que fait toute traduction, et c'est "
                "exactement là qu'elle perd. Les trois niveaux ne sont pas un ornement "
                "d'édition\u{202f}: ils sont la réponse technique à ce problème-là. Le corps "
                "donne la région que le passage éclaire, la glose rend le champ, et le niveau 3 "
                "pose le mot lui-même pour qu'on puisse aller vérifier."
            </p>
            <p>
                "On mesure alors ce que la traduction en grec a réellement fait. Elle n'a pas "
                "transporté un sens d'une langue vers une autre. Elle a fait passer une parole "
                <i>"constituante"</i>
                " dans une langue "
                <i>"descriptive"</i>
                ". Ce qui était un acte est devenu un énoncé au sujet de. Le texte dit encore "
                "la même chose, et il ne fait plus rien."
            </p>
            <p>
                "Cette langue porte un nom\u{202f}: la "
                <Nom>"Lashon ha-Qodesh"</Nom>
                " "
                <span dir="rtl" lang="he" class="font-hebreu">"לְשׁוֹן הַקֹּדֶשׁ"</span>
                " — la langue consacrée, mise à part, celle du sanctuaire. Et il faut être "
                "exact sur ce que nous tenons\u{202f}: l'hébreu du corpus n'est pas elle. Il "
                "en est le meilleur "
                <i>"vestige"</i>
                " — ce qui nous en est parvenu, et le témoin le plus complet dont on dispose."
            </p>
            <p>
                "Ce n'est pas de la nostalgie, et ce n'est pas une croyance qu'on demande "
                "d'adopter. C'est ce qui rend le travail possible. La mécanique est encore là, "
                "intacte et vérifiable\u{202f}: la parole qui accomplit, le "
                <Terme lemme="shem">"Shem"</Terme>
                " qui charge celui qui le porte, la racine de trois lettres qui tient un champ "
                "entier de sens. On ne ressuscite pas une langue perdue — on lit ce qui reste, "
                "et ce qui reste suffit."
            </p>
        </Bloc>

        <Bloc eclaire=true>
            <TitreDeSection numero="VI" titre="Et c'est le bon cadre" />

            <p class="lettrine">
                "Disons-le sans détour, parce que tout le reste en dépend. L'ontologie "
                "hébraïque antique n'est pas une curiosité culturelle qu'on restituerait par "
                "respect pour les anciens. Ce n'est pas une grille parmi d'autres, à ranger "
                "près des cosmologies exotiques. C'est la façon dont le réel fonctionne. Le "
                "reste est distorsion."
            </p>
            <p>
                "Regardez ce que vous pouvez constater. Personne n'a jamais rencontré une "
                "essence. On rencontre des choses qui tiennent un rôle, ou qui ont cessé de "
                "le tenir. Un cœur est un cœur parce qu'il bat. Une eau est potable par ce "
                "qu'elle fait. Une monnaie vaut par ce qu'elle permet, une loi par ce qu'elle "
                "ordonne, un outil par ce qu'il accomplit. Retirez la fonction et il ne reste "
                "pas une substance nue\u{202f}: il reste un débris, et l'on change son nom."
            </p>
            <p>
                "La substance, elle, ne se rencontre jamais. Elle se postule. C'est une "
                "couche que la pensée grecque a posée sous le réel pour l'expliquer, et que "
                "vingt-cinq siècles ont fini par prendre pour le fond."
            </p>
            <p>
                "On le vérifie aux problèmes que chaque cadre engendre. Le cadre grec a "
                "produit des difficultés qui n'existent que dans lui\u{202f}: comment une âme "
                "immatérielle agit-elle sur un corps matériel, où logent les universaux, "
                "qu'est-ce qui demeure d'une chose quand ses propriétés changent. Des "
                "bibliothèques entières. Dans le cadre hébreu, ces questions ne reçoivent pas "
                "une meilleure réponse — elles ne se posent pas. Un cadre qui dissout les "
                "énigmes d'un autre au lieu de les résoudre n'est pas son égal."
            </p>

            <Exergue>
                "Il n'y a pas deux lectures qui se valent. "
                "Il y a un cadre, et il y a ce qui s'est posé par-dessus."
            </Exergue>

            <p>
                "Restituer n'est donc pas un travail d'archive. C'est un redressement. Et "
                "quand le cadre est remis d'aplomb, le texte cesse d'être obscur — il devient "
                "cohérent de bout en bout, ce qu'aucune lecture en substance ne parvient à le "
                "rendre."
            </p>
        </Bloc>

        // ── V ─────────────────────────────────────────────────────────────
        <Bloc large=true>
            <TitreDeSection numero="VII" titre="L'hellénisation, ou comment le grec est entré" />

            <p class="lettrine">
                "La dégradation a une histoire, et elle est datable. Elle ne commence pas au "
                "Moyen Âge ni à la Réforme. Elle commence pendant que le corpus s'écrit "
                "encore."
            </p>
            <p>
                "Son nom est l'hellénisation. Et il faut d'emblée défaire l'image qu'on s'en "
                "fait\u{202f}: ce n'est pas une invasion culturelle imposée à un peuple qui "
                "résiste. C'est une adoption. Après Alexandre, le grec n'est pas la langue de "
                "l'occupant — c'est celle de la réussite. On l'apprend pour commercer, pour "
                "plaider, pour être lu. Le monde entier veut y entrer, et une partie de "
                <Nom>"Yehudah"</Nom> " — la Judée — la première."
            </p>
            <p>
                "L'hellénisation n'apporte pas seulement une langue. Elle apporte un système "
                "complet, et il se tient\u{202f}: une école — la "
                <i lang="grc">"paideia"</i>
                " —, un idéal du corps, une cité qui se gouverne, une physique, une "
                "métaphysique. Prendre la langue, c'est prendre le reste, parce que les mots "
                "de cette langue sont les mots de cette pensée. On ne peut pas dire "
                <i lang="grc">"psychē"</i>
                " sans dire un peu Platon."
            </p>
            <p>
                "C'est le mécanisme, et il n'a rien de violent\u{202f}: on garde le texte, on "
                "garde les fêtes, on garde le Temple — et l'on change la façon de penser ce "
                "qu'on garde. Voilà pourquoi la chose ne s'est vue de personne. Il n'y a pas "
                "eu de rupture à dater. Il y a eu une traduction, puis une école, puis un "
                "vocabulaire, puis un cadre."
            </p>

            <Chronologie>
                <Jalon date="586 av.">
                    <Titre slot>"La déportation à " <Nom>"Bavel"</Nom> " (Babylone)"</Titre>
                    <p>
                        <Nom>"Yehudah"</Nom> " tombe, le Temple brûle, la population lettrée part en exil. "
                        "L'araméen s'installe comme langue parlée. Pour la première fois, la "
                        "langue du texte cesse d'être la langue de tous les jours\u{202f}: "
                        "l'hébreu devient une langue qu'on apprend."
                    </p>
                </Jalon>

                <Jalon date="332 av.">
                    <Titre slot>"Alexandre, et le grec comme langue du monde"</Titre>
                    <p>
                        "L'Orient passe sous administration grecque. Le grec de la "
                        <i>"koinè"</i>
                        " devient la langue des affaires, du droit et de l'école, du Nil à "
                        "l'Indus. Ce n'est pas une langue de plus\u{202f}: c'est une langue "
                        "qui arrive avec une école, une physique et une métaphysique déjà "
                        "faites."
                    </p>
                </Jalon>

                <Jalon date="312 av.">
                    <Titre slot>"La déportation en " <Nom>"Mitsrayim"</Nom> " (l'Égypte)"</Titre>
                    <p>
                        "Ptolémée\u{a0}Iᵉʳ prend " <Nom>"Yerushalayim"</Nom> " — Jérusalem — et "
                        "emmène en Égypte "
                        "des milliers de captifs de " <Nom>"Yehudah"</Nom> " et de " <Nom>"Shomron"</Nom>
                        ", la Judée et la "
                        "Samarie, qu'il installe comme colons et "
                        "mercenaires. C'est l'un des actes fondateurs de la grande communauté "
                        "d'Alexandrie. Une génération plus tard, elle est nombreuse, prospère "
                        "— et elle ne lit plus l'hébreu."
                    </p>
                </Jalon>

                <Jalon date="IIIᵉ siècle av.">
                    <Titre slot>"La Septante"</Titre>
                    <p>
                        "Sous Ptolémée\u{a0}II, la " <Nom>"Torah"</Nom> " est traduite en grec à "
                        "Alexandrie. Le "
                        "reste suivra au siècle suivant. Ce n'est ni une trahison ni une "
                        "négligence\u{202f}: c'est une nécessité. Un peuple entier ne comprend "
                        "plus le texte qui le constitue, et il fallait qu'il l'entende."
                    </p>
                    <p class="mt-4">
                        "Mais traduire, c'est choisir. Et chaque mot grec disponible arrivait "
                        "déjà chargé de la philosophie qui l'avait travaillé. On n'a pas "
                        "transporté un sens dans un contenant vide — on l'a versé dans un "
                        "récipient qui avait sa forme."
                    </p>
                </Jalon>

                <Jalon date="175 av.">
                    <Titre slot>"Le gymnase à " <Nom>"Yerushalayim"</Nom></Titre>
                    <p>
                        "Voici l'hellénisation dans un seul fait, et c'est le plus parlant de "
                        "toute cette page. " <Nom>"Yason"</Nom> " — que l'histoire appelle Jason — achète le "
                        "grand-sacerdoce à "
                        "Antiochus\u{a0}IV, puis achète le droit d'ouvrir un "
                        <i lang="grc">"gymnasion"</i>
                        " et un "
                        <i lang="grc">"ephēbeion"</i>
                        " à " <Nom>"Yerushalayim"</Nom> ", et de faire enregistrer les habitants comme citoyens "
                        "d'Antioche. La ville devient une "
                        <i lang="grc">"polis"</i>
                        " grecque."
                    </p>
                    <p class="mt-4">
                        "Elle ne lui est pas imposée. Elle est "
                        <i>"achetée"</i>
                        ", par le grand prêtre, avec l'argent du Temple. Et les "
                        <Terme>"kohanim"</Terme>
                        " — les prêtres du Temple — y courent\u{202f}: le récit de "
                        <i>"Maccabées"</i>
                        " les montre délaissant le service pour la palestre."
                    </p>
                    <p class="mt-4">
                        "Le gymnase n'est pas une salle de sport. C'est l'institution qui "
                        "forme un homme grec — corps, langue, poésie, façon de raisonner. En "
                        "une génération, l'élite qui transmet le texte a été formée à penser "
                        "dans l'autre cadre. C'est là que "
                        <Terme>"musar"</Terme>
                        ", la correction qui redresse, devient "
                        <i lang="grc">"paideia"</i>
                        ", la culture qui élève."
                    </p>
                    <p class="mt-4">
                        "La crise de 167 et la révolte des Maccabées ont rendu le Temple. "
                        "Elles n'ont pas rendu le cadre\u{202f}: on peut chasser une garnison, "
                        "on ne désapprend pas une manière de penser."
                    </p>
                </Jalon>

                <Jalon date="Iᵉʳ siècle">
                    <Titre slot>"Philon, et la lecture par Platon"</Titre>
                    <p>
                        "Philon d'Alexandrie lit la " <Nom>"Torah"</Nom> " avec les catégories de "
                        "Platon et des "
                        "stoïciens. Le grec n'est plus seulement dans le vocabulaire\u{202f}: "
                        "il est dans la pensée. C'est le seuil que l'ONT tient pour "
                        "disqualifiant — un texte d'auteur juif écrit en catégories "
                        "hellénistiques ne dit plus le cosmos hébreu, et il n'entre pas au "
                        "corpus."
                    </p>
                </Jalon>

                <Jalon date="Iᵉʳ–IIᵉ siècle">
                    <Titre slot>"La " <Nom>"Berit Hadashah"</Nom> " cite la Septante"</Titre>
                    <p>
                        "Les textes de la "
                        <Nom>"Berit Hadashah"</Nom>
                        " — ce que la tradition latine a nommé le Nouveau Testament — sont "
                        "transmis en grec et citent l'Écriture dans sa version grecque. "
                        "Le vocabulaire d'Alexandrie entre ainsi dans le texte sacré "
                        "lui-même\u{202f}: "
                        <i lang="grc">"ψυχή"</i>
                        ", "
                        <i lang="grc">"νόμος"</i>
                        ", "
                        <i lang="grc">"δικαιοσύνη"</i>
                        ". Ce que les auteurs veulent dire reste hébreu. Les mots dont ils "
                        "disposent, non."
                    </p>
                </Jalon>

                <Jalon date="IVᵉ–Vᵉ siècle">
                    <Titre slot>"Le latin fige, et le canon se ferme"</Titre>
                    <p>
                        "Jérôme traduit l'Ancien Testament depuis l'hébreu — c'est son mérite, "
                        "et il faut le dire. Mais il écrit dans un latin d'église dont le "
                        "lexique est déjà formé sur le grec\u{202f}: "
                        <i>"anima"</i>
                        ", "
                        <i>"peccatum"</i>
                        ", "
                        <i>"justitia"</i>
                        ", "
                        <i>"aeternitas"</i>
                        ", "
                        <i>"infernus"</i>
                        ", "
                        <i>"sacerdos"</i>
                        ". La Vulgate fixe pour mille ans. Au même moment, le canon se "
                        "ferme — une opération tardive, que les auteurs des textes n'ont "
                        "jamais connue."
                    </p>
                </Jalon>

                <Jalon date="IVᵉ–Vᵉ siècle">
                    <Titre slot>"Les conciles parlent en substance"</Titre>
                    <p>
                        "La doctrine se formule désormais dans le vocabulaire technique de la "
                        "métaphysique grecque\u{202f}: "
                        <i lang="grc">"οὐσία"</i>
                        " (la substance), "
                        <i lang="grc">"ὑπόστασις"</i>
                        ", "
                        <i lang="grc">"φύσις"</i>
                        " (la nature). On ne discute plus de fonctions et de rôles dans un "
                        "ordre. On discute de natures et de substances. Le basculement est "
                        "achevé, et il est officiel."
                    </p>
                </Jalon>

                <Jalon date="XVIᵉ siècle → nos jours">
                    <Titre slot>"Le français hérite"</Titre>
                    <p>
                        "Les traducteurs modernes reviennent à l'hébreu, sérieusement et "
                        "honnêtement. Mais ils écrivent dans une langue dont les mots "
                        "religieux ont été forgés par mille ans de latin d'église. "
                        "«\u{202f}Âme\u{202f}», «\u{202f}péché\u{202f}», "
                        "«\u{202f}justice\u{202f}», «\u{202f}éternité\u{202f}», "
                        "«\u{202f}enfer\u{202f}», «\u{202f}ange\u{202f}», "
                        "«\u{202f}prêtre\u{202f}», «\u{202f}bénir\u{202f}»\u{202f}: chacun "
                        "arrive déjà décidé. Le traducteur choisit le mot juste dans une "
                        "langue dont les mots ne sont plus justes."
                    </p>
                </Jalon>
            </Chronologie>

            <div class="mt-16">
                <Principe chute=true>
                    "Nous ne lisons donc pas un texte hébreu avec un léger accent. Nous lisons "
                    "une ontologie grecque qui a gardé les noms propres hébreux."
                </Principe>
            </div>
        </Bloc>

        // ── VI ────────────────────────────────────────────────────────────
        <Bloc large=true>
            <TitreDeSection numero="VIII" titre="Neuf mots, et ce qu'ils sont devenus" />

            <p>
                "Ce ne sont pas des nuances de traduction. Dans chaque cas, le mot d'arrivée "
                "porte une ontologie que le mot de départ n'a pas — et c'est le mot d'arrivée "
                "que nous avons hérité."
            </p>

            <Correspondances>
                <Correspondance
                    hebreu="נֶפֶשׁ" translitteration="nefesh"
                    sens="la gorge, le souffle — le principe vital concret et incarné"
                    grec="ψυχή" grec_translitteration="psychē"
                    francais="âme"
                >
                    <p>
                        "Le "
                        <Terme lemme="nefesh">"nefesh"</Terme>
                        " est ce qui respire, désire et a faim. Il n'est pas dans le corps\u{202f}: "
                        "il "
                        <i>"est"</i>
                        " le vivant. Or "
                        <i lang="grc">"psychē"</i>
                        " avait déjà été travaillé par Platon, chez qui elle se sépare du "
                        "corps et lui survit. Les animaux ont un "
                        <Terme lemme="nefesh">"nefesh"</Terme>
                        " dans le texte hébreu — ce qui devient inintelligible dès qu'on lit "
                        "«\u{202f}âme\u{202f}»."
                    </p>
                </Correspondance>

                <Correspondance
                    hebreu="טוֹב" translitteration="tov"
                    sens="pleinement ajusté à sa destination dans l'ordre"
                    grec="καλόν" grec_translitteration="kalon"
                    francais="bon"
                >
                    <p>
                        "En "
                        <Nom>"Bereshit"</Nom>
                        " 1:4, la Septante rend "
                        <Terme lemme="tov">"tov"</Terme>
                        " par "
                        <i lang="grc">"kalon"</i>
                        " — «\u{202f}beau\u{202f}». D'un verdict d'inspection, on passe à un "
                        "jugement esthétique, puis, par le latin, à un jugement moral. "
                        <Terme lemme="tov">"Tov"</Terme>
                        " ne dit ni la beauté ni la vertu\u{202f}: il dit que la chose "
                        "accomplit sa fonction."
                    </p>
                </Correspondance>

                <Correspondance
                    hebreu="תּוֹרָה" translitteration="torah"
                    sens="l'instruction, la direction visée — de yarah, viser"
                    grec="νόμος" grec_translitteration="nomos"
                    francais="la Loi"
                >
                    <p>
                        "Presque toutes ses occurrences passent par "
                        <i lang="grc">"nomos"</i>
                        ", le mot de la loi civique grecque. Une instruction qui oriente "
                        "devient un code qui contraint. Des siècles de discussion sur "
                        "«\u{202f}la Loi\u{202f}» reposent sur ce seul choix de vocabulaire."
                    </p>
                </Correspondance>

                <Correspondance
                    hebreu="כָּבוֹד" translitteration="kavod"
                    sens="le poids — de kaved, être lourd\u{a0}: la pesanteur d'une réalité dans l'ordre"
                    grec="δόξα" grec_translitteration="doxa"
                    francais="gloire"
                >
                    <p>
                        "Le renversement le plus net. En grec classique, "
                        <i lang="grc">"doxa"</i>
                        " est l'apparence, l'opinion — chez Platon, précisément ce qui "
                        "s'oppose au savoir vrai. Le grec a donc pris le mot de "
                        <i>"ce qui paraît"</i>
                        " pour traduire le mot de "
                        <i>"ce qui pèse"</i>
                        ". La "
                        <Terme lemme="kavod">"kavod"</Terme>
                        " se mesure, elle ne se contemple pas."
                    </p>
                </Correspondance>

                <Correspondance
                    hebreu="שְׁאוֹל" translitteration="She'ol"
                    sens="le domaine bas du silence et de l'attente, où descend tout mort"
                    grec="ᾅδης" grec_translitteration="hadēs"
                    francais="l'enfer"
                >
                    <p>
                        "Au "
                        <Terme lemme="sheol">"She'ol"</Terme>
                        ", les morts sont morts. Dans l'Hadès grec, les âmes des morts sont "
                        "vivantes. D'un trait de plume, des morts sont devenus des vivants — "
                        "et le lieu du silence est devenu, par le latin "
                        <i>"infernus"</i>
                        ", un lieu de tourment. On descend au "
                        <Terme lemme="sheol">"She'ol"</Terme>
                        "\u{202f}; on ne s'en envole pas."
                    </p>
                </Correspondance>

                <Correspondance
                    hebreu="עוֹלָם" translitteration="olam"
                    sens="l'horizon qui se dérobe — la limite que le regard ne discerne pas"
                    grec="αἰών" grec_translitteration="aiōn"
                    francais="éternité"
                >
                    <p>
                        <Terme lemme="olam">"Olam"</Terme>
                        " vient d'une racine de dissimulation\u{202f}: ce qui est au-delà du "
                        "visible. Le latin "
                        <i>"aeternitas"</i>
                        " en fait une durée infinie, puis un hors-du-temps. Une promesse "
                        "«\u{202f}jusqu'à l'horizon\u{202f}» et une promesse «\u{202f}pour "
                        "l'éternité\u{202f}» ne disent pas la même chose."
                    </p>
                </Correspondance>

                <Correspondance
                    hebreu="צֶדֶק" translitteration="tsedeq"
                    sens="la conformité structurelle au bon fonctionnement du réel"
                    grec="δικαιοσύνη" grec_translitteration="dikaiosynē"
                    francais="justice"
                >
                    <p>
                        <i lang="grc">"Dikaiosynē"</i>
                        " est une vertu de la cité, une qualité morale du sujet. "
                        <Terme lemme="tsedeq">"Tsedeq"</Terme>
                        " est un état du monde\u{202f}: l'ordre juste, ce qui est ajusté. Tout "
                        "le droit divin hébraïque se lit de travers dès qu'on remplace un état "
                        "de l'ordre par une qualité de la personne."
                    </p>
                </Correspondance>

                <Correspondance
                    hebreu="חַטָּאת" translitteration="chattah"
                    sens="la déviation — de chata, rater sa cible"
                    grec="ἁμαρτία" grec_translitteration="hamartia"
                    francais="péché"
                >
                    <p>
                        "L'hébreu décrit une trajectoire qui manque sa marque. Le français "
                        "hérité décrit une faute morale, une souillure, une dette. On passe "
                        "d'un écart fonctionnel à une culpabilité subjective — catégorie que "
                        "le monde hébreu antique n'a pas."
                    </p>
                </Correspondance>

                <Correspondance
                    hebreu="מַלְאָךְ" translitteration="mal'akh"
                    sens="l'envoyé mandaté — défini par sa mission, non par sa nature"
                    grec="ἄγγελος" grec_translitteration="angelos"
                    francais="ange"
                >
                    <p>
                        "Le mot grec voulait dire «\u{202f}messager\u{202f}», ce qui était "
                        "juste. Puis il s'est figé en nom d'espèce\u{202f}: une créature avec "
                        "une nature, un rang, des ailes. Retour exact au geste grec — la "
                        "fonction devient un être."
                    </p>
                </Correspondance>
            </Correspondances>

            <p class="mt-14">
                "Neuf mots, et ce ne sont pas les seuls. "
                <Terme lemme="ruach">"Ruach"</Terme>
                ", "
                <Terme lemme="berith">"berith"</Terme>
                ", "
                <Terme lemme="kohen">"kohen"</Terme>
                ", "
                <Terme lemme="chesed">"chesed"</Terme>
                ", "
                <Terme lemme="mishpat">"mishpat"</Terme>
                " ont suivi le même chemin. Voilà pourquoi l'ONT en laisse un certain nombre "
                "debout, en hébreu, plutôt que de choisir entre deux mots faux."
            </p>
            <p class="mt-8">
                <Lien href="/fr/lexique">"Les cent cinq intraduisibles, avec leur champ complet"</Lien>
            </p>
        </Bloc>

        // ── VII ───────────────────────────────────────────────────────────
        <Bloc eclaire=true>
            <TitreDeSection numero="IX" titre="Le texte nomme lui-même le geste" />

            <p class="lettrine">
                "Plus troublant encore que l'histoire de la Septante\u{202f}: le geste qui fonde "
                "l'ontologie grecque est décrit dans le corpus lui-même, au deuxième "
                "chapitre, et il y est décrit comme une rupture."
            </p>

            {citer("Bereshit 2:6", "bereshit", "bereshit-2", vec![6])}

            <p>
                "Il faut savoir ce que "
                <i>"connaître"</i>
                " veut dire en hébreu. "
                <i>"Yada"</i>
                " est toujours participatif et engageant\u{202f}: on ne connaît pas une chose "
                "en la regardant du dehors, on la connaît en étant dedans. L'adam possède "
                "déjà cette "
                <i>"da'at"</i>
                " — c'est depuis elle qu'il nomme les vivants, depuis l'intérieur de l'ordre."
            </p>
            <p>
                "Ce que l'arbre propose n'est donc pas un supplément de connaissances. C'est "
                "une autre manière de connaître\u{202f}: se placer "
                <i>"au-dessus"</i>
                " de l'ordre pour le juger de façon autonome, au lieu d'y participer. "
                "Séparer, catégoriser, statuer depuis une position surplombante."
            </p>

            <Exergue>
                "Le regard qui se met au-dessus du réel pour le découper en catégories — "
                "c'est l'arbre, et c'est le père de l'ontologie grecque."
            </Exergue>

            <p>
                "Le corpus ne présente pas cela comme un progrès de la pensée. Il le présente "
                "comme la fracture d'un cosmos intégré, où tout était "
                <Terme lemme="tov">"tov"</Terme>
                " parce que tout tenait sa place. Restituer l'ontologie hébraïque, ce n'est "
                "donc pas préférer une école à une autre. C'est revenir en deçà de la "
                "fracture."
            </p>
        </Bloc>

        // ── VIII ──────────────────────────────────────────────────────────
        <Bloc large=true>
            <TitreDeSection
                numero="X"
                titre="«\u{202f}Créer\u{202f}» ne veut pas dire fabriquer"
            />

            <p class="lettrine">
                "Prenez la phrase la plus connue de la littérature mondiale. "
                "«\u{202f}Au commencement, Dieu créa les cieux et la terre\u{202f}» suppose un "
                "atelier, de la matière première, un avant et un après. Rien de tout cela "
                "n'est dans le verbe hébreu."
            </p>

            <Comparaison
                renvoi="Bereshit 1:1"
                classique=SEGOND_1910
                source=SEGOND_SOURCE
                ont=bereshit_1_1()
            />

            <p class="mt-10">
                "L'hébreu distingue trois verbes là où le français n'en a qu'un, et la "
                "distinction est ontologique, pas stylistique."
            </p>

            <ul class="mt-8 m-0 list-none p-0">
                <li class="border-t border-filet py-5 first:border-t-0">
                    <Terme lemme="bara">"Bara"</Terme>
                    " — orchestrer. Inaugurer dans l'existence fonctionnelle, attribuer des "
                    "rôles. Son sujet est toujours "
                    <Terme lemme="elohim">"Elohim"</Terme>
                    ", et jamais aucune matière première n'est mentionnée."
                </li>
                <li class="border-t border-filet py-5">
                    <Terme lemme="asah">"Asah"</Terme>
                    " — mettre en place. La dimension structurelle de la parole\u{202f}: sa "
                    "réalisation concrète."
                </li>
                <li class="border-t border-filet py-5">
                    <Terme lemme="yatsar">"Yatsar"</Terme>
                    " — façonner. Le verbe du potier, matériel et artisanal, toujours suivi "
                    "d'une matière première."
                </li>
            </ul>

            <p class="mt-10">
                <Terme lemme="bara">"Bara"</Terme>
                " n'est pas un acte d'artisan. C'est un acte de roi\u{202f}: inaugurer un "
                "espace, attribuer des rôles, mettre en fonction. Le cosmos ne sort pas d'une "
                "usine — il est inauguré comme on inaugure un Temple. Et un Temple commence à "
                "exister le jour où l'on y entre pour y résider."
            </p>

            <Exergue>
                "Le récit ne raconte pas la fabrication de la matière. "
                "Il raconte la mise en ordre d'un sanctuaire."
            </Exergue>

            <p>
                "Ce n'est pas une intuition isolée. C'est le paradigme du Temple cosmique, "
                "documenté dans tout le Proche-Orient ancien, et travaillé notamment par "
                "John\u{a0}H.\u{a0}Walton dans "
                <i>"The Lost World of Genesis One"</i>
                ". Les récits d'inauguration de temples de la région suivent la même "
                "structure\u{202f}: sept jours, l'installation des fonctions, puis la venue "
                "de la divinité qui y prend son repos. Le septième jour n'est pas une "
                "récupération après l'effort — c'est l'entrée en résidence."
            </p>
        </Bloc>

        // ── IX ────────────────────────────────────────────────────────────
        <Bloc>
            <TitreDeSection numero="XI" titre="Ce qui distingue l'être humain" />

            <p class="lettrine">
                "Demandez à n'importe qui ce qui sépare l'homme de l'animal, et vous "
                "entendrez\u{a0}: l'âme. C'est une réponse grecque, et le texte dit "
                "exactement le contraire."
            </p>
            <p>
                "Le "
                <Terme lemme="nefesh">"nefesh"</Terme>
                " est commun. Les poissons, les oiseaux, les bêtes sont des "
                <Terme lemme="nefesh">"nefesh"</Terme>
                " vivants, dans les mêmes termes que l'humain. Ce n'est donc pas là que "
                "passe la ligne."
            </p>

            {citer("Bereshit 1:26", "bereshit", "bereshit-1", vec![26])}

            <p>
                "Elle passe par le "
                <Terme lemme="tselem">"tselem"</Terme>
                ". Dans tout le Proche-Orient ancien, un "
                <i>"tselem"</i>
                " est la statue d'un roi, dressée dans une province qu'il ne peut pas "
                "gouverner en personne\u{202f}: elle y tient sa présence et son autorité. "
                "L'être humain est le "
                <Terme lemme="tselem">"tselem"</Terme>
                " d'"
                <Terme lemme="elohim">"Elohim"</Terme>
                " sur la Terre."
            </p>

            <Principe chute=true>
                "Ce qui distingue l'humain n'est pas une substance immatérielle logée en lui. "
                "C'est une charge\u{a0}: il est le vice-roi du cosmos, et il en répond."
            </Principe>

            <p class="mt-10">
                "Le déplacement est considérable. Dans le cadre grec, la dignité humaine "
                "tient à ce que l'homme "
                <i>"est"</i>
                " — une nature supérieure. Dans le cadre hébreu, elle tient à ce qu'il a "
                <i>"reçu"</i>
                " — un mandat. La première se possède. La seconde s'exerce, et peut se "
                "trahir."
            </p>
        </Bloc>

        // ── X ─────────────────────────────────────────────────────────────
        <Bloc eclaire=true large=true>
            <TitreDeSection numero="XII" titre="Comment on restitue" />

            <p class="lettrine">
                "Restituer un cadre demande plus qu'un bon vocabulaire. Le lecteur français "
                "ne possède pas les réalités hébraïques en tête, et une traduction sur une "
                "seule ligne doit choisir entre dire le texte et l'expliquer. L'ONT refuse ce "
                "choix\u{202f}: elle sépare les niveaux au lieu de les fondre."
            </p>

            <LegendeNiveaux />

            <p class="mt-14">
                "Quatre règles gouvernent ce que ces niveaux ont le droit de porter, et ce "
                "sont elles qui empêchent la restitution de devenir un commentaire."
            </p>

            <ul class="mt-8 m-0 list-none p-0">
                <li class="border-t border-filet py-6 first:border-t-0">
                    <p class="m-0 font-titre text-lg text-encre-vive">"On explicite, on n'invente jamais"</p>
                    <p class="m-0 mt-2">
                        "La glose ne dit que ce que le champ sémantique hébreu portait pour "
                        "son lecteur d'origine. Elle rend explicite un implicite\u{202f}; elle "
                        "n'ajoute rien. C'est la tradition des Targoums, pas celle du "
                        "commentaire."
                    </p>
                </li>
                <li class="border-t border-filet py-6">
                    <p class="m-0 font-titre text-lg text-encre-vive">"Aucune influence extérieure"</p>
                    <p class="m-0 mt-2">
                        "Aucune catégorie théologique protestante ou catholique, aucune "
                        "catégorie philosophique grecque, aucune catégorie morale moderne "
                        "n'entre dans une glose. Chacune se fonde exclusivement sur la "
                        "sémantique hébraïque, le contexte proche-oriental et la logique "
                        "fonctionnelle du cosmos. Et la règle a un corollaire "
                        "opératoire\u{202f}: si une explication exige une catégorie "
                        "extérieure, c'est le signal qu'elle est fausse."
                    </p>
                </li>
                <li class="border-t border-filet py-6">
                    <p class="m-0 font-titre text-lg text-encre-vive">"On ne résout pas ce que le texte ne résout pas"</p>
                    <p class="m-0 mt-2">
                        "Quand une construction hébraïque est structurellement ambiguë, "
                        "l'ambiguïté est une information. Les traductions tranchent, souvent "
                        "sans le dire. L'ONT restitue les lectures disponibles et laisse "
                        "ouvert ce que l'hébreu laisse ouvert."
                    </p>
                </li>
                <li class="border-t border-filet py-6">
                    <p class="m-0 font-titre text-lg text-encre-vive">"On signale la démythologisation"</p>
                    <p class="m-0 mt-2">
                        "Le texte hébreu réduit systématiquement les divinités voisines à des "
                        "instruments fonctionnels. Le soleil et la lune ne sont pas nommés — "
                        "ce sont deux luminaires. Les dragons des eaux, divinités du chaos "
                        "partout ailleurs, sont ici des vivants parmi d'autres, sans combat. "
                        "Un lecteur moderne ne peut pas voir ces refus\u{202f}: ils sont "
                        "signalés."
                    </p>
                </li>
            </ul>

            <p class="mt-12">
                "S'y ajoute une décision qui surprend souvent\u{202f}: les noms propres "
                "restent hébreux. " <Nom>"Qayin"</Nom> " et non Caïn, " <Nom>"Noach"</Nom>
                " et non Noé, " <Nom>"Mitsrayim"</Nom> " et non Égypte. Ce n'est pas une coquetterie\u{202f}: un nom hébreu est "
                "sémantiquement chargé, son étymologie fait partie du récit, et le texte joue "
                "constamment dessus. Traduire le nom efface l'argument."
            </p>
        </Bloc>

        // ── XI ────────────────────────────────────────────────────────────
        <Bloc>
            <TitreDeSection numero="XIII" titre="Pourquoi certains mots restent debout" />

            <p class="lettrine">
                "C'est le reproche le plus fréquent, et il est légitime\u{a0}: laisser des "
                "mots en hébreu ressemble à de l'obscurantisme, ou à une façon de se rendre "
                "intéressant. La réponse tient en une observation."
            </p>
            <p>
                "Un intraduisible n'est pas un mot difficile. C'est un mot dont chaque "
                "équivalent français disponible est "
                <i>"faux"</i>
                " — pas approximatif\u{202f}: faux, parce qu'il importe une ontologie "
                "étrangère. La section VIII le montre neuf fois. Devant deux mots faux, on peut "
                "en choisir un et taire le problème, ou laisser le mot debout et donner au "
                "lecteur son champ complet."
            </p>
            <p>
                "L'ONT fait le second choix, et elle tient l'engagement qui va avec\u{202f}: "
                "chaque intraduisible est en or dans le texte, et chaque mot d'or mène à sa "
                "fiche. L'or promet une explication, et il la tient. C'est la différence "
                "entre laisser un mot en hébreu et abandonner le lecteur devant."
            </p>
            <p>
                "Rien n'est d'ailleurs perdu\u{202f}: le mot reste, sa translittération et "
                "son hébreu sont donnés, et la glose dit ce qu'il porte. Le lecteur reçoit "
                "davantage que dans une traduction ordinaire — il reçoit aussi de quoi "
                "vérifier."
            </p>
        </Bloc>

        // ── XII ───────────────────────────────────────────────────────────
        <Bloc eclaire=true large=true>
            <TitreDeSection numero="XIV" titre="Ce qui entre dans le corpus" />

            <p class="lettrine">
                "Le même critère décide de tout, y compris de ce qu'on traduit. L'ONT ne se "
                "limite pas à la Bible canonique, parce que le canon est une construction "
                "tardive — quatrième siècle et après — que les auteurs des textes n'ont jamais "
                "connue. Le projet travaille sur la bibliothèque d'un Juif lettré du Second "
                "Temple."
            </p>
            <p>
                "Le filtre n'est donc pas canonique. Il est ontologique, et il tient en une "
                "question\u{a0}: "
                <i>"ce texte pense-t-il en hébreu, ou en grec\u{202f}?"</i>
                " Entre tout texte hébreu ou araméen antique qui éclaire le cosmos hébreu "
                "depuis l'intérieur. Sort tout texte qui a absorbé les catégories "
                "hellénistiques — fût-il d'auteur juif, comme Philon d'Alexandrie ou la "
                <i>"Sagesse de Salomon"</i>
                ". La distinction canon\u{202f}/\u{202f}apocryphe n'existe pas dans l'ONT."
            </p>
            <p>
                "Le corpus porte un nom hébreu, la "
                <Nom>"Kenesset"</Nom>
                " — le rassemblement — et il est ordonné en quatre modes qui ne sont pas des "
                "divisions canoniques mais des manières distinctes d'engager le réel\u{202f}: "
                "la "
                <Nom>"Torah"</Nom>
                " institue, les "
                <Nom>"Nevi'im"</Nom>
                " (les Prophètes) lisent l'alliance dans l'histoire, les "
                <Nom>"Ketouvim"</Nom>
                " (les Écrits) habitent l'expérience, et les "
                <Nom>"Nistarot"</Nom>
                " — les réalités voilées, que ce découpage est seul à nommer — traversent "
                "les structures invisibles. Là encore, c'est le principe de "
                <Terme lemme="qara">"qara"</Terme>
                " appliqué\u{202f}: si une distinction fonctionnelle est réelle, elle mérite "
                "un nom."
            </p>

            <Exergue>
                "Ramener ce qui a été perdu depuis d'anciens temps — "
                "les choses dont le monde n'a même pas conscience qu'il ne sait pas."
            </Exergue>

            <p>
                "Voilà le pourquoi. Ce n'est pas une traduction de plus, et ce n'est pas un "
                "exercice d'histoire. C'est une restitution\u{202f}: remettre le cadre "
                "d'aplomb, et laisser le texte dire ce qu'il a toujours dit."
            </p>

            <div class="mt-12 flex flex-wrap gap-4">
                <Lien href="/fr/lire">"Entrer dans le corpus"</Lien>
                <Lien href="/fr/ce-que-l-ont-n-est-pas">"Ce que l'ONT n'est pas"</Lien>
            </div>
        </Bloc>
    }
}

/// Un ou plusieurs versets du corpus, cités dans la page.
///
/// Le rendu du serveur les porte (`new_blocking`) : une démonstration dont la
/// pièce à conviction arrive après coup n'est une démonstration ni pour un
/// moteur de recherche, ni pour un lecteur sans JavaScript.
///
/// Une citation qui ne se charge pas **se tait** au lieu de casser la page.
/// C'est le même arbitrage que le verset du jour de l'accueil : la prose autour
/// reste vraie sans elle, et une page amputée d'un exemple vaut mieux qu'une
/// page d'erreur.
fn citer(
    renvoi: &'static str,
    livre: &'static str,
    unite: &'static str,
    numeros: Vec<u32>,
) -> impl IntoView {
    let source = Resource::new_blocking(
        || (),
        move |_| {
            let numeros = numeros.clone();
            async move { versets(livre.to_string(), unite.to_string(), numeros).await }
        },
    );

    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                match source.await {
                    Ok(v) if !v.is_empty() => {
                        view! {
                            <Citation
                                renvoi=renvoi
                                chemin=format!("/fr/lire/{livre}/{unite}")
                                versets=v
                            />
                        }
                            .into_any()
                    }
                    _ => ().into_any(),
                }
            })}
        </Suspense>
    }
}

/// Les versets que la page cite existent dans le corpus.
///
/// `citer` **se tait** quand elle ne trouve rien — c'est le bon comportement en
/// production, où une citation manquante vaut mieux qu'une page d'erreur. Mais
/// c'est aussi le pire des silences : le jour où le vault renumérote une unité
/// ou renomme un chapitre, la démonstration perd sa pièce et la prose continue
/// d'annoncer un verset qui n'apparaît plus. Personne ne compare la longueur
/// d'une page d'une semaine à l'autre.
///
/// Ce test rend la disparition bruyante. Il est le pendant de
/// `chaque_livre_embarque_s_analyse` : ce qu'une porte de sortie avale à
/// l'exécution, un test doit le rattraper à la compilation.
#[cfg(all(test, feature = "ssr"))]
mod tests {
    use crate::application::ports::Corpus;
    use crate::infrastructure::corpus::CorpusEmbarque;

    /// Les renvois cités par la page, et les versets qu'ils promettent.
    const CITATIONS: &[(&str, &str, &[u32])] = &[
        ("bereshit", "bereshit-1", &[2, 10, 26]),
        ("bereshit", "bereshit-2", &[6]),
    ];

    #[test]
    fn les_versets_cites_existent() {
        let corpus = CorpusEmbarque::charger().expect("le corpus s'ouvre");

        for (livre, unite, numeros) in CITATIONS {
            let ouvrage = corpus
                .livre(livre)
                .unwrap_or_else(|| panic!("« {livre} » a disparu du corpus"));
            let chapitre = ouvrage
                .chapitre(unite)
                .unwrap_or_else(|| panic!("« {unite} » a disparu de « {livre} »"));

            for n in *numeros {
                assert!(
                    chapitre.verset(*n).is_some(),
                    "« Le pourquoi » cite {unite}:{n}, qui n'existe plus — \
                     la citation se tairait sans que rien ne le dise"
                );
            }
        }
    }
}
