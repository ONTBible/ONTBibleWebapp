//! Les réglages de lecture — éteindre les niveaux du texte.
//!
//! ## Ce n'est pas une préférence d'affichage
//!
//! Le portage reprend le mot de l'app, dans `ONTKit/Reader/Reader.swift` :
//!
//! > Les deux premiers champs ne sont pas des préférences d'affichage : ce sont
//! > les **niveaux du texte**, et pouvoir les éteindre est la raison d'être de
//! > la liseuse.
//!
//! Le corps de la traduction ne s'éteint jamais. Ce qui s'éteint, c'est
//! l'appareil critique : la glose qui explicite l'implicite hébreu, et le mot
//! original avec sa translittération.
//!
//! ## Ce qui n'est **pas** réglable, et pourquoi
//!
//! Ni les intraduisibles, ni les accentuations. L'app n'en offre pas la
//! bascule non plus, et son moteur de rendu dit pourquoi :
//!
//! > Une accentuation survit à l'extinction des niveaux : elle appartient au
//! > corps, pas à l'appareil critique.
//!
//! Un intraduisible n'est pas un commentaire ajouté au texte — c'est le texte,
//! qu'on a refusé de traduire. L'éteindre ne rendrait pas la lecture plus
//! simple : il laisserait un trou. Et la promesse de l'or — une fiche, et le
//! lien la tient — cesserait d'être tenue selon un réglage.
//!
//! ## Le nettoyage, qui est le vrai travail
//!
//! Retirer un nœud laisse ses blancs derrière lui. Une glose se pose **après**
//! le mot qu'elle éclaire et **avant** la ponctuation qui suit : l'ôter donne
//! « habitant , et la face ». Une translittération est encadrée d'espaces :
//! l'ôter en laisse deux. C'est le même défaut que celui déjà corrigé sur les
//! aperçus de messagerie, et il se corrige au même endroit.

use serde::{Deserialize, Serialize};

use crate::domaine::texte::Noeud;

/// La peau de la page — les quatre thèmes de la liseuse de l'app.
///
/// ## Le site en offrait zéro, et c'était écrit
///
/// Le `CLAUDE.md` disait, au §8 bis : « Ce que le site n'emprunte pas : taille
/// du corps, interligne, fonte, **thème**. L'app a raison de les offrir — elle
/// est un lecteur, et un lecteur s'adapte à qui le tient. Le site est une
/// **édition** : sa nuit d'aubergine est une décision, pas un défaut qu'on
/// propose de corriger. »
///
/// **L'auteur a tranché autrement le 21 septembre 2026** : « je veux que la
/// webapp soit identique en tout point à l'app iOS — icône, design system, DA,
/// feature, tout. Je veux que l'user ait l'app iOS, mais en webapp. » Le thème
/// est une feature de la liseuse ; il entre.
///
/// Ce que la décision d'avant gardait de vrai est le **défaut** : le site
/// ouvre sur `Mystique` là où l'app ouvre sur `Parchemin`. L'édition a une
/// peau ; le lecteur peut en changer.
///
/// ## Les noms ne se traduisent pas
///
/// Ce sont ceux de l'app — `ReadingTheme` dans `ONTKit/Reader/Reader.swift` —
/// et ils doivent le rester : un lecteur qui passe du téléphone au site doit
/// retrouver les mêmes mots dans le même menu. `mystique` est d'ailleurs né
/// ici et a été transposé là-bas ; c'est le seul des quatre dont ce dépôt soit
/// la source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    /// Le papier de l'app — son défaut à elle, et il ne suit pas le système.
    Parchemin,
    /// Le blanc franc.
    Clair,
    /// Le gris neutre.
    Sombre,
    /// La nuit d'aubergine — la peau de ce site, et son défaut.
    #[default]
    Mystique,
}

/// Les chemins où la peau du lecteur s'applique — **les deux arbres**.
///
/// ## Une table, et non deux qui disent la même chose
///
/// Il y avait ici `LA_LISEUSE = ["/fr/webapp", "/fr/lexique"]`, posée avant que
/// le site ne serve deux arbres. Elle répondait à la même question qu'[`Arbre`]
/// — *sommes-nous dans la liseuse ?* — avec une seconde liste à tenir d'accord.
///
/// ==Deux tables qui disent la même chose finissent par en dire deux
/// différentes.== Celle-ci dérive donc d'`Arbre::TOUS`, et un arbre renommé
/// emporte le thème avec lui.
///
/// La forme de la comparaison ne change pas : égalité, ou préfixe **suivi d'une
/// barre**. Le piège est le mot voisin — `/fr/webappnt-ils` n'est pas sous
/// `/fr/webapp`, et un `starts_with` nu le dirait.
pub fn c_est_la_liseuse(chemin: &str) -> bool {
    Arbre::du_chemin(chemin).is_some()
}

impl Theme {
    /// Les bornes du corps, **reprises du curseur de l'app**.
    ///
    /// `Slider(in: 11...28, step: 1)`, défaut 19 — écrites une fois là-bas
    /// dans `TaillesAuClavier.corps`, pour qu'un raccourci ne puisse pas
    /// atteindre une valeur que le curseur refuse. Même raison ici.
    pub const CORPS_MINIMUM: u8 = 11;
    pub const CORPS_MAXIMUM: u8 = 28;
    pub const CORPS_PAR_DEFAUT: u8 = 19;

    /// Les bornes de l'interligne — celles de l'app, `0,2...1,0` par pas de
    /// 0,1. Le défaut est 0,5, et il rend les 1,68 que le §5 a mesurés.
    pub const INTERLIGNE_MINIMUM: u8 = 2;
    pub const INTERLIGNE_MAXIMUM: u8 = 10;
    pub const INTERLIGNE_PAR_DEFAUT: u8 = 5;

    /// Les quatre, dans l'ordre du menu de l'app.
    pub const TOUS: [Theme; 4] = [
        Theme::Parchemin,
        Theme::Clair,
        Theme::Sombre,
        Theme::Mystique,
    ];

    /// Ce que porte `<html data-theme="…">`, et donc le sélecteur des jetons.
    ///
    /// **La même chaîne sert des deux côtés du fil** : elle voyage dans le
    /// JSON du navigateur *et* dans l'attribut que lit `jetons.css`. Deux
    /// tables — une pour sérialiser, une pour l'attribut — finiraient par
    /// diverger sur le jour où l'on renommerait un thème.
    pub fn attribut(self) -> &'static str {
        match self {
            Theme::Parchemin => "parchemin",
            Theme::Clair => "clair",
            Theme::Sombre => "sombre",
            Theme::Mystique => "mystique",
        }
    }

    /// Le nom dans le menu — celui de l'app, capitale comprise.
    pub fn libelle(self) -> &'static str {
        match self {
            Theme::Parchemin => "Parchemin",
            Theme::Clair => "Clair",
            Theme::Sombre => "Sombre",
            Theme::Mystique => "Mystique",
        }
    }
}

/// La fonte du corps — **les sept de l'app**, `ReadingFont`.
///
/// ## Pourquoi sept et pas une
///
/// Le site n'en offrait aucune, et le `CLAUDE.md` le défendait : « le site est
/// une **édition** : sa nuit d'aubergine, son corps à 21 px et sa Literata sont
/// des décisions, pas des défauts qu'on propose de corriger ».
///
/// L'auteur a tranché autrement le 21 septembre 2026 — « identique en tout
/// point à l'app iOS, feature comprise ». Et sur ce point-ci la décision
/// d'avant était plus fragile qu'elle n'en avait l'air : **une fonte n'est pas
/// un goût quand on lit mal.** L'œil qui bute sur une romane à fort contraste
/// ne bute pas sur une linéale, et c'est mesurable sur la vitesse de lecture,
/// pas sur l'opinion.
///
/// Ce qui reste vrai est le **défaut** : Literata, ici comme là-bas.
///
/// ## Six embarquées, une du système
///
/// Georgia est fournie par la plateforme des deux côtés. L'app le note — « on
/// ne contrôle pas ses fichiers » —, et un navigateur l'a aussi. Les six autres
/// voyagent en woff2 par `scripts/fontes.sh`, **en trois coupes chacune** :
/// Regular, Italic, SemiBold.
///
/// Les trois comptent, et l'app dit pourquoi : *une famille amputée de son
/// italique se résout quand même, en pente simulée, penchée à la main par le
/// moteur de rendu.* Chez nous c'est pire — la translittération du niveau 3
/// **est** en italique, donc une famille incomplète abîme précisément la pièce
/// que la liseuse existe pour montrer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Fonte {
    /// Dessinée par TypeTogether pour Google Play Books — et déjà le corps de
    /// l'app comme du site.
    #[default]
    Literata,
    EbGaramond,
    Spectral,
    SourceSerif,
    Newsreader,
    Jost,
    Georgia,
}

