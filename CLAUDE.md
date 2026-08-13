# ONTBibleWebapp — le site de La Bible ONT

Document de reprise. Il contient ce qu'une nouvelle session ne peut pas
deviner : les décisions déjà prises, leurs raisons, et ce qui reste ouvert.

---

## 1. Ce qu'est ce site, et pour qui

`ontbible.com` — le site public de **La Bible ONT**, une restitution française
du corpus hébreu et araméen antique fondée sur l'ontologie hébraïque
fonctionnelle.

Il répond à trois besoins, dans cet ordre d'importance :

1. **Dire le pourquoi.** Aujourd'hui, un lien partagé depuis l'app arrive sur
   une page qui ne dit rien. Quelqu'un qui découvre l'ONT n'a nulle part où
   comprendre de quoi il s'agit.
2. **Servir les liens partagés** — `ontbible.com/fr/lire/<livre>/<unité>?v=1-3`
   et le fichier d'association des liens universels.
3. **Fournir ce que l'App Store réclamera** : une politique de confidentialité,
   des conditions, une adresse marketing.

## 2. L'auteur — à lire avant d'écrire une ligne

**Gloire Bikouta.** « Gloire » est son deuxième prénom, celui qu'il emploie en
public, y compris sur GitHub. **Jamais « Sha'eliel »** : c'est son nom
fonctionnel *interne* au projet, présent dans le vault, et il ne sort pas.

**Son registre.** Il écrit en parataxe — « Et… Et… Et… », le rythme du waw
consécutif hébreu. Phrases courtes, verbales, enchaînées sans subordination.
Aucune atténuation. Et il laisse les termes hébreux **debout, sans glose** :
*sod*, *neshamah*, *Olam*, *Maqom*, *Ish*, *bene ha-adamah*.

Ce n'est pas une méthode qu'il a adoptée pour l'ONT : c'est déjà sa façon
d'écrire en privé depuis des années. **Le site doit sonner comme lui**, pas
comme une notice. Phrases courtes et affirmatives, mots hébreux laissés tels
quels. Pas de « en revanche », pas de « il convient de noter ».

**L'origine du projet est publique — vision comprise.** Décision prise le
12 août 2026, après lui avoir proposé trois positions (rien de personnel /
l'origine nommée sans la vision / la vision). Il a choisi la troisième.

La source est une page Notion **privée** — *The Holy Day*, base « Book of the
Torpor » — écrite le 27 juillet 2026 sur une vision reçue deux ans plus tôt.
Elle est verrouillée, et il y écrit qu'il avait cru qu'elle ne devait pas être
écrite. **En tirer une version publique, pas la recopier.** Le cœur :

> un manteau d'antiquité, et l'ordre de ramener ce qui a été perdu depuis
> d'anciens temps — « les choses dont le monde n'a même pas conscience qu'il
> ne sait pas ».

Lui demander le texte définitif de cette page. Proposer un premier jet plutôt
qu'une page blanche, mais **ne pas publier sans sa relecture**.

## 3. La pile

**Leptos 0.8 en mode SSR + Axum, et Tailwind v4.** Tout en Rust, rendu côté
serveur — sa demande explicite. `cargo-leptos` télécharge lui-même le binaire
Tailwind ; il n'y a ni `node_modules`, ni `tailwind.config.js` (la v4 se
configure en CSS, dans `style/main.css`).

**L'architecture est en couches, et une flèche ne remonte jamais :**

```
domaine        ──▶  rien                       pur, compile aussi en wasm
application    ──▶  domaine                    déclare des ports (traits)
infrastructure ──▶  application, domaine       les réalise — `ssr` seulement
interface      ──▶  application, domaine       design/ (forme) + pages/ (propos)
main.rs        ──▶  tout                       la racine de composition
```

Le domaine ne prend même pas de bibliothèque de dates : il reçoit un numéro de
jour. C'est ce qui le rend testable sans horloge, et identique des deux côtés.