impl Fonte {
    /// Les sept, dans l'ordre du menu de l'app.
    pub const TOUTES: [Fonte; 7] = [
        Fonte::Literata,
        Fonte::EbGaramond,
        Fonte::Spectral,
        Fonte::SourceSerif,
        Fonte::Newsreader,
        Fonte::Jost,
        Fonte::Georgia,
    ];

    /// Ce que porte `<html data-fonte="…">`, et donc le sélecteur de la feuille.
    ///
    /// La même chaîne sert des deux côtés du fil — elle voyage dans le JSON du
    /// navigateur *et* dans l'attribut que lit la feuille. Deux tables
    /// finiraient par diverger le jour d'un renommage.
    pub fn attribut(self) -> &'static str {
        match self {
            Fonte::Literata => "literata",
            Fonte::EbGaramond => "eb-garamond",
            Fonte::Spectral => "spectral",
            Fonte::SourceSerif => "source-serif",
            Fonte::Newsreader => "newsreader",
            Fonte::Jost => "jost",
            Fonte::Georgia => "georgia",
        }
    }

    /// Le nom dans le menu — celui de l'app, à la lettre.
    pub fn libelle(self) -> &'static str {
        match self {
            Fonte::Literata => "Literata",
            Fonte::EbGaramond => "EB Garamond",
            Fonte::Spectral => "Spectral",
            Fonte::SourceSerif => "Source Serif",
            Fonte::Newsreader => "Newsreader",
            Fonte::Jost => "Jost",
            Fonte::Georgia => "Georgia",
        }
    }

    /// Ce que la fonte apporte, en une ligne — **les phrases de l'app**.
    ///
    /// Elles sont reprises mot pour mot, et c'est délibéré : un lecteur qui
    /// passe du téléphone au site doit retrouver les mêmes mots pour choisir la
    /// même chose. L'app les introduit ainsi — « de quoi choisir sans être
    /// typographe ».
    pub fn note(self) -> &'static str {
        match self {
            Fonte::Literata => "Dessinée pour la lecture longue à l'écran",
            Fonte::EbGaramond => "La lettre du livre imprimé classique",
            Fonte::Spectral => "Ouverte et franche, tient les petites tailles",
            Fonte::SourceSerif => "Neutre, elle s'efface derrière le texte",
            Fonte::Newsreader => "Étroite, plus de texte par écran",
            Fonte::Jost => "Géométrique — la fonte de l'édition imprimée",
            Fonte::Georgia => "La fonte du système, robuste et familière",
        }
    }
}

/// **Où l'on est** — l'arbre d'adresses, et non ce que le lecteur préfère.
///
/// ## Deux notions que le 30 septembre a séparées
///
/// Le site sert deux produits sous deux racines. `Arbre` est un **fait** : il se
/// lit dans le chemin, il n'a pas de défaut, et il décide du chrome rendu comme
/// de la forme des liens qu'on pose. [`Habillage`] est un **souhait** : ce que le
/// lecteur a demandé, qui peut diverger de l'arbre où il vient d'atterrir.
///
/// ```text
/// /fr/liseuse/…   l'édition : en-tête du site, nav horizontale, pied de page
/// /fr/webapp/…    l'app : barre latérale, barre d'onglets, corpus épinglé
/// ```
///
/// Quand les deux divergent — un lien vers la liseuse ouvert sur un téléphone
/// dont la préférence dit « l'app » —, ==c'est l'adresse qui s'aligne, jamais le
/// rendu qui se tait==. Une URL qui affiche autre chose que ce qu'elle nomme est
/// une URL qui ment : on la copie, on la repartage, on la met en signet.
// `Ord` parce qu'une épreuve range les deux arbres dans une table pour comparer
// leurs routes terme à terme. L'ordre est celui de la déclaration — la liseuse
// d'abord, qui est la canonique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Arbre {
    /// L'édition — la forme lisible sans JavaScript, et celle qui s'indexe.
    #[default]
    Liseuse,
    /// L'app — les barres, le corpus, les onglets.
    Webapp,
}

impl Arbre {
    /// Les deux, dans l'ordre de leur préséance.
    pub const TOUS: [Arbre; 2] = [Arbre::Liseuse, Arbre::Webapp];

    /// L'arbre qui s'indexe — **l'adresse officielle du texte**.
    ///
    /// Arbitré le 30 septembre 2026. C'est la liseuse, pour une raison qui n'est
    /// pas un goût : elle est la forme lisible **sans JavaScript**, celle qu'un
    /// moteur reçoit entière au premier octet. Les pages de l'autre arbre
    /// portent un `rel="canonical"` vers leur jumelle, et le plan du site ne
    /// déclare que celle-ci.
    pub const CANONIQUE: Arbre = Arbre::Liseuse;

    /// Le segment de chemin, sans barres — « liseuse » ou « webapp ».
    ///
    /// C'est **la seule table** : les routes, les composeurs de chemins et le
    /// script de l'en-tête la citent, jamais un littéral recopié. Un renommage
    /// se fait ici et se propage ; recopié, il laisserait des pages joignables
    /// par une adresse que plus rien n'engendre.
    pub fn segment(self) -> &'static str {
        match self {
            Arbre::Liseuse => "liseuse",
            Arbre::Webapp => "webapp",
        }
    }

    /// La racine, barre comprise — `/fr/liseuse` ou `/fr/webapp`.
    ///
    /// Sans barre finale : elle se concatène avec une suite qui commence par la
    /// sienne, de sorte que la racine seule reste une adresse valide.
    pub fn racine(self) -> String {
        format!("/fr/{}", self.segment())
    }

    /// L'arbre que désigne un chemin, s'il en désigne un.
    ///
    /// **Même forme que [`c_est_la_liseuse`]** — égalité, ou préfixe suivi d'une
    /// barre. Le piège est le mot voisin : `/fr/webappnt-ils` n'est pas sous
    /// `/fr/webapp`, et un simple `starts_with` le dirait.
    pub fn du_chemin(chemin: &str) -> Option<Arbre> {
        let chemin = chemin.trim_end_matches('/');
        Arbre::TOUS.into_iter().find(|arbre| {
            let racine = arbre.racine();
            chemin == racine || chemin.starts_with(&format!("{racine}/"))
        })
    }

    /// L'autre — celui vers lequel on bascule.
    pub fn autre(self) -> Arbre {
        match self {
            Arbre::Liseuse => Arbre::Webapp,
            Arbre::Webapp => Arbre::Liseuse,
        }
    }
}