**Un composant par fichier**, `design/` pour la forme et `pages/` pour le
propos. Une page n'écrit aucune valeur de style — si une forme manque, elle se
crée dans `design/`. Depuis Tailwind, la forme vit **dans** le `.rs`, à côté du
balisage : il n'y a plus de feuille parallèle qui puisse diverger.

Le backend de l'app est déjà du Rust sur AWS Lambda (`ONTBibleApp/backend/`,
axum + `lambda_http`, architecture hexagonale, eu-west-3). Le site suit la
même voie : `cargo lambda`, même région, même façon de déployer. C'est
cohérent, et ça évite d'introduire un hébergeur de plus.

**Réserve à connaître** : un démarrage à froid Lambda mesuré à ~450 ms. Sur un
site vitrine, c'est visible. Si ça gêne, les options sont une réservation de
concurrence provisionnée (payante) ou un hébergeur qui garde le processus
chaud (Fly.io, Railway). À trancher après l'avoir vu tourner, pas avant.

**Cloudflare Pages est écarté** : il ne fait pas tourner de serveur Rust.

## 4. L'architecture des domaines

État actuel — `ontbible.com` pointe vers la Lambda de l'**API** :

```
ontbible.com  →  CNAME  d-xdeopzr27e.execute-api.eu-west-3.amazonaws.com
                 (nuage GRIS chez Cloudflare — le proxy orange casse le TLS
                  du domaine personnalisé d'API Gateway)
```

**Cible** — le site prend la racine, l'API passe sur un sous-domaine :

| | |
|---|---|
| `ontbible.com` | le site (cette application) |
| `api.ontbible.com` | l'API de l'app (Lambda existante) |
| `labibleont.com` | redirige en 301 vers `ontbible.com`, chemin et paramètres préservés |

Deux routes doivent **impérativement** rester servies à la racine, parce
qu'elles sont aujourd'hui rendues par la Lambda de l'API et qu'elles cassent
si elles disparaissent :

- `/.well-known/apple-app-site-association` — **`application/json`, aucune
  redirection**. C'est ce fichier qui autorise l'app iOS à ouvrir les liens
  `ontbible.com`. Contenu exact à reprendre depuis
  `ONTBibleApp/backend/src/interface/web.rs` : `appIDs` vaut
  `N49VNC2G57.com.labibleont.ONT`, `components` vaut `/fr/lire/*`.
  iOS ne le relit qu'à l'installation de l'app et le met en cache via le CDN
  d'Apple — une erreur ici ne se voit pas tout de suite.
- `/fr/lire/{livre}/{unité}` — la page d'un passage, avec ses balises Open
  Graph. Le segment de langue est délibéré : il épargne une migration le jour
  d'une édition anglaise.

Migration à faire **dans cet ordre**, sinon les liens universels tombent :
créer `api.ontbible.com`, y basculer `ONTAPIBaseURL` dans l'app, vérifier,
puis seulement rediriger la racine vers le site.

## 5. La direction artistique

**« Un peu ancienne »** — sa demande. Le livre imprimé classique, pas le
pastiche : filets fins, capitales espacées, larges marges, papier plutôt
qu'écran.

### Le site est une nuit d'aubergine

Trois essais ont été nécessaires, et les deux premiers sont instructifs. Le
site a d'abord suivi le thème du système : chez un lecteur en mode sombre, il
retombait sur un gris quasi noir, sans matière — une page quelconque. On l'a
alors interdit et posé le site sur du parchemin. Les deux manquaient la cible :
**il le voulait sombre**, mais une aubergine, pas un noir.

La rampe est **dérivée de la marque** — teinte constante 343°, celle de
`#421B26` relevée au pixel, seule la clarté varie. Les valeurs sont calculées,
pas choisies à l'œil ; la marque elle-même retombe sur `#411B26`, ce qui
vérifie la dérivation.

| jeton | valeur | rôle |
|---|---|---|
| `nuit` | `#18090D` | le fond de la page |
| `surface` | `#261016` | une carte, une porte, une ligne |
| `surface-haute` | `#35151E` | un bandeau, ce qu'on détache |
| `aubergine` | `#421B26` | la marque, le haut des dégradés |
| `or` / `accent` | `#CDBE83` | l'intraduisible, les titres de section, les filets |
| `encre` | `#CFC5B9` | le corps — 11,4:1 |
| `encre-vive` | `#EDE3D6` | un titre — 15,3:1 |
| `encre-douce` | `#9D948B` | le niveau 2 — 6,5:1 |
| `important` | `#D87994` | le terme important — 6,5:1 |

**Jamais de blanc pur**, et jamais un gris. La halation fait déborder un texte
clair dans le noir qui l'entoure : les lettres paraissent plus grasses, vibrent
et se doublent sur une dalle OLED. Descendre à 87 % supprime l'essentiel du
halo sans rien coûter — on reste bien au-delà du seuil AAA. Et l'encre garde la
chaleur du parchemin de l'app plutôt que de virer au gris de laboratoire.

Trois conséquences dans `style/main.css`, et ce ne sont pas des goûts :

- `-webkit-font-smoothing: antialiased`. Il amincit les traits — sur fond clair
  il les fait disparaître, sur fond sombre il compense exactement le
  débordement optique. C'est le seul cas où il faut le forcer.
- un souffle d'interlettrage (`0.006em`) : sur une nuit, les lettres claires se
  rapprochent visuellement et finissent par se toucher.
- `grain-page` — un bruit `feTurbulence` à 3,5 %. **Nécessité technique** : un
  dégradé sombre étalé sur une page se découpe en bandes, parce qu'un écran
  n'a que 256 valeurs par canal et que l'écart entre deux nuances de nuit est
  plus petit que ça. Le bruit casse les bandes. Il donne au passage la matière
  d'un papier ancien.

Les filets sont **dorés à 18 %**, pas gris : sur une nuit chaude, un gris
paraît sale.

`voile-aubergine` donne la profondeur d'un bandeau — une lueur haute, la nuit
qui reprend vers le bas, de sorte que la lumière semble venir d'au-dessus du
texte, comme dans une nef. Trois couches et pas une de plus : au-delà, un
dégradé cesse d'être une atmosphère et devient un effet.

### Typographie

| rôle | fonte |
|---|---|
| titres, navigation, capitales | **Jost** — la géométrique du combination mark |
| tout le reste, citations comprises | **Literata** — celle du corps de l'app |
| hébreu | **Ezra SIL** — la seule qui positionne niqqud et te'amim |

**EB Garamond a été essayée puis retirée**, et il faut savoir pourquoi avant de
la reproposer : c'est la lettre du livre imprimé classique, mais ses déliés
très fins bavent sur fond sombre et ses empattements vibrent. C'est le défaut
connu des romanes à fort contraste en mode sombre. Literata est dessinée pour
l'écran, à faible contraste de graisse — et c'est déjà la fonte du corps de
l'app, donc un verset cité ici se lit exactement comme sur le téléphone.

Jost ne tient pas dix lignes de texte suivi : hauteur d'x basse, fûts égaux.
Elle garde les titres, où sa géométrie fait écho à la marque.

### Le portrait — pas de cadre, et il sort de la colonne

Deux erreurs successives, et la seconde était plus subtile que la première.

La photo flottait à même la page, détourée : elle avait l'air d'un autocollant,
et la chemise blanche faisait une masse lumineuse. On l'a donc inscrite dans une
arche bordée d'un filet d'or — ce qui l'a **enfermée dans une boîte**. Le
rectangle se voyait plus que le sujet.

Ce qui remplace le cadre est une **lueur** : un halo d'aubergine flou derrière
les épaules, sans contour. Il fait le même travail — donner un fond au sujet
pour qu'il ne flotte pas — sans rien dessiner. On ne voit pas d'où vient la
lumière, seulement qu'il y en a une. Le bas continue de se dissoudre dans la
nuit (`fondu-bas`) : le détourage laisse le vêtement s'effilocher, autant en
faire une intention.