/// L'habillage de la liseuse — **l'app, ou l'édition**.
///
/// ## Deux produits, et l'auteur a refusé de choisir
///
/// Le 21 septembre 2026 il a demandé une webapp *identique en tout point* à
/// l'app iOS. Le 30, il a regardé les deux côte à côte et dit l'inverse :
/// « finalement la liseuse est peut-être mieux ». Les deux jugements sont
/// justes, et ils ne portent pas sur la même chose.
///
/// Ce qui les distingue **n'est pas la composition du texte**, et c'est le fait
/// qui a tranché : les deux rendent la même mesure de 38 rem, la même fonte, la
/// même règle typographique. Mesuré sur les deux HTML servis le 30 septembre.
/// Ce qui change est l'**habillage** — tout ce qui entoure le texte sans en
/// faire partie :
///
/// ```text
/// l'app        barre des cinq onglets, barre latérale du corpus,
///              capsules de verre flottantes, barre collante
/// l'édition    l'en-tête du site, son pied de page, le fil d'Ariane seul,
///              et le bouton « aA » flottant en bas
/// ```
///
/// ## Le défaut reste l'app, et ce n'est pas un arbitrage de plus
///
/// C'est celui qu'il a demandé, et un lecteur qui vient du téléphone doit
/// retrouver ses gestes sans rien régler. L'édition est **un choix**, et un
/// choix se pose — il ne se subit pas.
///
/// ## Une préférence, pas deux adresses
///
/// Deux jeux d'adresses auraient dédoublé les cent soixante-trois pages du
/// corpus dans les index, et forcé chaque lien partagé à trancher pour son
/// destinataire. ==Un habillage est une façon de regarder, pas un lieu.== Il
/// vit donc avec les autres réglages, dans la même clé du stockage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
// La valeur retenue dans le stockage est celle que le script lit pour poser le
// cookie, donc elle ne se traduit nulle part. Comme `Theme`.
#[serde(rename_all = "lowercase")]
pub enum Habillage {
    /// **L'appareil décide** — grand écran l'édition, petit écran l'app.
    ///
    /// C'est le défaut, et ce n'est pas un compromis : les deux chromes ont été
    /// dessinés chacun pour une taille. La barre latérale du corpus demande
    /// seize rem et demie qu'un téléphone n'a pas ; l'en-tête centré du site
    /// suppose une largeur où un titre et cinq entrées tiennent sur une ligne.
    #[default]
    Auto,
    /// L'édition partout, quel que soit l'appareil.
    Edition,
    /// L'app partout.
    Application,
}

impl Habillage {
    /// Les trois, dans l'ordre où le sélecteur les pose.
    ///
    /// `Auto` en tête : c'est le défaut, et une liste qui commence par une
    /// exception apprend l'exception avant la règle.
    pub const TOUS: [Habillage; 3] = [Habillage::Auto, Habillage::Edition, Habillage::Application];

    /// La valeur retenue, telle que le script la relit.
    ///
    /// **Ce n'est plus un attribut CSS.** Il y a eu, le 30 septembre au soir, un
    /// montage où les deux chromes vivaient dans le même document et où un
    /// `data-habillage` en peignait un. Il a été retiré le même soir : `display:
    /// none` cache la peinture, il n'annule pas le travail — la barre latérale
    /// lançait ses deux requêtes et sérialisait le corpus entier, pour un chrome
    /// que le lecteur ne voyait pas. Cent soixante-six kilo-octets par page, et
    /// le coût croît avec le corpus.
    pub fn cle(self) -> &'static str {
        match self {
            Habillage::Auto => "auto",
            Habillage::Edition => "edition",
            Habillage::Application => "application",
        }
    }

    /// L'arbre que ce souhait désigne, la largeur de l'écran étant connue.
    ///
    /// C'est la **seule** fonction qui traduit un souhait en fait, et le script
    /// de l'en-tête la rejoue en JavaScript — l'épreuve `le_script_resout_l_auto`
    /// tient les deux d'accord.
    pub fn resoudre(self, grand_ecran: bool) -> Arbre {
        match self {
            Habillage::Edition => Arbre::Liseuse,
            Habillage::Application => Arbre::Webapp,
            Habillage::Auto if grand_ecran => Arbre::Liseuse,
            Habillage::Auto => Arbre::Webapp,
        }
    }

    /// Le nom qu'en donne le sélecteur.
    pub fn libelle(self) -> &'static str {
        match self {
            Habillage::Auto => "Selon l'appareil",
            Habillage::Application => "L'application",
            Habillage::Edition => "L'édition",
        }
    }

    /// Ce que le lecteur y gagne, en une ligne.
    ///
    /// Les deux notes disent une **conséquence visible**, jamais un nom de
    /// composant : « les onglets en bas » se constate, « BarreDOnglets » ne dit
    /// rien à qui lit.
    pub fn note(self) -> &'static str {
        match self {
            Habillage::Auto => {
                "L'édition sur un grand écran, l'application sur un téléphone. \
                 Chaque chrome a été dessiné pour une taille."
            }
            Habillage::Application => {
                "Les cinq onglets en bas, le corpus dans une barre latérale, \
                 les capsules du haut. Ce que vous avez sur le téléphone."
            }
            Habillage::Edition => {
                "L'en-tête du site et son pied de page, le fil d'Ariane seul, \
                 et rien autour du texte. La liseuse d'avant."
            }
        }
    }
}

/// Ce que le lecteur a choisi de voir.
/// `serde(default)` sur chaque champ, et ce n'est pas une précaution de style :
/// ces valeurs viennent du **stockage du navigateur**, écrit par une version
/// antérieure du site. Le jour où un quatrième réglage apparaît, les réglages
/// déjà retenus n'en portent pas la clé — sans cette tolérance, ils seraient
/// tous jetés d'un coup, et chaque lecteur retrouverait les défauts. L'app fait
/// la même chose, champ par champ, dans son `init(from:)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    /// Niveau 2 — ce que le champ sémantique hébreu portait implicitement.
    pub gloses: bool,
    /// Niveau 3 — la translittération et l'hébreu.
    pub niveau_3: bool,
    /// Les versets coulent en prose, numéros en exposant.
    pub continu: bool,
    /// Nommer les livres et les sections dans le français reçu.
    ///
    /// **Vrai par défaut**, et c'est délibéré : un lecteur qui arrive doit
    /// pouvoir se repérer avec les mots qu'il connaît. « Apocalypse », « la
    /// Loi », « Actes des Apôtres ».
    ///
    /// À faux, il lit ce que le nom ONT veut dire — « le *machazeh* de
    /// Yohanan », « la Fondation », « les *gevurot* de YHWH par ses *neviim* ».
    /// Les intraduisibles y restent en hébreu, là où le français les rend.
    ///
    /// **L'écart entre les deux est le projet lui-même** : *torah*,
    /// l'instruction qui vise, est devenue *nomos*, le code qui contraint. Le
    /// réglage laisse le lecteur passer d'un monde à l'autre au lieu de le lui
    /// raconter.
    pub francais: bool,
    /// La taille du corps de la traduction, **dans l'unité de l'app**.
    ///
    /// ## Deux échelles, et les confondre est un contresens
    ///
    /// L'app en porte deux, et son code dit pourquoi en nommant l'auteur :
    ///
    /// > Un lecteur atteint de kératocône monte le corps du texte très haut
    /// > pour lire, et n'a aucune raison de faire enfler du même geste une
    /// > barre latérale qui lui mangerait la place où ce texte s'affiche.
    ///
    /// | | l'app | le site |
    /// |---|---|---|
    /// | l'interface | ⌘+ / ⌘−, sept crans de 0,85 à 1,50 | le zoom du navigateur |
    /// | le corps du texte | un curseur, 11 à 28 | **ce champ** |
    ///
    /// Le site n'avait que la première, et par chance elle était déjà juste :
    /// la feuille est tout entière en `rem`, donc le zoom et la taille de
    /// police par défaut du navigateur commandent déjà — c'est le rôle que
    /// `@ScaledMetric` tient chez elle. Ce qui manquait est la seconde.
    ///
    /// ## Pourquoi 11 à 28 et pas un pourcentage
    ///
    /// Ce sont **les bornes du curseur de l'app**, `Slider(in: 11...28,
    /// step: 1)`, et son défaut est 19. Garder son unité plutôt qu'un facteur
    /// rend les deux réglages littéralement comparables : un lecteur à 24 sur
    /// son téléphone est à 24 ici, et la même valeur voyage dans le même
    /// champ. Un pourcentage aurait demandé une conversion — donc un endroit
    /// où se tromper, et un jour où les deux divergent.
    ///
    /// Le site n'applique pas ces points tels quels : son corps de lecture
    /// n'est pas celui de l'app, il a été mesuré ici (§5). Ce qui voyage est
    /// le **rapport au défaut** — 19 laisse le site exactement tel qu'il est.
    pub corps: u8,
    /// L'interligne, **dans l'unité de l'app** — `lineSpacing`, de 0,2 à 1,0.
    ///
    /// ## Pourquoi son unité n'est pas celle du web
    ///
    /// SwiftUI compte un **supplément** : `.lineSpacing` s'ajoute à
    /// l'interligne naturel de la fonte, et l'app le multiplie par la taille
    /// du corps. La CSS, elle, compte un **total** — `line-height: 1,68` est
    /// la hauteur entière d'une ligne, fonte comprise.
    ///
    /// Les deux ne se convertissent pas exactement : le supplément naturel de
    /// Literata n'est pas un nombre que ce dépôt connaît. Ce qu'on tient, et
    /// qui suffit, c'est **le défaut et l'amplitude** :
    ///
    /// | | app | site |
    /// |---|---|---|
    /// | défaut | 0,5 | **1,68** — la valeur du §5 |
    /// | plus serré | 0,2 | 1,38 |
    /// | plus aéré | 1,0 | 2,18 |
    ///
    /// Le défaut du site rend donc **exactement** ce qu'il rendait avant ce
    /// réglage, et c'est une condition : le §5 a mesuré 1,68 et dit pourquoi.
    /// Un curseur dont le cran du milieu déplacerait la valeur documentée
    /// changerait la composition de tout le monde pour offrir un réglage à
    /// quelques-uns.
    ///
    /// ## En **dixièmes**, et non en flottant
    ///
    /// L'app le porte en `Double`, mais son curseur avance `step: 0.1` : ce
    /// sont **neuf crans**, pas un continuum. Les compter en entier a trois
    /// effets, et les trois comptent ici :
    ///
    /// - `Preferences` reste `Eq`, donc comparable — un flottant ne l'est pas,
    ///   et c'est ce qui a refusé la première écriture ;
    /// - la sérialisation est exacte : `0.7` ne s'écrit pas en binaire, et un
    ///   aller-retour par `localStorage` peut rendre `0.7000000000000001` ;
    /// - deux réglages identiques se reconnaissent, ce dont un signal a besoin
    ///   pour ne pas se déclencher sur une égalité qui n'en est pas une.
    ///
    /// C'est le même choix que `corps`, qui compte des points entiers.
    pub interligne: u8,
    /// Couper les mots en fin de ligne.
    ///
    /// **Éteint par défaut, comme chez l'app**, et sa raison vaut mot pour
    /// mot ici : *« la césure hache les mots, et qui grossit le texte pour le
    /// voir se retrouve avec plus de coupures, pas moins »*. C'est la phrase
    /// qui décide, sur un site dont l'auteur lit à 0,6/10.
    ///
    /// Le site l'avait posée **globalement** sur `p`, donc allumée pour tout
    /// le monde et sans moyen de l'éteindre. Elle devient un réglage du
    /// lecteur, là où elle change quelque chose — dans le corpus.
    pub coupure: bool,
    /// La peau de la page.
    ///
    /// Elle vit ici, avec les niveaux du texte, parce qu'elle vit **au même
    /// endroit chez le lecteur** : une seule clé dans le stockage du
    /// navigateur, un seul panneau, un seul signal. C'est aussi ce que fait
    /// l'app, dont les réglages de lecture portent son `ReadingTheme`.
    ///
    /// Elle ne traverse pas `depouiller` pour autant : ce n'est pas un niveau
    /// du texte, et rien ne se retire quand elle change.
    pub theme: Theme,
    /// La fonte du corps, choisie par le lecteur.
    pub fonte: Fonte,
    /// L'habillage de la liseuse — les barres de l'app, ou l'édition du site.
    ///
    /// Comme le thème, il ne traverse pas `depouiller` : ce n'est pas un niveau
    /// du texte, et **rien du texte ne change quand il bascule**. C'est ce qui
    /// rend la bascule sûre — elle ne peut pas faire disparaître un mot.
    pub habillage: Habillage,
}

impl Default for Preferences {
    /// Tout est montré, et les versets se tiennent séparés.
    ///
    /// Ce sont les défauts de l'app, repris tels quels — un lecteur qui ouvre
    /// l'un puis l'autre doit voir la même chose. Et c'est le seul défaut
    /// honnête : le site montre d'abord ce que la traduction fait, puis laisse
    /// en retirer.
    fn default() -> Self {
        Self {
            francais: true,
            gloses: true,
            niveau_3: true,
            continu: false,
            theme: Theme::Mystique,
            corps: Theme::CORPS_PAR_DEFAUT,
            interligne: Theme::INTERLIGNE_PAR_DEFAUT,
            coupure: false,
            fonte: Fonte::Literata,
            // **`Auto`, et il a fallu le mesurer pour s'en apercevoir.**
            //
            // Ce champ valait `Application`, alors que le type dit le contraire
            // à deux lignes de sa définition — « `Auto` : c'est le défaut » — et
            // que tout le §8 duodecies en dépend. Un lecteur neuf avait donc
            // « L'application » cochée dans « Vous », jamais « Selon
            // l'appareil » : sur un grand écran, `/fr/liseuse/…` se faisait
            // renvoyer vers la webapp, et la préférence se figeait au premier
            // chargement.
            //
            // Les deux chemins divergeaient en silence : un `ont.lecture`
            // **sans** la clé retombe sur `Habillage::default()`, donc `Auto` ;
            // **aucun** `ont.lecture` passe par ici, donc `Application`. Le même
            // lecteur voyait deux sites selon qu'il avait déjà réglé autre
            // chose.
            //
            // ==Un défaut écrit deux fois est un défaut qu'on peut contredire,
            // et le second exemplaire n'a aucune raison d'être relu.==
            //
            // Trouvé le 2 octobre 2026 en vidant le profil d'un simulateur iPad
            // et en relevant ce que la page réécrit toute seule. L'auteur avait
            // la même lecture : *« j'ai l'impression que par défaut on est
            // application dans les settings et pas en selon appareil »*.
            habillage: Habillage::Auto,
        }
    }
}

impl Preferences {
    /// Le corps seul — ni glose, ni translittération.
    ///
    /// C'est ce qu'emploie une citation hors de la liseuse : un aperçu de
    /// messagerie, une carte de partage. Sortie de son appareil critique, où
    /// elle est consultable et attribuée, une glose devient une affirmation
    /// sans recours.
    ///
    /// L'app a la même chose, et sous le même nom d'intention —
    /// `composeBare` force les deux à faux, indépendamment des réglages.
    pub fn nu() -> Self {
        Self {
            // Le registre reste le français : une citation sortie de son
            // appareil critique doit être **reconnaissable**. « Apocalypse »
            // hors contexte situe le lecteur ; « le machazeh de Yohanan » le
            // laisse devant un mot qu'aucune fiche n'accompagne plus.
            francais: true,
            gloses: false,
            niveau_3: false,
            continu: false,
            // Sans effet ici : ni la peau ni la taille ne dépouillent quoi
            // que ce soit, et une citation qui part vers un aperçu de
            // messagerie n'emporte ni couleur ni corps. Les deux champs
            // doivent être remplis, ils n'ont pas à être choisis.
            theme: Theme::Mystique,
            corps: Theme::CORPS_PAR_DEFAUT,
            interligne: Theme::INTERLIGNE_PAR_DEFAUT,
            coupure: false,
            fonte: Fonte::Literata,
            // Sans effet, comme le thème et le corps : une carte de partage
            // n'a pas de barres à montrer ni d'en-tête à poser. La valeur est
            // là parce que la structure l'exige, et le défaut est le bon —
            // c'est-à-dire celui du type, depuis qu'il l'est aussi plus haut.
            habillage: Habillage::Auto,
        }
    }
}

/// Applique les réglages à un arbre de nœuds.
///
/// Trois passes, et l'ordre compte :
///
/// 1. **retirer** les nœuds éteints, en descendant dans les enfants ;
/// 2. **fondre** les fragments de texte devenus voisins — c'est ce qui met les
///    blancs orphelins côte à côte, où on peut les voir ;
/// 3. **resserrer** les blancs.
///
/// Sans la deuxième, la troisième ne verrait rien : chaque espace serait seul
/// dans son fragment, et parfaitement légitime.
pub fn preparer(noeuds: &[Noeud], preferences: Preferences) -> Vec<Noeud> {
    resserrer(fondre(retirer(noeuds, preferences)))
}