Et le portrait **sort de la colonne de lecture** par la gauche
(`lg:-ml-40 lg:w-80`). La mesure borne le *texte* ; elle n'a aucune raison de
borner une image, et la marge du site est vide. Il y gagne le tiers de taille
qui lui manquait sans que la ligne de texte s'allonge d'un signe. En dessous de
`lg` il n'y a plus de marge à occuper : il repasse dans la colonne, puis
au-dessus du texte sur téléphone.

C'est le patron à réutiliser pour toute image du site.

### L'échelle typographique

Le corps est à **21 px**, pas à 16. Deux effets se cumulent : Literata a une
hauteur d'x généreuse mais une **chasse étroite**, et sur une nuit d'aubergine
le texte clair **paraît plus petit** qu'il ne l'est — l'inverse de la halation,
qui l'épaissit sans l'agrandir. Les essais à 17 puis 19 px se lisaient encore
comme des notes de bas de page. Il a demandé plus grand deux fois : ne pas
redescendre sans qu'il le demande.

| | |
|---|---|
| `text-sm` | 16 px — capitales espacées, légendes |
| `text-base` | **21 px** — le corps |
| `text-lg` → `text-3xl` | quarte juste, en `clamp` |

**La mesure suit la taille**, et c'est la règle à retenir : à 21 px, 34 rem ne
tiendraient plus que 52 signes par ligne — sous la fourchette confortable de 55
à 75, et le texte se hacherait. Elle passe donc à 38 rem, soit environ 58
signes : toujours du côté étroit qu'il préfère. Grandir le texte sans élargir
la mesure produit des lignes trop courtes, ce qui se lit aussi mal que des
lignes trop longues.

L'interligne descend à 1,68 en compensation : plus le corps grandit, moins il a
besoin d'air proportionnel pour que l'œil retrouve la ligne suivante.

### L'ouverture est **une** unité

**Toutes** les ouvertures remplissent l'écran, l'accueil comme les pages
intérieures. La variante `sobre` ne diffère que par la **lumière** — jamais par
la hauteur. Un essai l'avait fixée à 70 % : les pages intérieures montraient
alors leur ouverture *et* le début du bloc suivant, ce qui détruit exactement
l'unité recherchée.

Et le contenu y est **ancré en haut**, à distance fixe, non centré. Le centrage
est cohérent avec lui-même et incohérent d'une page à l'autre : sur la page de
l'auteur, dont l'ouverture porte un portrait, le contenu plus haut commençait
cent soixante pixels au-dessus des autres, collé sous la navigation. Le rappel
et le titre tombent désormais à la même hauteur partout — vérifié au pixel,
y = 328 sur les quatre pages. Le vide qui reste en bas n'est pas perdu : c'est
là que l'horizon se lit.


Ce qui frappe en arrivant sur un site bien fait, c'est un premier écran qui est
une seule chose et qui le remplit. Le premier essai posait l'en-tête en bande
au-dessus de l'ouverture : on voyait alors deux objets — un bandeau, puis un
écran — et l'effet tombait.

**`Hero` contient l'en-tête.** La marque et la navigation flottent dans le lieu
au lieu de le surmonter — d'où le `z-20` et l'absence de fond.

Un premier montage remontait l'ouverture *sous* l'en-tête, d'une marge négative
égale à un jeton `--hauteur-entete`. **Ça ne pouvait pas tenir** : sur un
téléphone, la navigation passe sur plusieurs lignes et l'en-tête devient deux
fois plus haut que le nombre inscrit. La bande réapparaissait, et le contenu
débordait sous la ligne de flottaison. Un nombre magique qui décrit la
géométrie d'un autre élément est toujours faux quelque part.

Les pages sans ouverture — les légales, l'erreur — posent leur en-tête
elles-mêmes. Il n'y en a jamais deux : vérifié, une balise `<header>` par page.

La navigation se resserre en dessous du seuil : à 16 px avec un interlettrage
de 0,16 em, ses trois entrées passaient sur trois lignes sur un téléphone.

### Un écran par bloc