fn retirer(noeuds: &[Noeud], p: Preferences) -> Vec<Noeud> {
    noeuds
        .iter()
        .filter_map(|noeud| match noeud {
            Noeud::Glose(_) if !p.gloses => None,
            Noeud::Hebreu { .. } | Noeud::HebreuNu(_) if !p.niveau_3 => None,

            // Les conteneurs sont conservés, mais leur contenu est nettoyé :
            // une glose peut en contenir une autre, et une accentuation peut
            // contenir une translittération.
            Noeud::Glose(enfants) => Some(Noeud::Glose(retirer(enfants, p))),
            Noeud::Accentuation(enfants) => Some(Noeud::Accentuation(retirer(enfants, p))),
            Noeud::Emphase(enfants) => Some(Noeud::Emphase(retirer(enfants, p))),
            Noeud::Lien { href, enfants } => Some(Noeud::Lien {
                href: href.clone(),
                enfants: retirer(enfants, p),
            }),

            autre => Some(autre.clone()),
        })
        .collect()
}

/// Fond les fragments de texte devenus voisins.
///
/// **Récursive**, et ça n'allait pas de soi : un niveau 3 retiré à l'intérieur
/// d'une glose laisse deux fragments voisins *dans la glose*. Une fusion qui ne
/// travaillerait qu'au premier niveau ne les verrait jamais, et la glose
/// garderait son double blanc.
fn fondre(noeuds: Vec<Noeud>) -> Vec<Noeud> {
    let mut sortie: Vec<Noeud> = Vec::with_capacity(noeuds.len());
    for noeud in noeuds {
        let noeud = match noeud {
            Noeud::Glose(enfants) => Noeud::Glose(fondre(enfants)),
            Noeud::Accentuation(enfants) => Noeud::Accentuation(fondre(enfants)),
            Noeud::Emphase(enfants) => Noeud::Emphase(fondre(enfants)),
            Noeud::Lien { href, enfants } => Noeud::Lien {
                href,
                enfants: fondre(enfants),
            },
            autre => autre,
        };
        match (sortie.last_mut(), noeud) {
            (Some(Noeud::Texte(precedent)), Noeud::Texte(suivant)) => precedent.push_str(&suivant),
            (_, autre) => sortie.push(autre),
        }
    }
    sortie
}

/// Resserre les blancs qu'a laissés le retrait.
///
/// Deux règles, et la seconde est de la typographie française :
///
/// * une suite de blancs devient **une** espace ;
/// * l'espace disparaît devant `,` `.` `)` `]` `…`, jamais devant `:` `;` `!`
///   `?` `»`, qui en veulent une.
///
/// Le premier fragment perd son blanc de tête, le dernier son blanc de queue :
/// une glose en début de verset laissait la ligne commencer par un blanc.
fn resserrer(noeuds: Vec<Noeud>) -> Vec<Noeud> {
    let dernier = noeuds.len().saturating_sub(1);
    noeuds
        .into_iter()
        .enumerate()
        .map(|(rang, noeud)| match noeud {
            Noeud::Texte(texte) => {
                Noeud::Texte(resserrer_texte(&texte, rang == 0, rang == dernier))
            }
            Noeud::Glose(enfants) => Noeud::Glose(resserrer(enfants)),
            Noeud::Accentuation(enfants) => Noeud::Accentuation(resserrer(enfants)),
            Noeud::Emphase(enfants) => Noeud::Emphase(resserrer(enfants)),
            Noeud::Lien { href, enfants } => Noeud::Lien {
                href,
                enfants: resserrer(enfants),
            },
            autre => autre,
        })
        // Un fragment devenu vide n'a plus rien à faire là : il ferait un nœud
        // de texte sans texte, que le rendu traduirait en balise vide.
        .filter(|noeud| !matches!(noeud, Noeud::Texte(t) if t.is_empty()))
        .collect()
}

fn resserrer_texte(texte: &str, premier: bool, dernier: bool) -> String {
    let mut sortie = String::with_capacity(texte.len());
    let mut blanc_en_attente = false;

    for caractere in texte.chars() {
        if caractere.is_whitespace() {
            blanc_en_attente = true;
            continue;
        }
        if blanc_en_attente {
            blanc_en_attente = false;
            // L'espace ne survit pas devant la ponctuation qui n'en veut pas —
            // ni au tout début du verset.
            let ferme = matches!(caractere, ',' | '.' | ')' | ']' | '…');
            if !ferme && !(premier && sortie.is_empty()) {
                sortie.push(' ');
            }
        }
        sortie.push(caractere);
    }

    // Le blanc de queue ne survit qu'au milieu : il sépare de ce qui suit.
    //
    // La condition porte sur `premier`, **pas** sur « le fragment est vide ».
    // Un fragment qui ne contient qu'une espace est le cas le plus courant du
    // corpus : c'est celui qui sépare un intraduisible de sa translittération.
    // Le vider parce qu'il ne reste rien à sa gauche collait les deux mots —
    // et il l'aurait fait même quand on n'éteint rien du tout.
    if blanc_en_attente && !dernier && !(premier && sortie.is_empty()) {
        sortie.push(' ');
    }
    sortie
}

#[cfg(test)]
mod tests {
    /// Un lecteur qui n'a rien réglé laisse l'appareil décider.
    ///
    /// `Preferences::default()` posait `Application` quand le type dit `Auto`
    /// deux lignes sous sa définition. « Selon l'appareil » n'était donc jamais
    /// en vigueur : sur un grand écran, `/fr/liseuse/…` se faisait renvoyer
    /// vers la webapp, et la préférence se figeait au premier chargement.
    ///
    /// Trouvé le 2 octobre 2026 en vidant le profil d'un simulateur et en
    /// relevant ce que la page réécrit toute seule.
    #[test]
    fn un_lecteur_neuf_sur_grand_ecran_voit_l_edition() {
        // La propriété est écrite par son **effet**, et non par l'égalité des
        // deux défauts : c'est ce qu'un lecteur obtient qui s'était inversé, et
        // une épreuve qui compare deux constantes ne dit pas ce qu'elle protège.
        assert_eq!(
            Preferences::default().habillage.resoudre(true),
            Arbre::Liseuse,
            "sur un grand écran, un lecteur qui n'a rien réglé doit voir l'édition"
        );
        assert_eq!(
            Preferences::default().habillage.resoudre(false),
            Arbre::Webapp,
            "sur un téléphone, le même lecteur doit voir l'app"
        );
    }

    /// Les deux chemins qui mènent à un habillage par défaut disent la même
    /// chose.
    ///
    /// Un `ont.lecture` **sans** la clé retombe sur `Habillage::default()` ;
    /// **aucun** `ont.lecture` passe par `Preferences::default()`. Les deux
    /// valeurs ont divergé pendant tout le chantier des deux arbres, et le même
    /// lecteur voyait deux sites selon qu'il avait déjà réglé autre chose.
    ///
    /// ==Un défaut écrit deux fois est un défaut qu'on peut contredire.==
    #[test]
    fn les_deux_defauts_d_habillage_concordent() {
        assert_eq!(
            Preferences::default().habillage,
            Habillage::default(),
            "le défaut des préférences doit être celui du type"
        );
    }

    /// La sérialisation d'un habillage **est** la clé que le script relit.
    ///
    /// Le script d'avant-rendu lit `ont.lecture` et compare `o.habillage` aux
    /// clés qu'il connaît : si `serde` rendait « Auto » là où le script attend
    /// « auto », le lecteur retomberait sur le défaut sans qu'aucune erreur ne
    /// le dise — et la préférence paraîtrait ne pas tenir d'une visite à
    /// l'autre. C'est la garde du thème, pour la même raison.
    #[test]
    fn la_serialisation_d_un_habillage_est_sa_cle() {
        for habillage in Habillage::TOUS {
            let json = serde_json::to_string(&habillage).expect("un habillage se sérialise");
            assert_eq!(
                json,
                format!("\"{}\"", habillage.cle()),
                "{habillage:?} ne se sérialise pas comme sa clé"
            );
        }
    }