Chaque `Bloc` occupe au moins la hauteur de la fenêtre, contenu centré. Sans
ça, les fonds clairs et sombres épousaient la longueur des textes et se
lisaient comme des rayures posées au hasard.

`min-h-dvh`, et les deux mots comptent :

- **minimale** et non fixe — la comparaison et les pages légales dépassent un
  écran, une hauteur fixe les couperait ;
- **`dvh`** et non `vh` — sur un téléphone, `vh` compte la fenêtre sans la
  barre d'adresse, qui se rétracte au défilement : chaque bloc sauterait de
  quelques dizaines de pixels au premier geste.

L'ouverture, elle, occupe l'écran **moins l'en-tête** (`--hauteur-entete`) :
sinon le premier écran montrerait la marque, puis le haut d'un hero d'un écran
entier, et la phrase tomberait sous la ligne de flottaison.

### « Ancienne mais moderne » — où ça se joue

L'ancien donne la **structure**, le moderne donne la **retenue**. Chaque fois
qu'un signe ancien est employé, il l'est sans ornementation : c'est ce contraste
qui empêche la page de virer à l'antiquaire.

| le signe, ancien | l'exécution, moderne |
|---|---|
| chiffres romains sur les sections (`TitreDeSection`) | un chiffre, un filet court, beaucoup d'air — ni cartouche ni enluminure |
| lettrine sur trois lignes (`lettrine`) | une lettre d'or dans la fonte des titres, sans fond ni cadre |
| chiffres elzéviriens dans le texte | et chiffres alignés de chasse fixe dans les tableaux — une colonne n'est pas une phrase |
| ponctuation suspendue | `hanging-punctuation`, que le navigateur fait seul |
| portrait en arche, comme une gravure | un filet d'or d'un pixel, une surface unie, un recadrage net |

Deux pièges rencontrés, à ne pas refaire :

- `::first-letter` **embarque la ponctuation qui précède**. Un paragraphe qui
  commence par un guillemet met donc le guillemet dans la lettrine — et la
  ponctuation suspendue le jette dans la marge par-dessus. Les paragraphes à
  lettrine commencent par une lettre.
- Un filigrane de montagne trop grand et rogné par son cadre ne se lit plus
  comme une montagne : c'est du bruit. Il n'en reste qu'un, dans le `Hero`, à
  6 % et débordant par le haut de sorte qu'on lise une crête.

### Les trois niveaux du texte — le cœur du projet

Si le site affiche du corpus, il doit les rendre **distincts**. C'est la
raison d'être de tout le pipeline (voir `ONTBibleApp/README.md`) :

| dans le `.md` | rôle | rendu |
|---|---|---|
| texte nu | niveau 1 — le corps | encre |
| `**mot**` | intraduisible | **or**, cliquable vers sa fiche |
| `==mot==` | terme important | **`#862742`**, semi-gras, **inerte** |
| `*[glose]*` | niveau 2 | plus petit, encre atténuée |
| `(*translit* / hébreu)` | niveau 3 | italique + Ezra SIL, isolation bidi FSI/PDI |

L'or **promet** une fiche et la tient. Le bordeaux clair marque sans rien
promettre. Ne pas les confondre.

## 6. Les ressources

Dans `public/images/` — et pas dans un `assets/` séparé : `public/` est le
dossier que `cargo-leptos` recopie tel quel vers la racine du site, donc une
image y est à la fois la source et le livrable. Un second dossier obligerait à
synchroniser les deux, et l'un des deux finirait périmé.

| fichier | usage |
|---|---|
| `logomark.svg` | la montagne seule — **favicon**, et signe de section |
| `wordmark.svg` | « La Bible ONT » et מקרא הקדם |
| `combination-mark.svg` | les deux, séparés par un filet |
| `montagne-512.png` | repli de favicon, pour les navigateurs sans SVG |
| `portrait-640.webp`, `portrait-1024.webp` | Gloire Bikouta, détouré depuis le brut |
| `apercu.png` | l'aperçu des messageries, 1200 × 630 — composé, voir ci-dessous |
| `touch-icon.png` | l'icône d'écran d'accueil iOS, opaque |

Les SVG sortent d'Affinity et passent par `scripts/normaliser-svg.py`, qui est
idempotent : à relancer après chaque nouvel export, sans réfléchir. Il pose la
`viewBox` que l'export omet — sans elle un vecteur ne se met pas à l'échelle,
ce qui lui retire sa raison d'être — retire la taille en pixels pour que la CSS
décide, et remplace la couleur en dur par `currentColor`, plus l'or de la
marque en attribut de présentation : une valeur par défaut qu'une règle CSS
écrase, donc la montagne suit le thème sombre tout en restant dorée quand le
fichier est ouvert seul.

L'or exporté par Affinity vaut `#CFBD7A`, pas `#CDBE83` — un décalage de profil
colorimétrique. C'est le second qui fait foi : il est relevé au pixel sur le
rendu de la marque.

`scripts/images-sociales.py` compose l'aperçu et l'icône **depuis les jetons du
site** plutôt qu'à la main : une image d'aperçu dessinée à part trahit le jour
où la marque bouge. Deux raisons de les avoir faites :

- l'aperçu servait la montagne seule en 512 × 512, un carré dans un cadre
  paysage — la plupart des messageries le rognent ou l'entourent de blanc ;
- iOS ne gère pas la transparence d'une icône d'écran d'accueil : il la remplit
  de noir. Il faut un fond d'aubergine opaque.

Le script rastérise les SVG par QuickLook, qui rend sur **fond blanc opaque**.
La transparence est reconstruite par l'arithmétique — le tracé n'a qu'une
couleur, donc α = (255 − L) / (255 − L_or). Un détourage naïf récupérerait le
carré entier, ce qu'un premier essai a fait.

Le portrait est détouré **depuis le fichier brut** — `~/Downloads/IMG_3655.DNG`,
5348 × 7132 — par `scripts/portrait.py`. Ce fichier n'est pas dans le dépôt :
gardez-le, c'est la seule source. `Doneground/CV/done/photo CV/export/pro.png`
est un ancien détourage sur fond blanc, désormais inutile.

Le script mesure au lieu de supposer. Le mur n'est pas uniforme — 202 de
luminance à hauteur de tête, 142 à hauteur d'épaule — donc il est **modélisé**
ligne par ligne depuis les deux marges. Les seuils viennent de mesures : le
résidu du modèle sur le mur plafonne à 39, l'écart sur le visage commence à
113 ; on prend 42 et 95, et les mèches vivent entre les deux.

Et l'opacité se calcule **en cherchant le fond, pas le sujet**. Seuiller sur
« ressemble au sujet » perce un trou dans la chemise blanche là où un pli a la
valeur du mur. Le mur, lui, est connexe au bord de l'image — ce qu'aucune
partie du sujet n'est. On propage donc depuis les bords, et tout ce qui n'est
pas atteint appartient au sujet.

Le cadrage s'arrête au buste : plus bas, l'ombre creuse sous l'épaule gauche se
lit comme une déchirure sur fond sombre. Elle est dans la photographie, pas
dans le détourage.

Sortie en **WebP à qualité 85** : 152 Ko contre 1 665 en PNG, onze fois moins,
sans différence visible et en gardant l'alpha. Pas de repli PNG — tous les
navigateurs lisent le WebP depuis 2020, et un repli qu'on ne teste jamais est
un fichier mort.

La montagne existe aussi en catalogue d'assets 1×/2×/3× dans
`ONTBibleApp/app/Packages/ONTDesignSystem/Sources/ONTDesignSystem/Resources/`.
Lui **demander un export SVG** : il l'a proposé, c'est le bon original pour un
usage en grand sur le web.

## 7. Les pages, et où trouver la matière

Le vault est le dépôt voisin : `../ONTBibleTranslation/`.