    /// Les trois habillages se distinguent, jusque dans leur note.
    ///
    /// Un sélecteur dont deux lignes disent la même chose ne choisit rien.
    #[test]
    fn les_trois_habillages_se_distinguent() {
        for (i, a) in Habillage::TOUS.into_iter().enumerate() {
            for b in Habillage::TOUS.into_iter().skip(i + 1) {
                assert_ne!(a.cle(), b.cle(), "{a:?} et {b:?} partagent leur clé");
                assert_ne!(
                    a.libelle(),
                    b.libelle(),
                    "{a:?} et {b:?} partagent leur libellé"
                );
                assert_ne!(a.note(), b.note(), "{a:?} et {b:?} partagent leur note");
            }
        }
    }

    /// `Auto` se résout par la largeur, les deux autres l'ignorent.
    ///
    /// C'est la **seule** traduction d'un souhait en fait, et le script de
    /// l'en-tête la rejoue en JavaScript. Les deux sens sont éprouvés : un
    /// habillage explicite ne doit pas se laisser fléchir par l'écran, sans quoi
    /// « l'application partout » ne voudrait rien dire.
    #[test]
    fn auto_se_resout_par_la_largeur_et_les_autres_non() {
        assert_eq!(Habillage::Auto.resoudre(true), Arbre::Liseuse);
        assert_eq!(Habillage::Auto.resoudre(false), Arbre::Webapp);
        for grand in [true, false] {
            assert_eq!(Habillage::Edition.resoudre(grand), Arbre::Liseuse);
            assert_eq!(Habillage::Application.resoudre(grand), Arbre::Webapp);
        }
    }

    /// Un arbre se reconnaît à son chemin, et le mot voisin ne le trompe pas.
    ///
    /// `/fr/webappnt-ils` n'est pas sous `/fr/webapp` — c'est le piège que
    /// `c_est_la_liseuse` a déjà payé, et `du_chemin` emploie la même forme.
    #[test]
    fn un_arbre_se_reconnait_sans_deborder() {
        assert_eq!(Arbre::du_chemin("/fr/liseuse"), Some(Arbre::Liseuse));
        assert_eq!(Arbre::du_chemin("/fr/liseuse/"), Some(Arbre::Liseuse));
        assert_eq!(
            Arbre::du_chemin("/fr/webapp/bible/bereshit"),
            Some(Arbre::Webapp)
        );
        assert_eq!(Arbre::du_chemin("/fr/webappnt-ils"), None);
        assert_eq!(Arbre::du_chemin("/fr/liseusement"), None);
        assert_eq!(Arbre::du_chemin("/fr"), None);
        assert_eq!(Arbre::du_chemin("/fr/l-app"), None);
    }

    /// La racine d'un arbre se compose de son segment, et d'aucun littéral.
    #[test]
    fn la_racine_vient_du_segment() {
        for arbre in super::Arbre::TOUS {
            assert_eq!(arbre.racine(), format!("/fr/{}", arbre.segment()));
            assert_eq!(arbre.autre().autre(), arbre);
            assert_ne!(arbre.autre(), arbre);
        }
    }

    use super::*;

    /// Un verset réel, dans la forme que le pipeline produit : le mot, sa
    /// translittération, sa glose, puis la suite de la phrase.
    fn verset() -> Vec<Noeud> {
        vec![
            Noeud::Texte("Quand ".into()),
            Noeud::Intraduisible {
                mot: "Elohim".into(),
                lemme: "elohim".into(),
            },
            Noeud::Texte(" ".into()),
            Noeud::Hebreu {
                translitteration: "elohim".into(),
                hebreu: "אֱלֹהִים".into(),
                cible: None,
            },
            Noeud::Texte(" ".into()),
            Noeud::Glose(vec![Noeud::Texte("nom divin laissé intact".into())]),
            Noeud::Texte(" commença à orchestrer".into()),
        ]
    }

    #[test]
    fn par_defaut_rien_n_est_retire() {
        assert_eq!(preparer(&verset(), Preferences::default()), verset());
    }

    /// Le fragment qui ne contient qu'une espace est le plus courant du
    /// corpus : celui qui sépare un intraduisible de sa translittération. Une
    /// version de `resserrer` le vidait, et collait les deux mots — sans qu'on
    /// ait rien éteint.
    #[test]
    fn une_espace_seule_entre_deux_noeuds_survit() {
        let noeuds = vec![
            Noeud::Intraduisible {
                mot: "Elohim".into(),
                lemme: "elohim".into(),
            },
            Noeud::Texte(" ".into()),
            Noeud::Accentuation(vec![Noeud::Texte("Cieux".into())]),
        ];
        assert_eq!(preparer(&noeuds, Preferences::default()), noeuds);
    }

    #[test]
    fn eteindre_les_gloses_ne_touche_pas_au_niveau_3() {
        let p = Preferences {
            gloses: false,
            ..Default::default()
        };
        let resultat = preparer(&verset(), p);
        assert!(!resultat.iter().any(|n| matches!(n, Noeud::Glose(_))));
        assert!(resultat.iter().any(|n| matches!(n, Noeud::Hebreu { .. })));
    }

    #[test]
    fn eteindre_le_niveau_3_ne_touche_pas_aux_gloses() {
        let p = Preferences {
            niveau_3: false,
            ..Default::default()
        };
        let resultat = preparer(&verset(), p);
        assert!(!resultat.iter().any(|n| matches!(n, Noeud::Hebreu { .. })));
        assert!(resultat.iter().any(|n| matches!(n, Noeud::Glose(_))));
    }

    /// Le point de tout ce module : le corps reste **une phrase**, pas une
    /// phrase trouée de blancs.
    #[test]
    fn le_retrait_ne_laisse_aucun_blanc_derriere_lui() {
        let resultat = preparer(&verset(), Preferences::nu());
        assert_eq!(
            resultat,
            vec![
                Noeud::Texte("Quand ".into()),
                Noeud::Intraduisible {
                    mot: "Elohim".into(),
                    lemme: "elohim".into(),
                },
                Noeud::Texte(" commença à orchestrer".into()),
            ]
        );
    }

    /// Le cas qui a mordu sur les aperçus de messagerie : la glose se pose
    /// avant la virgule.
    #[test]
    fn la_ponctuation_se_referme_sur_le_mot() {
        let noeuds = vec![
            Noeud::Texte("ni habitant ".into()),
            Noeud::Glose(vec![Noeud::Texte("tohu wa-bohu".into())]),
            Noeud::Texte(", et la face".into()),
        ];
        assert_eq!(
            preparer(&noeuds, Preferences::nu()),
            vec![Noeud::Texte("ni habitant, et la face".into())]
        );
    }

    /// Et le français garde l'espace qu'il exige devant les autres signes.
    #[test]
    fn le_deux_points_garde_son_espace() {
        let noeuds = vec![
            Noeud::Texte("Il dit ".into()),
            Noeud::Glose(vec![Noeud::Texte("wayomer".into())]),
            Noeud::Texte(" : que".into()),
        ];
        assert_eq!(
            preparer(&noeuds, Preferences::nu()),
            vec![Noeud::Texte("Il dit : que".into())]
        );
    }

    /// Une glose en tête de verset laissait la ligne commencer par un blanc.
    #[test]
    fn un_verset_ne_commence_pas_par_un_blanc() {
        let noeuds = vec![
            Noeud::Glose(vec![Noeud::Texte("incise".into())]),
            Noeud::Texte(" Et la Terre".into()),
        ];
        assert_eq!(
            preparer(&noeuds, Preferences::nu()),
            vec![Noeud::Texte("Et la Terre".into())]
        );
    }

    /// Une accentuation **survit** à l'extinction des niveaux — c'est la
    /// règle de l'app, et elle tient au sens : il appartient au corps.
    /// Mais ses enfants sont nettoyés.
    #[test]
    fn une_accentuation_survit_mais_son_contenu_est_nettoye() {
        let noeuds = vec![Noeud::Accentuation(vec![
            Noeud::Texte("Cieux".into()),
            Noeud::Hebreu {
                translitteration: "shamayim".into(),
                hebreu: "שָׁמַיִם".into(),
                cible: None,
            },
        ])];
        assert_eq!(
            preparer(&noeuds, Preferences::nu()),
            vec![Noeud::Accentuation(vec![Noeud::Texte("Cieux".into())])]
        );
    }

    /// Un intraduisible ne s'éteint jamais : il est le texte, pas son
    /// commentaire.
    #[test]
    fn un_intraduisible_ne_s_eteint_jamais() {
        let resultat = preparer(&verset(), Preferences::nu());
        assert!(resultat
            .iter()
            .any(|n| matches!(n, Noeud::Intraduisible { .. })));
    }

    /// Une glose qui en contient une autre est retirée en entier, et une glose
    /// conservée voit ses enfants nettoyés.
    #[test]
    fn le_nettoyage_descend_dans_les_enfants() {
        let noeuds = vec![Noeud::Glose(vec![
            Noeud::Texte("forme de ".into()),
            Noeud::Hebreu {
                translitteration: "eloah".into(),
                hebreu: "אֱלוֹהַּ".into(),
                cible: None,
            },
            Noeud::Texte(" au pluriel".into()),
        ])];
        let p = Preferences {
            niveau_3: false,
            ..Default::default()
        };
        assert_eq!(
            preparer(&noeuds, p),
            vec![Noeud::Glose(vec![Noeud::Texte(
                "forme de au pluriel".into()
            )])]
        );
    }
}

/// Le corps d'une suite de nœuds, en texte plat.
///
/// C'est [`Preferences::nu`] suivi d'un aplatissement : ce qu'on cite hors de
/// la liseuse — un aperçu de messagerie, une carte de partage.
///
/// Il vit ici et non dans [`crate::domaine::texte`] pour que la règle de
/// nettoyage n'existe **qu'une fois**. Elle y était écrite une seconde fois, et
/// deux copies d'une règle typographique finissent toujours par diverger d'un
/// signe que personne ne remarque.
pub fn corps(noeuds: &[Noeud]) -> String {
    fn aplatir(noeuds: &[Noeud], sortie: &mut String) {
        for noeud in noeuds {
            match noeud {
                Noeud::Texte(t) => sortie.push_str(t),
                // Le libellé du renvoi **est** du corps de texte : « déjà posé
                // en ((la-chuqqah|cette disposition)) » se lit d'une traite, et
                // l'omettre trouerait la phrase.
                // La référence biblique aussi : « comme il est dit en
                // *Genèse* 9:27 » est une phrase, et retirer la référence la
                // couperait en deux.
                Noeud::Intraduisible { mot, .. }
                | Noeud::Shem { mot, .. }
                | Noeud::Renvoi { libelle: mot, .. }
                | Noeud::Reference { libelle: mot, .. } => sortie.push_str(mot),
                Noeud::Accentuation(enfants)
                | Noeud::Emphase(enfants)
                | Noeud::Lien { enfants, .. } => aplatir(enfants, sortie),
                Noeud::Saut => sortie.push(' '),
                // `preparer` les a déjà retirés ; le cas reste pour que le
                // compilateur signale un nouveau niveau qu'on oublierait.
                Noeud::Glose(_) | Noeud::Hebreu { .. } | Noeud::HebreuNu(_) => {}
            }
        }
    }

    let mut sortie = String::new();
    aplatir(&preparer(noeuds, Preferences::nu()), &mut sortie);
    sortie
}

#[cfg(test)]
mod epreuves_de_la_liseuse {
    use super::c_est_la_liseuse;

    #[test]
    fn les_trois_pages_de_corpus_portent_la_peau() {
        // La peau vaut sous **les deux arbres** : un lecteur qui a choisi le
        // parchemin le garde qu'il lise dans l'édition ou dans l'app.
        for arbre in super::Arbre::TOUS {
            for chemin in [
                crate::domaine::chemins::bible(arbre),
                crate::domaine::chemins::livre(arbre, "bereshit"),
                crate::domaine::chemins::unite(arbre, "bereshit", "bereshit-1"),
                crate::domaine::chemins::lexique(arbre),
                crate::domaine::chemins::fiche(arbre, "bara"),
                crate::domaine::chemins::compte(arbre),
                crate::domaine::chemins::rechercher(arbre),
            ] {
                assert!(
                    c_est_la_liseuse(&chemin),
                    "{chemin} devrait porter la peau du lecteur"
                );
            }
        }
    }

    #[test]
    fn l_edition_garde_la_nuit_d_aubergine() {
        for chemin in [
            "/fr",
            "/fr/le-pourquoi",
            "/fr/l-auteur",
            "/fr/l-app",
            "/fr/ce-que-l-ont-n-est-pas",
            // Elle montre du corpus, et elle garde pourtant la nuit : elle
            // porte une ouverture, et une ouverture ne survit pas au clair.
            "/fr/rechercher",
            "/fr/rechercher?q=ruach",
            "/fr/confidentialite",
            "/fr/conditions",
            "/",
        ] {
            assert!(!c_est_la_liseuse(chemin), "{chemin} est l'édition");
        }
    }

    /// Un préfixe n'est pas une frontière.
    ///
    /// `/fr/webappnt-ils` commence par `/fr/webapp`. Aucune de ces adresses
    /// n'existe aujourd'hui — et c'est justement pourquoi l'épreuve compte :
    /// le jour où l'une d'elles naîtra, personne ne pensera à revenir ici.
    #[test]
    fn un_prefixe_ne_deborde_pas_sur_le_mot_voisin() {
        for chemin in ["/fr/webappnt-ils", "/fr/lexiquement", "/fr/webapp-moi"] {
            assert!(!c_est_la_liseuse(chemin), "{chemin} n'est pas la liseuse");
        }
    }

    /// La barre finale ne change rien — un routeur peut la poser ou non.
    #[test]
    fn la_barre_finale_est_sans_effet() {
        assert!(c_est_la_liseuse("/fr/webapp/"));
        assert!(!c_est_la_liseuse("/fr/"));
    }
}

#[cfg(all(test, feature = "ssr"))]
mod epreuves_du_corps {
    use super::Theme;

    /// ## Les bornes du curseur sont celles de l'app, et c'est mesuré
    ///
    /// L'app les écrit une fois, dans `TaillesAuClavier.corps`, avec sa raison :
    ///
    /// > Les bornes du corps du texte — **les mêmes que le curseur des
    /// > réglages**, `Slider(in: 11...28, step: 1)`. Écrites une fois ici,
    /// > employées par les deux, pour qu'un raccourci ne puisse pas atteindre
    /// > une valeur que le curseur refuse.
    ///
    /// Le site fait la même chose d'un cran plus loin : il ne les recopie pas,
    /// il les **relit chez elle**. Le jour où l'auteur élargit son curseur, ce
    /// test rougit ici — et l'écart se voit avant qu'un lecteur ne trouve deux
    /// amplitudes différentes sur ses deux appareils.
    ///
    /// C'est la même forme que le témoin des couleurs, et pour la même raison :
    /// **une valeur copropriétaire de deux parties se garde par une mesure**,
    /// jamais par une transcription.
    ///
    /// Le défaut, lui, n'est pas dans ce fichier-là — il vit dans
    /// `ReadingPreferences.default`. On le relève à part.
    #[test]
    fn les_bornes_sont_celles_du_curseur_de_l_app() {
        let chemin = "../ONTBibleApp/app/MacSources/TaillesAuClavier.swift";
        let source = std::fs::read_to_string(chemin).unwrap_or_else(|erreur| {
            panic!(
                "  {chemin} est illisible : {erreur}\n\
                 \n\
                 Les trois dépôts se rangent côte à côte sous `~/ONTBible/`, et\n\
                 la CI les clone ainsi. Sans le voisin, cette garde ne peut pas\n\
                 mesurer — et elle refuse plutôt que de se taire."
            )
        });

        let ligne = source
            .lines()
            .find(|l| l.contains("static let corps"))
            .unwrap_or_else(|| {
                panic!(
                    "  `static let corps` a disparu de TaillesAuClavier.swift.\n\
                     Les bornes ont changé de nom ou de place : les relever et\n\
                     remettre ce test d'accord avec elles."
                )
            });

        // `static let corps: ClosedRange<Double> = 11...28`
        let plage = ligne.split('=').nth(1).expect("une affectation").trim();
        let (bas, haut) = plage.split_once("...").unwrap_or_else(|| {
            panic!("  les bornes ne s'écrivent plus `bas...haut` : « {plage} »")
        });

        assert_eq!(
            (bas.trim(), haut.trim()),
            (
                Theme::CORPS_MINIMUM.to_string().as_str(),
                Theme::CORPS_MAXIMUM.to_string().as_str()
            ),
            "le curseur de l'app va de {bas} à {haut} ; le site dit {} à {}",
            Theme::CORPS_MINIMUM,
            Theme::CORPS_MAXIMUM
        );
    }