| page | contenu | source |
|---|---|---|
| **Accueil** | le principe en une phrase — *le cosmos hébreu n'est pas une usine, c'est un Temple* — et le verset du jour | `CLAUDE.md` §1 |
| **Le pourquoi** | l'ontologie fonctionnelle ; pourquoi *bara* ne veut pas dire fabriquer ; les trois niveaux montrés sur un vrai verset | `CLAUDE.md` §1, §2.1 |
| **Ce que l'ONT n'est pas** | ses cinq lignes, telles quelles — la page la plus forte, la seule qui prend un risque | `CLAUDE.md` §10 |
| **L'auteur** | l'origine, la vision, le rapport de travail avec Claude — qu'il assume | `context/auteur.md` + sa relecture |
| **Lire** | les passages partagés | `ONTBibleApp/dist/` |
| **Confidentialité / Conditions** | à rédiger — comptes, synchronisation, Sentry, RGPD | — |

**Le verset du jour est fait, et l'accord avec l'app est prouvé.**

C'est une *fonction de la date*, pas un tirage : l'app, le widget et la
notification tombent sur le même verset le même jour sans se parler. Le portage
vit dans `domaine/verset_du_jour.rs`, le vivier est embarqué à la compilation
depuis `../ONTBibleApp/dist/daily.json` (251 versets) — **jamais copié**, sinon
la règle éditoriale du pipeline et celle du site divergeraient.

Deux pièges découverts en le faisant, et tous deux sont maintenant tenus par
des témoins relevés en **exécutant le Swift** :

1. **La permutation.** Un pas fixe premier avec la taille du vivier, valant
   ~0,618 × celle-ci. Il garantit qu'aucun verset ne revient avant que tous
   soient passés — un tirage donnerait un doublon dans le mois avec quatre
   chances sur cinq.

2. **Le décalage d'un jour.** L'app fait `startOfDay` (minuit **local**) puis
   divise cet horodatage comme s'il était UTC. Pour tout fuseau à l'est de
   Greenwich, le numéro de jour vaut donc *la date civile moins un*. Le verset
   change bien à minuit chez le lecteur — seul l'entier est décalé — mais comme
   le choix est une permutation de cet entier, un jour d'écart donne **un autre
   verset**. `infrastructure/horloge.rs` reproduit le calcul de l'app ; le
   corriger ferait diverger le site du téléphone du même lecteur.

   Le premier témoin employait un calendrier en UTC : il passait, et le site
   annonçait quand même un autre verset. C'est le fuseau de l'édition qu'il
   fallait éprouver.

## 7 bis. Voir le site

Deux outils, et ils ne servent pas à la même chose.

`scripts/apercu.py` rend une page avec QuickLook, qui embarque WebKit. Il suffit
à juger une composition sur grand écran.

`scripts/sim.sh` ouvre une page dans le **simulateur iOS** et en capture
l'écran. C'est la seule vérification qui vaille pour tout ce qui dépend de la
largeur : QuickLook rend à une fenêtre fixe puis réduit l'image, donc les
requêtes média y voient toujours un grand écran. Toutes les vérifications
« mobile » faites avec lui étaient sans valeur, et une bande qui réapparaissait
sur téléphone est passée à travers.

    ./scripts/sim.sh /fr/l-auteur

Le simulateur partage le réseau de l'hôte : `127.0.0.1:3000` lui répond. Le
script prend l'appareil nommé « Web » s'il existe — les autres portent l'app
ONT, et y ouvrir Safari les sortirait de leur état.

**Une limite connue** : QuickLook ne rend pas les masques CSS pointant un SVG
externe. Le signe de section et les massifs y apparaissent comme des vides.
Ce n'est pas un défaut de la page — ne pas « corriger » ces blancs.

## 8. Où en est le site

**Fait** — squelette Leptos SSR, couches, design system, cinq pages, verset du
jour accordé à l'app, vecteurs de la marque normalisés, métadonnées complètes
(canonique, hreflang, Open Graph, JSON-LD).

```
/            → 307 vers /fr        (temporaire : « / » choisira la langue un jour)
/fr                                la page d'accueil — voir ci-dessous
/fr/le-pourquoi                    l'essai de fond
/fr/ce-que-l-ont-n-est-pas         les cinq lignes du §10 du vault
/fr/l-auteur                       premier jet, en attente de sa relecture
/fr/confidentialite                vérifiée dans le code de l'app, pas recopiée
/fr/conditions
```

### L'accueil montre, il n'annonce pas

La première version était un sommaire : trois liens vers trois pages maigres.
Un sommaire demande au lecteur de choisir avant de savoir de quoi il s'agit —
et sur un projet dont personne n'a entendu parler, il repart.

L'ordre de la page est un argument, et il ne se réorganise pas au hasard :

1. **L'affirmation** — la montagne en grand, une phrase, deux chemins.
2. **La démonstration** — Bereshit 1:1 chez Louis Segond (1910, domaine
   public), puis la restitution avec ses trois niveaux. C'est la pièce qui
   porte tout le site : l'écart se voit avant d'être compris. Citer une
   traduction moderne serait une contrefaçon ; une traduction ancienne et
   respectée rend d'ailleurs l'écart plus parlant.
3. **La clé de lecture** — les trois niveaux, expliqués après avoir été vus.
4. **Le verset du jour**.
5. **L'état du chantier** — 3 livres sur 70, et les chiffres viennent du
   `manifest.json` du pipeline, figés par `build.rs`. Jamais recopiés : un site
   qui annonce trois livres quand le vault en a cinq ment sans que personne ne
   le remarque.
6. **Qui traduit**, en trois lignes, puis le lien vers la page.
7. **Pour aller plus loin.**

`Hero` ouvre, `Bloc` fait tout le reste — une seule primitive de mise en page,
pleine largeur avec la mesure rétablie à l'intérieur. Deux primitives finissent
toujours par diverger sur un espacement.

**Reste à faire**, dans cet ordre :

1. Les deux routes techniques — l'association et `/fr/lire/{livre}/{unité}` —
   portées depuis `ONTBibleApp/backend/src/interface/web.rs`.
2. La liseuse, si elle est décidée (§9) : elle demande un adaptateur qui lit
   `dist/` — publié comme artefact, jamais dupliqué.
3. Le lexique — `/fr/lexique/{lemme}`, que les intraduisibles pointent déjà.
4. Déploiement, puis bascule des domaines dans l'ordre du §4.

## 9. Ce qui reste à trancher

- Le **texte de la page auteur** — le jet est écrit, il attend sa relecture.
  **Rien de cette page ne doit être mis en ligne avant.**
- **Jost ou EB Garamond** pour le corps du site. Jost est en place, EB Garamond
  est chargée à côté : un seul jeton à changer pour comparer en direct.
- L'**adresse de contact**. `contact@ontbible.com` est écrite dans les pages
  légales ; elle n'existe pas encore. Cloudflare Email Routing la fait suivre
  gratuitement, en deux minutes.
- Une **image d'aperçu** dessinée pour 1200 × 630. Sans elle, une messagerie
  n'affiche qu'une vignette.
- Le site affiche-t-il **le corpus** (vraie liseuse en ligne) ou seulement les
  passages partagés ?
- Le **®** du combination mark. Il est sur le wordmark, donc sur **toutes les
  pages**. En France, apposer ® sur une marque non déposée à l'INPI relève de
  l'article L.716-9 du code de la propriété intellectuelle — ce n'est pas un
  risque commercial, c'est une infraction. Trois sorties : la marque est
  déposée ; elle va l'être (~190 € pour une classe) ; ou on produit une
  variante sans le sigle, à côté des fichiers d'origine.

## 10. Les dépôts voisins

| | |
|---|---|
| `ONTBible/ONTBibleTranslation` | le vault — la traduction. **Public.** 93 commits signés |
| `ONTBible/ONTBibleApp` | pipeline, app iOS, backend. **Public** |
| `ONTBible/ONTBibleWebapp` | ce dépôt |

Les trois portent le même ruleset : `main` protégée, passage par PR
obligatoire, **signatures exigées**, suppression de branche après fusion.
Signer les commits — `commit.gpgsign` et une clé SSH sont déjà configurés
globalement sur cette machine.