    /// ## Les sept fontes sont celles de l'app, libellés et notes compris
    ///
    /// Relues dans `ReadingFont`, jamais recopiées. Trois choses y sont
    /// vérifiées, et la troisième est la moins évidente :
    ///
    /// - **le nombre**, pour qu'une fonte ajoutée là-bas rougisse ici plutôt
    ///   que de manquer en silence dans un menu ;
    /// - **les libellés**, parce qu'un lecteur qui passe du téléphone au site
    ///   cherche le même mot ;
    /// - **les notes**, pour la même raison, et parce qu'elles ont été écrites
    ///   pour un usage précis — « de quoi choisir sans être typographe ».
    ///
    /// C'est le témoin des couleurs, transposé au texte : une valeur
    /// copropriétaire de deux parties se garde par une mesure.
    #[test]
    fn les_fontes_sont_celles_de_l_app() {
        let chemin = "../ONTBibleApp/app/Packages/ONTKit/Sources/ONTKit/Reader/Reader.swift";
        let source = std::fs::read_to_string(chemin)
            .unwrap_or_else(|erreur| panic!("  {chemin} est illisible : {erreur}"));

        // **Borner au bon type avant de relever.** Le premier jet prenait le
        // premier `public var label` du fichier — c'était celui des cinq
        // couleurs de surlignage, et l'épreuve a rendu « Or, Olive, Ciel, Rose,
        // Violet ». Elle a rougi, donc elle a fait son travail ; mais un relevé
        // qui n'est pas borné mesure ce qu'il croise, pas ce qu'il cherche.
        let source = source
            .split("public enum ReadingFont")
            .nth(1)
            .unwrap_or_else(|| panic!("  `ReadingFont` a disparu de Reader.swift"))
            .split("\n}")
            .next()
            .expect("un bloc non vide rend au moins un fragment");

        /// Relève les membres droits d'un `switch` de `ReadingFont`.
        ///
        /// Le `switch` s'écrit `case .literata: "Literata"` — une ligne par
        /// entrée, la valeur entre guillemets. On les prend dans l'ordre, qui
        /// est celui du menu des deux côtés.
        fn membres(source: &str, apres: &str) -> Vec<String> {
            source
                .split(apres)
                .nth(1)
                .unwrap_or_else(|| panic!("  `{apres}` a disparu de ReadingFont"))
                .lines()
                .take_while(|l| !l.trim_start().starts_with('}'))
                .filter_map(|l| l.split_once(": \""))
                .filter_map(|(_, reste)| reste.split_once('"'))
                .map(|(valeur, _)| valeur.to_string())
                .collect()
        }

        let libelles = membres(&source, "public var label: String {");
        let notes = membres(&source, "public var note: String {");

        assert_eq!(
            libelles.len(),
            super::Fonte::TOUTES.len(),
            "l'app offre {} fontes, le site {} — {libelles:?}",
            libelles.len(),
            super::Fonte::TOUTES.len()
        );

        for (i, fonte) in super::Fonte::TOUTES.into_iter().enumerate() {
            assert_eq!(
                fonte.libelle(),
                libelles[i],
                "le libellé {i} diverge : le site dit « {} », l'app « {} »",
                fonte.libelle(),
                libelles[i]
            );
            assert_eq!(
                fonte.note(),
                notes[i],
                "la note de {} diverge :\n  site : {}\n  app  : {}",
                fonte.libelle(),
                fonte.note(),
                notes[i]
            );
        }
    }

    /// ## Les quatre rayons sont ceux d'`ONTRadius`
    ///
    /// Relus dans l'app, en points, et comparés aux jetons de la feuille, en
    /// `rem`. La conversion est à `16 px = 1 rem` — la taille de base d'un
    /// navigateur, que le site ne change jamais (c'est ce qui fait que le zoom
    /// du lecteur commande, §5).
    ///
    /// **Le rayon de la feuille est celui qui valait l'épreuve.** Il est mesuré
    /// et non choisi : « à 22, la carte du Mac se lisait comme une boîte de
    /// dialogue, pas comme une feuille ». Le panneau « aA » du site était
    /// précisément à 22.
    #[test]
    fn les_rayons_sont_ceux_de_l_app() {
        let chemin =
            "../ONTBibleApp/app/Packages/ONTDesignSystem/Sources/ONTDesignSystem/Tokens/ONTMetrics.swift";
        let source = std::fs::read_to_string(chemin)
            .unwrap_or_else(|erreur| panic!("  {chemin} est illisible : {erreur}"));
        let feuille = include_str!("../../style/main.css");

        for (la_bas, ici) in [
            ("highlight", "--radius-surlignage"),
            ("block", "--radius-bloc"),
            ("card", "--radius-carte"),
            ("feuille", "--radius-feuille"),
        ] {
            let points: f64 = source
                .lines()
                .find(|l| l.contains(&format!("static let {la_bas}:")))
                .and_then(|l| l.split('=').nth(1))
                .and_then(|v| v.trim().parse().ok())
                .unwrap_or_else(|| panic!("  `ONTRadius.{la_bas}` est illisible"));

            let rem: f64 = feuille
                .lines()
                .find(|l| l.trim_start().starts_with(&format!("{ici}:")))
                .and_then(|l| l.split(':').nth(1))
                .and_then(|v| {
                    v.trim()
                        .trim_end_matches(';')
                        .trim_end_matches("rem")
                        .parse()
                        .ok()
                })
                .unwrap_or_else(|| panic!("  `{ici}` est illisible dans style/main.css"));

            assert!(
                (rem * 16.0 - points).abs() < 0.01,
                "{ici} vaut {rem} rem soit {} px ; `ONTRadius.{la_bas}` vaut {points} pt",
                rem * 16.0
            );
        }
    }

    /// Le défaut laisse le site **exactement** tel qu'il était.
    ///
    /// `--lecture` vaut `corps / 19`. Au défaut il vaut 1, donc `calc(x * 1)`
    /// rend `x` : pas un pixel ne bouge tant que le lecteur n'a pas touché au
    /// réglage. C'est la même sûreté que le portage des couleurs, et c'est
    /// elle qui rend le changement sans risque.
    #[test]
    fn le_defaut_est_neutre() {
        let p = super::Preferences::default();
        assert_eq!(p.corps, Theme::CORPS_PAR_DEFAUT);
        assert!(
            (f64::from(p.corps) / f64::from(Theme::CORPS_PAR_DEFAUT) - 1.0).abs() < f64::EPSILON
        );
    }

    /// Le défaut de l'app, relevé chez elle aussi.
    #[test]
    fn le_defaut_est_celui_de_l_app() {
        let chemin = "../ONTBibleApp/app/Packages/ONTKit/Sources/ONTKit/Reader/Reader.swift";
        let source = std::fs::read_to_string(chemin)
            .unwrap_or_else(|erreur| panic!("  {chemin} est illisible : {erreur}"));
        let ligne = source
            .lines()
            .find(|l| l.contains("textSize: Double ="))
            .unwrap_or_else(|| {
                panic!("  le défaut de `textSize` a changé de forme dans Reader.swift")
            });
        let valeur: u8 = ligne
            .split('=')
            .nth(1)
            .and_then(|v| v.trim().trim_end_matches(',').parse().ok())
            .unwrap_or_else(|| panic!("  défaut illisible : « {ligne} »"));
        assert_eq!(
            valeur,
            Theme::CORPS_PAR_DEFAUT,
            "l'app ouvre à {valeur}, le site dit {}",
            Theme::CORPS_PAR_DEFAUT
        );
    }
}
